// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The article or artifact that the Citation Resource is related to."]
#[derive(Debug, Clone)]
pub enum CitationRelatesToTarget {
    Uri(Box<super::super::types::Uri>),
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for CitationRelatesToTarget {
    fn default() -> CitationRelatesToTarget {
        CitationRelatesToTarget::Invalid
    }
}
#[doc = "The article or artifact that the cited artifact is related to."]
#[derive(Debug, Clone)]
pub enum CitationCitedArtifactRelatesToTarget {
    Uri(Box<super::super::types::Uri>),
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for CitationCitedArtifactRelatesToTarget {
    fn default() -> CitationCitedArtifactRelatesToTarget {
        CitationCitedArtifactRelatesToTarget::Invalid
    }
}
#[doc = "A human-readable display of the citation."]
#[derive(Default, Debug, Clone)]
pub struct CitationSummary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Format for display of the citation."]
    pub r#style: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human-readable display of the citation."]
    pub r#text: super::super::types::Markdown,
}
impl serde::ser::Serialize for CitationSummary {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#style.as_ref() {
                state.serialize_entry("style", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#text.id.as_ref(),
                        extension: &self.r#text.extension,
                    };
                    state.serialize_entry("_text", &primitive_element)?;
                }
            } else {
                state.serialize_entry("text", &self.r#text)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationSummary {
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
            #[serde(rename = "style")]
            Style,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationSummary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationSummary")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationSummary, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#style: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#text: Option<super::super::types::Markdown> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Style => {
                                if r#style.is_some() {
                                    return Err(serde::de::Error::duplicate_field("style"));
                                }
                                r#style = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &["id", "extension", "modifierExtension", "style", "text"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "style", "text"],
                                ));
                            },
                        }
                    }
                    Ok(CitationSummary {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#style,
                        r#text: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#text.unwrap_or(Default::default())
                        } else {
                            r#text.ok_or(serde::de::Error::missing_field("text"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The assignment to an organizing scheme."]
#[derive(Default, Debug, Clone)]
pub struct CitationClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of classifier (e.g. publication type, keyword)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific classification value."]
    pub r#classifier: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for CitationClassification {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#classifier.is_empty() {
                state.serialize_entry("classifier", &self.r#classifier)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationClassification {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "classifier")]
            Classifier,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationClassification")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationClassification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#classifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Classifier => {
                                if r#classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("classifier"));
                                }
                                r#classifier = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "classifier"],
                                ));
                            },
                        }
                    }
                    Ok(CitationClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#classifier: r#classifier.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An effective date or period for a status of the citation."]
#[derive(Default, Debug, Clone)]
pub struct CitationStatusDate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Classification of the status."]
    pub r#activity: Box<super::super::types::CodeableConcept>,
    #[doc = "Either occurred or expected."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "When the status started and/or ended."]
    pub r#period: Box<super::super::types::Period>,
}
impl serde::ser::Serialize for CitationStatusDate {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("activity", &self.r#activity)?;
            if _ctx.output_json {
                if let Some(some) = self.r#actual.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("actual", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_actual", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#actual.as_ref() {
                    state.serialize_entry("actual", some)?;
                }
            }
            state.serialize_entry("period", &self.r#period)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationStatusDate {
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
            #[serde(rename = "activity")]
            Activity,
            #[serde(rename = "actual")]
            Actual,
            #[serde(rename = "_actual")]
            ActualPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationStatusDate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationStatusDate")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationStatusDate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#activity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actual: Option<super::super::types::Boolean> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Activity => {
                                if r#activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("activity"));
                                }
                                r#activity = Some(map_access.next_value()?);
                            }
                            Field::Actual => {
                                if _ctx.from_json {
                                    let some = r#actual.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("actual"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#actual.is_some() {
                                        return Err(serde::de::Error::duplicate_field("actual"));
                                    }
                                    r#actual = Some(map_access.next_value()?);
                                }
                            }
                            Field::ActualPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#actual.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_actual"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "actual",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "activity",
                                            "actual",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
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
                                        "activity",
                                        "actual",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationStatusDate {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#activity: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#activity.unwrap_or(Default::default())
                        } else {
                            r#activity.ok_or(serde::de::Error::missing_field("activity"))?
                        },
                        r#actual,
                        r#period: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Artifact related to the Citation Resource."]
#[derive(Default, Debug, Clone)]
pub struct CitationRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the Citation resource relates to the target artifact."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The clasification of the related artifact."]
    pub r#target_classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The article or artifact that the Citation Resource is related to."]
    pub r#target: CitationRelatesToTarget,
}
impl serde::ser::Serialize for CitationRelatesTo {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("relationshipType", &self.r#relationship_type)?;
            if !self.r#target_classifier.is_empty() {
                state.serialize_entry("targetClassifier", &self.r#target_classifier)?;
            }
            match self.r#target {
                CitationRelatesToTarget::Uri(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("targetUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_targetUri", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("targetUri", value)?;
                    }
                }
                CitationRelatesToTarget::Identifier(ref value) => {
                    state.serialize_entry("targetIdentifier", value)?;
                }
                CitationRelatesToTarget::Reference(ref value) => {
                    state.serialize_entry("targetReference", value)?;
                }
                CitationRelatesToTarget::Attachment(ref value) => {
                    state.serialize_entry("targetAttachment", value)?;
                }
                CitationRelatesToTarget::Invalid => {
                    return Err(serde::ser::Error::custom("target is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationRelatesTo {
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
            #[serde(rename = "relationshipType")]
            RelationshipType,
            #[serde(rename = "targetClassifier")]
            TargetClassifier,
            #[serde(rename = "targetUri")]
            TargetUri,
            #[serde(rename = "_targetUri")]
            TargetUriPrimitiveElement,
            #[serde(rename = "targetIdentifier")]
            TargetIdentifier,
            #[serde(rename = "targetReference")]
            TargetReference,
            #[serde(rename = "targetAttachment")]
            TargetAttachment,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationRelatesTo")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationRelatesTo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relationship_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#target_classifier: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#target: Option<CitationRelatesToTarget> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::RelationshipType => {
                                if r#relationship_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relationshipType",
                                    ));
                                }
                                r#relationship_type = Some(map_access.next_value()?);
                            }
                            Field::TargetClassifier => {
                                if r#target_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetClassifier",
                                    ));
                                }
                                r#target_classifier = Some(map_access.next_value()?);
                            }
                            Field::TargetUri => {
                                if _ctx.from_json {
                                    let r#enum = r#target.get_or_insert(
                                        CitationRelatesToTarget::Uri(Default::default()),
                                    );
                                    if let CitationRelatesToTarget::Uri(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "targetUri",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("target[x]"));
                                    }
                                } else {
                                    if r#target.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetUri"));
                                    }
                                    r#target = Some(CitationRelatesToTarget::Uri(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::TargetUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#target.get_or_insert(
                                        CitationRelatesToTarget::Uri(Default::default()),
                                    );
                                    if let CitationRelatesToTarget::Uri(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_targetUri",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_target[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "targetUri",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "relationshipType",
                                            "targetClassifier",
                                            "targetUri",
                                            "targetIdentifier",
                                            "targetReference",
                                            "targetAttachment",
                                        ],
                                    ));
                                }
                            }
                            Field::TargetIdentifier => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetIdentifier",
                                    ));
                                }
                                r#target = Some(CitationRelatesToTarget::Identifier(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TargetReference => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetReference",
                                    ));
                                }
                                r#target = Some(CitationRelatesToTarget::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TargetAttachment => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetAttachment",
                                    ));
                                }
                                r#target = Some(CitationRelatesToTarget::Attachment(
                                    map_access.next_value()?,
                                ));
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
                                        "relationshipType",
                                        "targetClassifier",
                                        "targetUri",
                                        "targetIdentifier",
                                        "targetReference",
                                        "targetAttachment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationRelatesTo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationship_type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#relationship_type.unwrap_or(Default::default())
                        } else {
                            r#relationship_type
                                .ok_or(serde::de::Error::missing_field("relationshipType"))?
                        },
                        r#target_classifier: r#target_classifier.unwrap_or(vec![]),
                        r#target: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#target.unwrap_or(Default::default())
                        } else {
                            r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The defined version of the cited artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The version number or other version identifier."]
    pub r#value: super::super::types::String,
    #[doc = "Citation for the main version of the cited artifact."]
    pub r#base_citation: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CitationCitedArtifactVersion {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            if let Some(some) = self.r#base_citation.as_ref() {
                state.serialize_entry("baseCitation", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactVersion {
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
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            #[serde(rename = "baseCitation")]
            BaseCitation,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactVersion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactVersion")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactVersion, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<super::super::types::String> = None;
                let mut r#base_citation: Option<Box<super::super::types::Reference>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "value",
                                            "baseCitation",
                                        ],
                                    ));
                                }
                            }
                            Field::BaseCitation => {
                                if r#base_citation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("baseCitation"));
                                }
                                r#base_citation = Some(map_access.next_value()?);
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
                                        "value",
                                        "baseCitation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactVersion {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                        r#base_citation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An effective date or period for a status of the cited artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactStatusDate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Classification of the status."]
    pub r#activity: Box<super::super::types::CodeableConcept>,
    #[doc = "Either occurred or expected."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "When the status started and/or ended."]
    pub r#period: Box<super::super::types::Period>,
}
impl serde::ser::Serialize for CitationCitedArtifactStatusDate {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("activity", &self.r#activity)?;
            if _ctx.output_json {
                if let Some(some) = self.r#actual.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("actual", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_actual", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#actual.as_ref() {
                    state.serialize_entry("actual", some)?;
                }
            }
            state.serialize_entry("period", &self.r#period)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactStatusDate {
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
            #[serde(rename = "activity")]
            Activity,
            #[serde(rename = "actual")]
            Actual,
            #[serde(rename = "_actual")]
            ActualPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactStatusDate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactStatusDate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactStatusDate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#activity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actual: Option<super::super::types::Boolean> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Activity => {
                                if r#activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("activity"));
                                }
                                r#activity = Some(map_access.next_value()?);
                            }
                            Field::Actual => {
                                if _ctx.from_json {
                                    let some = r#actual.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("actual"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#actual.is_some() {
                                        return Err(serde::de::Error::duplicate_field("actual"));
                                    }
                                    r#actual = Some(map_access.next_value()?);
                                }
                            }
                            Field::ActualPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#actual.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_actual"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "actual",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "activity",
                                            "actual",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
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
                                        "activity",
                                        "actual",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactStatusDate {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#activity: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#activity.unwrap_or(Default::default())
                        } else {
                            r#activity.ok_or(serde::de::Error::missing_field("activity"))?
                        },
                        r#actual,
                        r#period: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The title details of the article or artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactTitle {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Used to express the reason or specific aspect for the title."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to express the specific language."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The title of the article or artifact."]
    pub r#text: super::super::types::Markdown,
}
impl serde::ser::Serialize for CitationCitedArtifactTitle {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if let Some(some) = self.r#language.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#text.id.as_ref(),
                        extension: &self.r#text.extension,
                    };
                    state.serialize_entry("_text", &primitive_element)?;
                }
            } else {
                state.serialize_entry("text", &self.r#text)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactTitle {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactTitle;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactTitle")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationCitedArtifactTitle, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#text: Option<super::super::types::Markdown> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Language => {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "language",
                                            "text",
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
                                        "modifierExtension",
                                        "type",
                                        "language",
                                        "text",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactTitle {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: r#type.unwrap_or(vec![]),
                        r#language,
                        r#text: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#text.unwrap_or(Default::default())
                        } else {
                            r#text.ok_or(serde::de::Error::missing_field("text"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Summary of the article or artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactAbstract {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Used to express the reason or specific aspect for the abstract."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to express the specific language."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Abstract content."]
    pub r#text: super::super::types::Markdown,
    #[doc = "Copyright notice for the abstract."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CitationCitedArtifactAbstract {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#language.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#text.id.as_ref(),
                        extension: &self.r#text.extension,
                    };
                    state.serialize_entry("_text", &primitive_element)?;
                }
            } else {
                state.serialize_entry("text", &self.r#text)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactAbstract {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactAbstract;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactAbstract")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactAbstract, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#text: Option<super::super::types::Markdown> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Language => {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "language",
                                            "text",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
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
                                        "copyright",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "language",
                                            "text",
                                            "copyright",
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
                                        "modifierExtension",
                                        "type",
                                        "language",
                                        "text",
                                        "copyright",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactAbstract {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#language,
                        r#text: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#text.unwrap_or(Default::default())
                        } else {
                            r#text.ok_or(serde::de::Error::missing_field("text"))?
                        },
                        r#copyright,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The component of the article or artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactPart {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of component."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specification of the component."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "The citation for the full article or artifact."]
    pub r#base_citation: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CitationCitedArtifactPart {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("value", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_value", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#value.as_ref() {
                    state.serialize_entry("value", some)?;
                }
            }
            if let Some(some) = self.r#base_citation.as_ref() {
                state.serialize_entry("baseCitation", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactPart {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            #[serde(rename = "baseCitation")]
            BaseCitation,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactPart;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPart")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationCitedArtifactPart, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<super::super::types::String> = None;
                let mut r#base_citation: Option<Box<super::super::types::Reference>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "value",
                                            "baseCitation",
                                        ],
                                    ));
                                }
                            }
                            Field::BaseCitation => {
                                if r#base_citation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("baseCitation"));
                                }
                                r#base_citation = Some(map_access.next_value()?);
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
                                        "type",
                                        "value",
                                        "baseCitation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactPart {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#value,
                        r#base_citation,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The artifact related to the cited artifact."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the cited artifact relates to the target artifact."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The clasification of the related artifact."]
    pub r#target_classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The article or artifact that the cited artifact is related to."]
    pub r#target: CitationCitedArtifactRelatesToTarget,
}
impl serde::ser::Serialize for CitationCitedArtifactRelatesTo {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("relationshipType", &self.r#relationship_type)?;
            if !self.r#target_classifier.is_empty() {
                state.serialize_entry("targetClassifier", &self.r#target_classifier)?;
            }
            match self.r#target {
                CitationCitedArtifactRelatesToTarget::Uri(ref value) => {
                    if _ctx.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("targetUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            state.serialize_entry("_targetUri", &primitive_element)?;
                        }
                    } else {
                        state.serialize_entry("targetUri", value)?;
                    }
                }
                CitationCitedArtifactRelatesToTarget::Identifier(ref value) => {
                    state.serialize_entry("targetIdentifier", value)?;
                }
                CitationCitedArtifactRelatesToTarget::Reference(ref value) => {
                    state.serialize_entry("targetReference", value)?;
                }
                CitationCitedArtifactRelatesToTarget::Attachment(ref value) => {
                    state.serialize_entry("targetAttachment", value)?;
                }
                CitationCitedArtifactRelatesToTarget::Invalid => {
                    return Err(serde::ser::Error::custom("target is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactRelatesTo {
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
            #[serde(rename = "relationshipType")]
            RelationshipType,
            #[serde(rename = "targetClassifier")]
            TargetClassifier,
            #[serde(rename = "targetUri")]
            TargetUri,
            #[serde(rename = "_targetUri")]
            TargetUriPrimitiveElement,
            #[serde(rename = "targetIdentifier")]
            TargetIdentifier,
            #[serde(rename = "targetReference")]
            TargetReference,
            #[serde(rename = "targetAttachment")]
            TargetAttachment,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactRelatesTo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactRelatesTo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relationship_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#target_classifier: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#target: Option<CitationCitedArtifactRelatesToTarget> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::RelationshipType => {
                                if r#relationship_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relationshipType",
                                    ));
                                }
                                r#relationship_type = Some(map_access.next_value()?);
                            }
                            Field::TargetClassifier => {
                                if r#target_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetClassifier",
                                    ));
                                }
                                r#target_classifier = Some(map_access.next_value()?);
                            }
                            Field::TargetUri => {
                                if _ctx.from_json {
                                    let r#enum = r#target.get_or_insert(
                                        CitationCitedArtifactRelatesToTarget::Uri(
                                            Default::default(),
                                        ),
                                    );
                                    if let CitationCitedArtifactRelatesToTarget::Uri(variant) =
                                        r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "targetUri",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("target[x]"));
                                    }
                                } else {
                                    if r#target.is_some() {
                                        return Err(serde::de::Error::duplicate_field("targetUri"));
                                    }
                                    r#target = Some(CitationCitedArtifactRelatesToTarget::Uri(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::TargetUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#target.get_or_insert(
                                        CitationCitedArtifactRelatesToTarget::Uri(
                                            Default::default(),
                                        ),
                                    );
                                    if let CitationCitedArtifactRelatesToTarget::Uri(variant) =
                                        r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_targetUri",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_target[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "targetUri",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "relationshipType",
                                            "targetClassifier",
                                            "targetUri",
                                            "targetIdentifier",
                                            "targetReference",
                                            "targetAttachment",
                                        ],
                                    ));
                                }
                            }
                            Field::TargetIdentifier => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetIdentifier",
                                    ));
                                }
                                r#target = Some(CitationCitedArtifactRelatesToTarget::Identifier(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TargetReference => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetReference",
                                    ));
                                }
                                r#target = Some(CitationCitedArtifactRelatesToTarget::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TargetAttachment => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetAttachment",
                                    ));
                                }
                                r#target = Some(CitationCitedArtifactRelatesToTarget::Attachment(
                                    map_access.next_value()?,
                                ));
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
                                        "relationshipType",
                                        "targetClassifier",
                                        "targetUri",
                                        "targetIdentifier",
                                        "targetReference",
                                        "targetAttachment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactRelatesTo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationship_type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#relationship_type.unwrap_or(Default::default())
                        } else {
                            r#relationship_type
                                .ok_or(serde::de::Error::missing_field("relationshipType"))?
                        },
                        r#target_classifier: r#target_classifier.unwrap_or(vec![]),
                        r#target: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#target.unwrap_or(Default::default())
                        } else {
                            r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The collection the cited article or artifact is published in."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactPublicationFormPublishedIn {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Kind of container (e.g. Periodical, database, or book)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID; Book identifiers include ISBN."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Name of the database or title of the book or journal."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Name of the publisher."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "Geographic location of the publisher."]
    pub r#publisher_location: Option<super::super::types::String>,
}
impl serde::ser::Serialize for CitationCitedArtifactPublicationFormPublishedIn {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if let Some(some) = self.r#publisher.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publisher_location.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisherLocation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisherLocation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher_location.as_ref() {
                    state.serialize_entry("publisherLocation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactPublicationFormPublishedIn {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "publisherLocation")]
            PublisherLocation,
            #[serde(rename = "_publisherLocation")]
            PublisherLocationPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactPublicationFormPublishedIn;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationFormPublishedIn")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactPublicationFormPublishedIn, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#publisher: Option<Box<super::super::types::Reference>> = None;
                let mut r#publisher_location: Option<super::super::types::String> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_title"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "identifier",
                                            "title",
                                            "publisher",
                                            "publisherLocation",
                                        ],
                                    ));
                                }
                            }
                            Field::Publisher => {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                r#publisher = Some(map_access.next_value()?);
                            }
                            Field::PublisherLocation => {
                                if _ctx.from_json {
                                    let some =
                                        r#publisher_location.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publisherLocation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publisher_location.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "publisherLocation",
                                        ));
                                    }
                                    r#publisher_location = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublisherLocationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#publisher_location.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publisherLocation",
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
                                        "publisherLocation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "identifier",
                                            "title",
                                            "publisher",
                                            "publisherLocation",
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
                                        "modifierExtension",
                                        "type",
                                        "identifier",
                                        "title",
                                        "publisher",
                                        "publisherLocation",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactPublicationFormPublishedIn {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#title,
                        r#publisher,
                        r#publisher_location,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Defining the date on which the issue of the journal was published."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date on which the issue of the journal was published."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Year on which the issue of the journal was published."]
    pub r#year: Option<super::super::types::String>,
    #[doc = "Month on which the issue of the journal was published."]
    pub r#month: Option<super::super::types::String>,
    #[doc = "Day on which the issue of the journal was published."]
    pub r#day: Option<super::super::types::String>,
    #[doc = "Spring, Summer, Fall/Autumn, Winter."]
    pub r#season: Option<super::super::types::String>,
    #[doc = "Text representation of the date of which the issue of the journal was published."]
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize
    for CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication
{
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#year.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("year", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_year", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#year.as_ref() {
                    state.serialize_entry("year", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#month.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("month", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_month", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#month.as_ref() {
                    state.serialize_entry("month", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#day.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("day", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_day", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#day.as_ref() {
                    state.serialize_entry("day", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#season.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("season", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_season", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#season.as_ref() {
                    state.serialize_entry("season", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de>
    for CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication
{
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
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "year")]
            Year,
            #[serde(rename = "_year")]
            YearPrimitiveElement,
            #[serde(rename = "month")]
            Month,
            #[serde(rename = "_month")]
            MonthPrimitiveElement,
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "_day")]
            DayPrimitiveElement,
            #[serde(rename = "season")]
            Season,
            #[serde(rename = "_season")]
            SeasonPrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication",
                )
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#year: Option<super::super::types::String> = None;
                let mut r#month: Option<super::super::types::String> = None;
                let mut r#day: Option<super::super::types::String> = None;
                let mut r#season: Option<super::super::types::String> = None;
                let mut r#text: Option<super::super::types::String> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_date"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                            Field::Year => {
                                if _ctx.from_json {
                                    let some = r#year.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("year"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#year.is_some() {
                                        return Err(serde::de::Error::duplicate_field("year"));
                                    }
                                    r#year = Some(map_access.next_value()?);
                                }
                            }
                            Field::YearPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#year.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_year"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "year",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                            Field::Month => {
                                if _ctx.from_json {
                                    let some = r#month.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("month"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#month.is_some() {
                                        return Err(serde::de::Error::duplicate_field("month"));
                                    }
                                    r#month = Some(map_access.next_value()?);
                                }
                            }
                            Field::MonthPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#month.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_month"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "month",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                            Field::Day => {
                                if _ctx.from_json {
                                    let some = r#day.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("day"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#day.is_some() {
                                        return Err(serde::de::Error::duplicate_field("day"));
                                    }
                                    r#day = Some(map_access.next_value()?);
                                }
                            }
                            Field::DayPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#day.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_day"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "day",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                            Field::Season => {
                                if _ctx.from_json {
                                    let some = r#season.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("season"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#season.is_some() {
                                        return Err(serde::de::Error::duplicate_field("season"));
                                    }
                                    r#season = Some(map_access.next_value()?);
                                }
                            }
                            Field::SeasonPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#season.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_season"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "season",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_text"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "year",
                                            "month",
                                            "day",
                                            "season",
                                            "text",
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
                                        "modifierExtension",
                                        "date",
                                        "year",
                                        "month",
                                        "day",
                                        "season",
                                        "text",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(
                        CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication {
                            r#id,
                            r#extension: r#extension.unwrap_or(vec![]),
                            r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                            r#date,
                            r#year,
                            r#month,
                            r#day,
                            r#season,
                            r#text,
                        },
                    )
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The specific issue in which the cited article resides."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactPublicationFormPeriodicRelease {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes the form of the medium cited. Common codes are \"Internet\" or \"Print\"."]
    pub r#cited_medium: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Volume number of journal in which the article is published."]
    pub r#volume: Option<super::super::types::String>,
    #[doc = "Issue, part or supplement of journal in which the article is published."]
    pub r#issue: Option<super::super::types::String>,
    #[doc = "Defining the date on which the issue of the journal was published."]
    pub r#date_of_publication:
        Option<CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication>,
}
impl serde::ser::Serialize for CitationCitedArtifactPublicationFormPeriodicRelease {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#cited_medium.as_ref() {
                state.serialize_entry("citedMedium", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#volume.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("volume", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_volume", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#volume.as_ref() {
                    state.serialize_entry("volume", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issue.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issue", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issue", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issue.as_ref() {
                    state.serialize_entry("issue", some)?;
                }
            }
            if let Some(some) = self.r#date_of_publication.as_ref() {
                state.serialize_entry("dateOfPublication", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactPublicationFormPeriodicRelease {
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
            #[serde(rename = "citedMedium")]
            CitedMedium,
            #[serde(rename = "volume")]
            Volume,
            #[serde(rename = "_volume")]
            VolumePrimitiveElement,
            #[serde(rename = "issue")]
            Issue,
            #[serde(rename = "_issue")]
            IssuePrimitiveElement,
            #[serde(rename = "dateOfPublication")]
            DateOfPublication,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactPublicationFormPeriodicRelease;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationFormPeriodicRelease")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactPublicationFormPeriodicRelease, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#cited_medium: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#volume: Option<super::super::types::String> = None;
                let mut r#issue: Option<super::super::types::String> = None;
                let mut r#date_of_publication: Option<
                    CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication,
                > = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::CitedMedium => {
                                if r#cited_medium.is_some() {
                                    return Err(serde::de::Error::duplicate_field("citedMedium"));
                                }
                                r#cited_medium = Some(map_access.next_value()?);
                            }
                            Field::Volume => {
                                if _ctx.from_json {
                                    let some = r#volume.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("volume"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#volume.is_some() {
                                        return Err(serde::de::Error::duplicate_field("volume"));
                                    }
                                    r#volume = Some(map_access.next_value()?);
                                }
                            }
                            Field::VolumePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#volume.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_volume"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "volume",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "citedMedium",
                                            "volume",
                                            "issue",
                                            "dateOfPublication",
                                        ],
                                    ));
                                }
                            }
                            Field::Issue => {
                                if _ctx.from_json {
                                    let some = r#issue.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issue"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#issue.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issue"));
                                    }
                                    r#issue = Some(map_access.next_value()?);
                                }
                            }
                            Field::IssuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#issue.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_issue"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "issue",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "citedMedium",
                                            "volume",
                                            "issue",
                                            "dateOfPublication",
                                        ],
                                    ));
                                }
                            }
                            Field::DateOfPublication => {
                                if r#date_of_publication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dateOfPublication",
                                    ));
                                }
                                r#date_of_publication = Some(map_access.next_value()?);
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
                                        "citedMedium",
                                        "volume",
                                        "issue",
                                        "dateOfPublication",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactPublicationFormPeriodicRelease {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#cited_medium,
                        r#volume,
                        r#issue,
                        r#date_of_publication,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "If multiple, used to represent alternative forms of the article that are not separate citations."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactPublicationForm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The collection the cited article or artifact is published in."]
    pub r#published_in: Option<CitationCitedArtifactPublicationFormPublishedIn>,
    #[doc = "The specific issue in which the cited article resides."]
    pub r#periodic_release: Option<CitationCitedArtifactPublicationFormPeriodicRelease>,
    #[doc = "The date the article was added to the database, or the date the article was released (which may differ from the journal issue publication date)."]
    pub r#article_date: Option<super::super::types::DateTime>,
    #[doc = "The date the article was last revised or updated in the database."]
    pub r#last_revision_date: Option<super::super::types::DateTime>,
    #[doc = "Language in which this form of the article is published."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Entry number or identifier for inclusion in a database."]
    pub r#accession_number: Option<super::super::types::String>,
    #[doc = "Used for full display of pagination."]
    pub r#page_string: Option<super::super::types::String>,
    #[doc = "Used for isolated representation of first page."]
    pub r#first_page: Option<super::super::types::String>,
    #[doc = "Used for isolated representation of last page."]
    pub r#last_page: Option<super::super::types::String>,
    #[doc = "Actual or approximate number of pages or screens."]
    pub r#page_count: Option<super::super::types::String>,
    #[doc = "Copyright notice for the full article or artifact."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for CitationCitedArtifactPublicationForm {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#published_in.as_ref() {
                state.serialize_entry("publishedIn", some)?;
            }
            if let Some(some) = self.r#periodic_release.as_ref() {
                state.serialize_entry("periodicRelease", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#article_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("articleDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_articleDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#article_date.as_ref() {
                    state.serialize_entry("articleDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_revision_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastRevisionDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastRevisionDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_revision_date.as_ref() {
                    state.serialize_entry("lastRevisionDate", some)?;
                }
            }
            if !self.r#language.is_empty() {
                state.serialize_entry("language", &self.r#language)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#accession_number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("accessionNumber", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_accessionNumber", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#accession_number.as_ref() {
                    state.serialize_entry("accessionNumber", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#page_string.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("pageString", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_pageString", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#page_string.as_ref() {
                    state.serialize_entry("pageString", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#first_page.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("firstPage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_firstPage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#first_page.as_ref() {
                    state.serialize_entry("firstPage", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_page.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastPage", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastPage", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_page.as_ref() {
                    state.serialize_entry("lastPage", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#page_count.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("pageCount", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_pageCount", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#page_count.as_ref() {
                    state.serialize_entry("pageCount", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactPublicationForm {
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
            #[serde(rename = "publishedIn")]
            PublishedIn,
            #[serde(rename = "periodicRelease")]
            PeriodicRelease,
            #[serde(rename = "articleDate")]
            ArticleDate,
            #[serde(rename = "_articleDate")]
            ArticleDatePrimitiveElement,
            #[serde(rename = "lastRevisionDate")]
            LastRevisionDate,
            #[serde(rename = "_lastRevisionDate")]
            LastRevisionDatePrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "accessionNumber")]
            AccessionNumber,
            #[serde(rename = "_accessionNumber")]
            AccessionNumberPrimitiveElement,
            #[serde(rename = "pageString")]
            PageString,
            #[serde(rename = "_pageString")]
            PageStringPrimitiveElement,
            #[serde(rename = "firstPage")]
            FirstPage,
            #[serde(rename = "_firstPage")]
            FirstPagePrimitiveElement,
            #[serde(rename = "lastPage")]
            LastPage,
            #[serde(rename = "_lastPage")]
            LastPagePrimitiveElement,
            #[serde(rename = "pageCount")]
            PageCount,
            #[serde(rename = "_pageCount")]
            PageCountPrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactPublicationForm;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationForm")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactPublicationForm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#published_in: Option<CitationCitedArtifactPublicationFormPublishedIn> =
                    None;
                let mut r#periodic_release: Option<
                    CitationCitedArtifactPublicationFormPeriodicRelease,
                > = None;
                let mut r#article_date: Option<super::super::types::DateTime> = None;
                let mut r#last_revision_date: Option<super::super::types::DateTime> = None;
                let mut r#language: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#accession_number: Option<super::super::types::String> = None;
                let mut r#page_string: Option<super::super::types::String> = None;
                let mut r#first_page: Option<super::super::types::String> = None;
                let mut r#last_page: Option<super::super::types::String> = None;
                let mut r#page_count: Option<super::super::types::String> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::PublishedIn => {
                                if r#published_in.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publishedIn"));
                                }
                                r#published_in = Some(map_access.next_value()?);
                            }
                            Field::PeriodicRelease => {
                                if r#periodic_release.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "periodicRelease",
                                    ));
                                }
                                r#periodic_release = Some(map_access.next_value()?);
                            }
                            Field::ArticleDate => {
                                if _ctx.from_json {
                                    let some = r#article_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "articleDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#article_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "articleDate",
                                        ));
                                    }
                                    r#article_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ArticleDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#article_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_articleDate",
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
                                        "articleDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::LastRevisionDate => {
                                if _ctx.from_json {
                                    let some =
                                        r#last_revision_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastRevisionDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_revision_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastRevisionDate",
                                        ));
                                    }
                                    r#last_revision_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastRevisionDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#last_revision_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lastRevisionDate",
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
                                        "lastRevisionDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value()?);
                            }
                            Field::AccessionNumber => {
                                if _ctx.from_json {
                                    let some = r#accession_number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "accessionNumber",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#accession_number.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "accessionNumber",
                                        ));
                                    }
                                    r#accession_number = Some(map_access.next_value()?);
                                }
                            }
                            Field::AccessionNumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#accession_number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_accessionNumber",
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
                                        "accessionNumber",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::PageString => {
                                if _ctx.from_json {
                                    let some = r#page_string.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "pageString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#page_string.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "pageString",
                                        ));
                                    }
                                    r#page_string = Some(map_access.next_value()?);
                                }
                            }
                            Field::PageStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#page_string.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_pageString",
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
                                        "pageString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::FirstPage => {
                                if _ctx.from_json {
                                    let some = r#first_page.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("firstPage"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#first_page.is_some() {
                                        return Err(serde::de::Error::duplicate_field("firstPage"));
                                    }
                                    r#first_page = Some(map_access.next_value()?);
                                }
                            }
                            Field::FirstPagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#first_page.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_firstPage",
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
                                        "firstPage",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::LastPage => {
                                if _ctx.from_json {
                                    let some = r#last_page.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("lastPage"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_page.is_some() {
                                        return Err(serde::de::Error::duplicate_field("lastPage"));
                                    }
                                    r#last_page = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastPagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#last_page.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_lastPage"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "lastPage",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::PageCount => {
                                if _ctx.from_json {
                                    let some = r#page_count.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("pageCount"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#page_count.is_some() {
                                        return Err(serde::de::Error::duplicate_field("pageCount"));
                                    }
                                    r#page_count = Some(map_access.next_value()?);
                                }
                            }
                            Field::PageCountPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#page_count.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_pageCount",
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
                                        "pageCount",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
                                        ],
                                    ));
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
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
                                        "copyright",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "publishedIn",
                                            "periodicRelease",
                                            "articleDate",
                                            "lastRevisionDate",
                                            "language",
                                            "accessionNumber",
                                            "pageString",
                                            "firstPage",
                                            "lastPage",
                                            "pageCount",
                                            "copyright",
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
                                        "modifierExtension",
                                        "publishedIn",
                                        "periodicRelease",
                                        "articleDate",
                                        "lastRevisionDate",
                                        "language",
                                        "accessionNumber",
                                        "pageString",
                                        "firstPage",
                                        "lastPage",
                                        "pageCount",
                                        "copyright",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactPublicationForm {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#published_in,
                        r#periodic_release,
                        r#article_date,
                        r#last_revision_date,
                        r#language: r#language.unwrap_or(vec![]),
                        r#accession_number,
                        r#page_string,
                        r#first_page,
                        r#last_page,
                        r#page_count,
                        r#copyright,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Used for any URL for the article or artifact cited."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactWebLocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Code the reason for different URLs, e.g. abstract and full-text."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific URL."]
    pub r#url: Option<super::super::types::Uri>,
}
impl serde::ser::Serialize for CitationCitedArtifactWebLocation {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactWebLocation {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactWebLocation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactWebLocation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactWebLocation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_url"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
                                        &["id", "extension", "modifierExtension", "type", "url"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "url"],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactWebLocation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#url,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Provenance and copyright of classification."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactClassificationWhoClassified {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Person who created the classification."]
    pub r#person: Option<Box<super::super::types::Reference>>,
    #[doc = "Organization who created the classification."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The publisher of the classification, not the publisher of the article or artifact being cited."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "Rights management statement for the classification."]
    pub r#classifier_copyright: Option<super::super::types::String>,
    #[doc = "Acceptable to re-use the classification."]
    pub r#free_to_share: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for CitationCitedArtifactClassificationWhoClassified {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#person.as_ref() {
                state.serialize_entry("person", some)?;
            }
            if let Some(some) = self.r#organization.as_ref() {
                state.serialize_entry("organization", some)?;
            }
            if let Some(some) = self.r#publisher.as_ref() {
                state.serialize_entry("publisher", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#classifier_copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("classifierCopyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_classifierCopyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#classifier_copyright.as_ref() {
                    state.serialize_entry("classifierCopyright", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#free_to_share.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("freeToShare", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_freeToShare", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#free_to_share.as_ref() {
                    state.serialize_entry("freeToShare", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactClassificationWhoClassified {
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
            #[serde(rename = "person")]
            Person,
            #[serde(rename = "organization")]
            Organization,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "classifierCopyright")]
            ClassifierCopyright,
            #[serde(rename = "_classifierCopyright")]
            ClassifierCopyrightPrimitiveElement,
            #[serde(rename = "freeToShare")]
            FreeToShare,
            #[serde(rename = "_freeToShare")]
            FreeToSharePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactClassificationWhoClassified;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactClassificationWhoClassified")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactClassificationWhoClassified, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#person: Option<Box<super::super::types::Reference>> = None;
                let mut r#organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#publisher: Option<Box<super::super::types::Reference>> = None;
                let mut r#classifier_copyright: Option<super::super::types::String> = None;
                let mut r#free_to_share: Option<super::super::types::Boolean> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Person => {
                                if r#person.is_some() {
                                    return Err(serde::de::Error::duplicate_field("person"));
                                }
                                r#person = Some(map_access.next_value()?);
                            }
                            Field::Organization => {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                r#organization = Some(map_access.next_value()?);
                            }
                            Field::Publisher => {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                r#publisher = Some(map_access.next_value()?);
                            }
                            Field::ClassifierCopyright => {
                                if _ctx.from_json {
                                    let some =
                                        r#classifier_copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "classifierCopyright",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#classifier_copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "classifierCopyright",
                                        ));
                                    }
                                    r#classifier_copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::ClassifierCopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#classifier_copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_classifierCopyright",
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
                                        "classifierCopyright",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "person",
                                            "organization",
                                            "publisher",
                                            "classifierCopyright",
                                            "freeToShare",
                                        ],
                                    ));
                                }
                            }
                            Field::FreeToShare => {
                                if _ctx.from_json {
                                    let some = r#free_to_share.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "freeToShare",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#free_to_share.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "freeToShare",
                                        ));
                                    }
                                    r#free_to_share = Some(map_access.next_value()?);
                                }
                            }
                            Field::FreeToSharePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#free_to_share.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_freeToShare",
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
                                        "freeToShare",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "person",
                                            "organization",
                                            "publisher",
                                            "classifierCopyright",
                                            "freeToShare",
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
                                        "modifierExtension",
                                        "person",
                                        "organization",
                                        "publisher",
                                        "classifierCopyright",
                                        "freeToShare",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactClassificationWhoClassified {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#person,
                        r#organization,
                        r#publisher,
                        r#classifier_copyright,
                        r#free_to_share,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The assignment to an organizing scheme."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of classifier (e.g. publication type, keyword)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific classification value."]
    pub r#classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provenance and copyright of classification."]
    pub r#who_classified: Option<CitationCitedArtifactClassificationWhoClassified>,
}
impl serde::ser::Serialize for CitationCitedArtifactClassification {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#classifier.is_empty() {
                state.serialize_entry("classifier", &self.r#classifier)?;
            }
            if let Some(some) = self.r#who_classified.as_ref() {
                state.serialize_entry("whoClassified", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactClassification {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "classifier")]
            Classifier,
            #[serde(rename = "whoClassified")]
            WhoClassified,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactClassification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#classifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#who_classified: Option<CitationCitedArtifactClassificationWhoClassified> =
                    None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Classifier => {
                                if r#classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("classifier"));
                                }
                                r#classifier = Some(map_access.next_value()?);
                            }
                            Field::WhoClassified => {
                                if r#who_classified.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whoClassified"));
                                }
                                r#who_classified = Some(map_access.next_value()?);
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
                                        "type",
                                        "classifier",
                                        "whoClassified",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#classifier: r#classifier.unwrap_or(vec![]),
                        r#who_classified,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Organization affiliated with the entity."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactContributorshipEntryAffiliationInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Display for the organization."]
    pub r#affiliation: Option<super::super::types::String>,
    #[doc = "Role within the organization, such as professional title."]
    pub r#role: Option<super::super::types::String>,
    #[doc = "Identifier for the organization."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for CitationCitedArtifactContributorshipEntryAffiliationInfo {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#affiliation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("affiliation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_affiliation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#affiliation.as_ref() {
                    state.serialize_entry("affiliation", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#role.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("role", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_role", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#role.as_ref() {
                    state.serialize_entry("role", some)?;
                }
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactContributorshipEntryAffiliationInfo {
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
            #[serde(rename = "affiliation")]
            Affiliation,
            #[serde(rename = "_affiliation")]
            AffiliationPrimitiveElement,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "_role")]
            RolePrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactContributorshipEntryAffiliationInfo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipEntryAffiliationInfo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactContributorshipEntryAffiliationInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#affiliation: Option<super::super::types::String> = None;
                let mut r#role: Option<super::super::types::String> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Affiliation => {
                                if _ctx.from_json {
                                    let some = r#affiliation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "affiliation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#affiliation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "affiliation",
                                        ));
                                    }
                                    r#affiliation = Some(map_access.next_value()?);
                                }
                            }
                            Field::AffiliationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#affiliation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_affiliation",
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
                                        "affiliation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "affiliation",
                                            "role",
                                            "identifier",
                                        ],
                                    ));
                                }
                            }
                            Field::Role => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#role.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    r#role = Some(map_access.next_value()?);
                                }
                            }
                            Field::RolePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_role"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "role",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "affiliation",
                                            "role",
                                            "identifier",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
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
                                        "affiliation",
                                        "role",
                                        "identifier",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactContributorshipEntryAffiliationInfo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#affiliation,
                        r#role,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Contributions with accounting for time or number."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactContributorshipEntryContributionInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific contribution."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The time that the contribution was made."]
    pub r#time: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for CitationCitedArtifactContributorshipEntryContributionInstance {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if _ctx.output_json {
                if let Some(some) = self.r#time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("time", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_time", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time.as_ref() {
                    state.serialize_entry("time", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de>
    for CitationCitedArtifactContributorshipEntryContributionInstance
{
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "time")]
            Time,
            #[serde(rename = "_time")]
            TimePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactContributorshipEntryContributionInstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipEntryContributionInstance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactContributorshipEntryContributionInstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#time: Option<super::super::types::DateTime> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Time => {
                                if _ctx.from_json {
                                    let some = r#time.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("time"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#time.is_some() {
                                        return Err(serde::de::Error::duplicate_field("time"));
                                    }
                                    r#time = Some(map_access.next_value()?);
                                }
                            }
                            Field::TimePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#time.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_time"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "time",
                                        &["id", "extension", "modifierExtension", "type", "time"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "time"],
                                ));
                            },
                        }
                    }
                    Ok(
                        CitationCitedArtifactContributorshipEntryContributionInstance {
                            r#id,
                            r#extension: r#extension.unwrap_or(vec![]),
                            r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                            r#type: if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                            {
                                r#type.unwrap_or(Default::default())
                            } else {
                                r#type.ok_or(serde::de::Error::missing_field("type"))?
                            },
                            r#time,
                        },
                    )
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An individual entity named in the author list or contributor list."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactContributorshipEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A name associated with the individual."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "Initials for forename."]
    pub r#initials: Option<super::super::types::String>,
    #[doc = "Used for collective or corporate name as an author."]
    pub r#collective_name: Option<super::super::types::String>,
    #[doc = "Unique person identifier."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Organization affiliated with the entity."]
    pub r#affiliation_info: Vec<CitationCitedArtifactContributorshipEntryAffiliationInfo>,
    #[doc = "Physical mailing address for the author or contributor."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "Email or telephone contact methods for the author or contributor."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "This element identifies the specific nature of an individual’s contribution with respect to the cited work."]
    pub r#contribution_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The role of the contributor (e.g. author, editor, reviewer)."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Contributions with accounting for time or number."]
    pub r#contribution_instance: Vec<CitationCitedArtifactContributorshipEntryContributionInstance>,
    #[doc = "Indication of which contributor is the corresponding contributor for the role."]
    pub r#corresponding_contact: Option<super::super::types::Boolean>,
    #[doc = "Used to code order of authors."]
    pub r#list_order: Option<super::super::types::PositiveInt>,
}
impl serde::ser::Serialize for CitationCitedArtifactContributorshipEntry {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#name.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#initials.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("initials", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_initials", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#initials.as_ref() {
                    state.serialize_entry("initials", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#collective_name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("collectiveName", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_collectiveName", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#collective_name.as_ref() {
                    state.serialize_entry("collectiveName", some)?;
                }
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if !self.r#affiliation_info.is_empty() {
                state.serialize_entry("affiliationInfo", &self.r#affiliation_info)?;
            }
            if !self.r#address.is_empty() {
                state.serialize_entry("address", &self.r#address)?;
            }
            if !self.r#telecom.is_empty() {
                state.serialize_entry("telecom", &self.r#telecom)?;
            }
            if !self.r#contribution_type.is_empty() {
                state.serialize_entry("contributionType", &self.r#contribution_type)?;
            }
            if let Some(some) = self.r#role.as_ref() {
                state.serialize_entry("role", some)?;
            }
            if !self.r#contribution_instance.is_empty() {
                state.serialize_entry("contributionInstance", &self.r#contribution_instance)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#corresponding_contact.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("correspondingContact", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_correspondingContact", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#corresponding_contact.as_ref() {
                    state.serialize_entry("correspondingContact", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#list_order.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("listOrder", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_listOrder", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#list_order.as_ref() {
                    state.serialize_entry("listOrder", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactContributorshipEntry {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "initials")]
            Initials,
            #[serde(rename = "_initials")]
            InitialsPrimitiveElement,
            #[serde(rename = "collectiveName")]
            CollectiveName,
            #[serde(rename = "_collectiveName")]
            CollectiveNamePrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "affiliationInfo")]
            AffiliationInfo,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "contributionType")]
            ContributionType,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "contributionInstance")]
            ContributionInstance,
            #[serde(rename = "correspondingContact")]
            CorrespondingContact,
            #[serde(rename = "_correspondingContact")]
            CorrespondingContactPrimitiveElement,
            #[serde(rename = "listOrder")]
            ListOrder,
            #[serde(rename = "_listOrder")]
            ListOrderPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactContributorshipEntry;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipEntry")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactContributorshipEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<Box<super::super::types::HumanName>> = None;
                let mut r#initials: Option<super::super::types::String> = None;
                let mut r#collective_name: Option<super::super::types::String> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#affiliation_info: Option<
                    Vec<CitationCitedArtifactContributorshipEntryAffiliationInfo>,
                > = None;
                let mut r#address: Option<Vec<Box<super::super::types::Address>>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#contribution_type: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#contribution_instance: Option<
                    Vec<CitationCitedArtifactContributorshipEntryContributionInstance>,
                > = None;
                let mut r#corresponding_contact: Option<super::super::types::Boolean> = None;
                let mut r#list_order: Option<super::super::types::PositiveInt> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::Initials => {
                                if _ctx.from_json {
                                    let some = r#initials.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("initials"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#initials.is_some() {
                                        return Err(serde::de::Error::duplicate_field("initials"));
                                    }
                                    r#initials = Some(map_access.next_value()?);
                                }
                            }
                            Field::InitialsPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#initials.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_initials"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "initials",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "initials",
                                            "collectiveName",
                                            "identifier",
                                            "affiliationInfo",
                                            "address",
                                            "telecom",
                                            "contributionType",
                                            "role",
                                            "contributionInstance",
                                            "correspondingContact",
                                            "listOrder",
                                        ],
                                    ));
                                }
                            }
                            Field::CollectiveName => {
                                if _ctx.from_json {
                                    let some = r#collective_name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collectiveName",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#collective_name.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "collectiveName",
                                        ));
                                    }
                                    r#collective_name = Some(map_access.next_value()?);
                                }
                            }
                            Field::CollectiveNamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#collective_name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_collectiveName",
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
                                        "collectiveName",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "initials",
                                            "collectiveName",
                                            "identifier",
                                            "affiliationInfo",
                                            "address",
                                            "telecom",
                                            "contributionType",
                                            "role",
                                            "contributionInstance",
                                            "correspondingContact",
                                            "listOrder",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::AffiliationInfo => {
                                if r#affiliation_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "affiliationInfo",
                                    ));
                                }
                                r#affiliation_info = Some(map_access.next_value()?);
                            }
                            Field::Address => {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                r#address = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some(map_access.next_value()?);
                            }
                            Field::ContributionType => {
                                if r#contribution_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributionType",
                                    ));
                                }
                                r#contribution_type = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::ContributionInstance => {
                                if r#contribution_instance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributionInstance",
                                    ));
                                }
                                r#contribution_instance = Some(map_access.next_value()?);
                            }
                            Field::CorrespondingContact => {
                                if _ctx.from_json {
                                    let some =
                                        r#corresponding_contact.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "correspondingContact",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#corresponding_contact.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "correspondingContact",
                                        ));
                                    }
                                    r#corresponding_contact = Some(map_access.next_value()?);
                                }
                            }
                            Field::CorrespondingContactPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#corresponding_contact.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_correspondingContact",
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
                                        "correspondingContact",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "initials",
                                            "collectiveName",
                                            "identifier",
                                            "affiliationInfo",
                                            "address",
                                            "telecom",
                                            "contributionType",
                                            "role",
                                            "contributionInstance",
                                            "correspondingContact",
                                            "listOrder",
                                        ],
                                    ));
                                }
                            }
                            Field::ListOrder => {
                                if _ctx.from_json {
                                    let some = r#list_order.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("listOrder"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#list_order.is_some() {
                                        return Err(serde::de::Error::duplicate_field("listOrder"));
                                    }
                                    r#list_order = Some(map_access.next_value()?);
                                }
                            }
                            Field::ListOrderPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#list_order.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_listOrder",
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
                                        "listOrder",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
                                            "initials",
                                            "collectiveName",
                                            "identifier",
                                            "affiliationInfo",
                                            "address",
                                            "telecom",
                                            "contributionType",
                                            "role",
                                            "contributionInstance",
                                            "correspondingContact",
                                            "listOrder",
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
                                        "modifierExtension",
                                        "name",
                                        "initials",
                                        "collectiveName",
                                        "identifier",
                                        "affiliationInfo",
                                        "address",
                                        "telecom",
                                        "contributionType",
                                        "role",
                                        "contributionInstance",
                                        "correspondingContact",
                                        "listOrder",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactContributorshipEntry {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name,
                        r#initials,
                        r#collective_name,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#affiliation_info: r#affiliation_info.unwrap_or(vec![]),
                        r#address: r#address.unwrap_or(vec![]),
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#contribution_type: r#contribution_type.unwrap_or(vec![]),
                        r#role,
                        r#contribution_instance: r#contribution_instance.unwrap_or(vec![]),
                        r#corresponding_contact,
                        r#list_order,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Used to record a display of the author/contributor list without separate coding for each list member."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactContributorshipSummary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Used most commonly to express an author list or a contributorship statement."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The format for the display string."]
    pub r#style: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to code the producer or rule for creating the display string."]
    pub r#source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The display string for the author list, contributor list, or contributorship statement."]
    pub r#value: super::super::types::Markdown,
}
impl serde::ser::Serialize for CitationCitedArtifactContributorshipSummary {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#style.as_ref() {
                state.serialize_entry("style", some)?;
            }
            if let Some(some) = self.r#source.as_ref() {
                state.serialize_entry("source", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if self.r#value.id.is_some() || !self.r#value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#value.id.as_ref(),
                        extension: &self.r#value.extension,
                    };
                    state.serialize_entry("_value", &primitive_element)?;
                }
            } else {
                state.serialize_entry("value", &self.r#value)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactContributorshipSummary {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "style")]
            Style,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactContributorshipSummary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipSummary")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactContributorshipSummary, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#style: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#source: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<super::super::types::Markdown> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Style => {
                                if r#style.is_some() {
                                    return Err(serde::de::Error::duplicate_field("style"));
                                }
                                r#style = Some(map_access.next_value()?);
                            }
                            Field::Source => {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value()?);
                            }
                            Field::Value => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("value"));
                                    }
                                    r#value = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValuePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#value.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_value"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "value",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "style",
                                            "source",
                                            "value",
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
                                        "modifierExtension",
                                        "type",
                                        "style",
                                        "source",
                                        "value",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactContributorshipSummary {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#style,
                        r#source,
                        r#value: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This element is used to list authors and other contributors, their contact information, specific contributions, and summary statements."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifactContributorship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates if the list includes all authors and/or contributors."]
    pub r#complete: Option<super::super::types::Boolean>,
    #[doc = "An individual entity named in the author list or contributor list."]
    pub r#entry: Vec<CitationCitedArtifactContributorshipEntry>,
    #[doc = "Used to record a display of the author/contributor list without separate coding for each list member."]
    pub r#summary: Vec<CitationCitedArtifactContributorshipSummary>,
}
impl serde::ser::Serialize for CitationCitedArtifactContributorship {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#complete.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("complete", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_complete", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#complete.as_ref() {
                    state.serialize_entry("complete", some)?;
                }
            }
            if !self.r#entry.is_empty() {
                state.serialize_entry("entry", &self.r#entry)?;
            }
            if !self.r#summary.is_empty() {
                state.serialize_entry("summary", &self.r#summary)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifactContributorship {
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
            #[serde(rename = "complete")]
            Complete,
            #[serde(rename = "_complete")]
            CompletePrimitiveElement,
            #[serde(rename = "entry")]
            Entry,
            #[serde(rename = "summary")]
            Summary,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifactContributorship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorship")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CitationCitedArtifactContributorship, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#complete: Option<super::super::types::Boolean> = None;
                let mut r#entry: Option<Vec<CitationCitedArtifactContributorshipEntry>> = None;
                let mut r#summary: Option<Vec<CitationCitedArtifactContributorshipSummary>> = None;
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Complete => {
                                if _ctx.from_json {
                                    let some = r#complete.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("complete"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#complete.is_some() {
                                        return Err(serde::de::Error::duplicate_field("complete"));
                                    }
                                    r#complete = Some(map_access.next_value()?);
                                }
                            }
                            Field::CompletePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#complete.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_complete"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "complete",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "complete",
                                            "entry",
                                            "summary",
                                        ],
                                    ));
                                }
                            }
                            Field::Entry => {
                                if r#entry.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entry"));
                                }
                                r#entry = Some(map_access.next_value()?);
                            }
                            Field::Summary => {
                                if r#summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("summary"));
                                }
                                r#summary = Some(map_access.next_value()?);
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
                                        "complete",
                                        "entry",
                                        "summary",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifactContributorship {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#complete,
                        r#entry: r#entry.unwrap_or(vec![]),
                        r#summary: r#summary.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The article or artifact being described."]
#[derive(Default, Debug, Clone)]
pub struct CitationCitedArtifact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A formal identifier that is used to identify this citation when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A formal identifier that is used to identify things closely related to this citation."]
    pub r#related_identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "When the cited artifact was accessed."]
    pub r#date_accessed: Option<super::super::types::DateTime>,
    #[doc = "The defined version of the cited artifact."]
    pub r#version: Option<CitationCitedArtifactVersion>,
    #[doc = "The status of the cited artifact."]
    pub r#current_state: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An effective date or period for a status of the cited artifact."]
    pub r#status_date: Vec<CitationCitedArtifactStatusDate>,
    #[doc = "The title details of the article or artifact."]
    pub r#title: Vec<CitationCitedArtifactTitle>,
    #[doc = "Summary of the article or artifact."]
    pub r#abstract: Vec<CitationCitedArtifactAbstract>,
    #[doc = "The component of the article or artifact."]
    pub r#part: Option<CitationCitedArtifactPart>,
    #[doc = "The artifact related to the cited artifact."]
    pub r#relates_to: Vec<CitationCitedArtifactRelatesTo>,
    #[doc = "If multiple, used to represent alternative forms of the article that are not separate citations."]
    pub r#publication_form: Vec<CitationCitedArtifactPublicationForm>,
    #[doc = "Used for any URL for the article or artifact cited."]
    pub r#web_location: Vec<CitationCitedArtifactWebLocation>,
    #[doc = "The assignment to an organizing scheme."]
    pub r#classification: Vec<CitationCitedArtifactClassification>,
    #[doc = "This element is used to list authors and other contributors, their contact information, specific contributions, and summary statements."]
    pub r#contributorship: Option<CitationCitedArtifactContributorship>,
    #[doc = "Any additional information or content for the article or artifact."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for CitationCitedArtifact {
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
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if !self.r#related_identifier.is_empty() {
                state.serialize_entry("relatedIdentifier", &self.r#related_identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date_accessed.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dateAccessed", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_dateAccessed", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date_accessed.as_ref() {
                    state.serialize_entry("dateAccessed", some)?;
                }
            }
            if let Some(some) = self.r#version.as_ref() {
                state.serialize_entry("version", some)?;
            }
            if !self.r#current_state.is_empty() {
                state.serialize_entry("currentState", &self.r#current_state)?;
            }
            if !self.r#status_date.is_empty() {
                state.serialize_entry("statusDate", &self.r#status_date)?;
            }
            if !self.r#title.is_empty() {
                state.serialize_entry("title", &self.r#title)?;
            }
            if !self.r#abstract.is_empty() {
                state.serialize_entry("abstract", &self.r#abstract)?;
            }
            if let Some(some) = self.r#part.as_ref() {
                state.serialize_entry("part", some)?;
            }
            if !self.r#relates_to.is_empty() {
                state.serialize_entry("relatesTo", &self.r#relates_to)?;
            }
            if !self.r#publication_form.is_empty() {
                state.serialize_entry("publicationForm", &self.r#publication_form)?;
            }
            if !self.r#web_location.is_empty() {
                state.serialize_entry("webLocation", &self.r#web_location)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if let Some(some) = self.r#contributorship.as_ref() {
                state.serialize_entry("contributorship", some)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CitationCitedArtifact {
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
            #[serde(rename = "relatedIdentifier")]
            RelatedIdentifier,
            #[serde(rename = "dateAccessed")]
            DateAccessed,
            #[serde(rename = "_dateAccessed")]
            DateAccessedPrimitiveElement,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "currentState")]
            CurrentState,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "abstract")]
            Abstract,
            #[serde(rename = "part")]
            Part,
            #[serde(rename = "relatesTo")]
            RelatesTo,
            #[serde(rename = "publicationForm")]
            PublicationForm,
            #[serde(rename = "webLocation")]
            WebLocation,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "contributorship")]
            Contributorship,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CitationCitedArtifact;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifact")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CitationCitedArtifact, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#related_identifier: Option<Vec<Box<super::super::types::Identifier>>> =
                    None;
                let mut r#date_accessed: Option<super::super::types::DateTime> = None;
                let mut r#version: Option<CitationCitedArtifactVersion> = None;
                let mut r#current_state: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#status_date: Option<Vec<CitationCitedArtifactStatusDate>> = None;
                let mut r#title: Option<Vec<CitationCitedArtifactTitle>> = None;
                let mut r#abstract: Option<Vec<CitationCitedArtifactAbstract>> = None;
                let mut r#part: Option<CitationCitedArtifactPart> = None;
                let mut r#relates_to: Option<Vec<CitationCitedArtifactRelatesTo>> = None;
                let mut r#publication_form: Option<Vec<CitationCitedArtifactPublicationForm>> =
                    None;
                let mut r#web_location: Option<Vec<CitationCitedArtifactWebLocation>> = None;
                let mut r#classification: Option<Vec<CitationCitedArtifactClassification>> = None;
                let mut r#contributorship: Option<CitationCitedArtifactContributorship> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
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
                            Field::RelatedIdentifier => {
                                if r#related_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedIdentifier",
                                    ));
                                }
                                r#related_identifier = Some(map_access.next_value()?);
                            }
                            Field::DateAccessed => {
                                if _ctx.from_json {
                                    let some = r#date_accessed.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dateAccessed",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date_accessed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dateAccessed",
                                        ));
                                    }
                                    r#date_accessed = Some(map_access.next_value()?);
                                }
                            }
                            Field::DateAccessedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#date_accessed.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_dateAccessed",
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
                                        "dateAccessed",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "relatedIdentifier",
                                            "dateAccessed",
                                            "version",
                                            "currentState",
                                            "statusDate",
                                            "title",
                                            "abstract",
                                            "part",
                                            "relatesTo",
                                            "publicationForm",
                                            "webLocation",
                                            "classification",
                                            "contributorship",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Version => {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                r#version = Some(map_access.next_value()?);
                            }
                            Field::CurrentState => {
                                if r#current_state.is_some() {
                                    return Err(serde::de::Error::duplicate_field("currentState"));
                                }
                                r#current_state = Some(map_access.next_value()?);
                            }
                            Field::StatusDate => {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date = Some(map_access.next_value()?);
                            }
                            Field::Title => {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                r#title = Some(map_access.next_value()?);
                            }
                            Field::Abstract => {
                                if r#abstract.is_some() {
                                    return Err(serde::de::Error::duplicate_field("abstract"));
                                }
                                r#abstract = Some(map_access.next_value()?);
                            }
                            Field::Part => {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                r#part = Some(map_access.next_value()?);
                            }
                            Field::RelatesTo => {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to = Some(map_access.next_value()?);
                            }
                            Field::PublicationForm => {
                                if r#publication_form.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationForm",
                                    ));
                                }
                                r#publication_form = Some(map_access.next_value()?);
                            }
                            Field::WebLocation => {
                                if r#web_location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("webLocation"));
                                }
                                r#web_location = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::Contributorship => {
                                if r#contributorship.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributorship",
                                    ));
                                }
                                r#contributorship = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "relatedIdentifier",
                                        "dateAccessed",
                                        "version",
                                        "currentState",
                                        "statusDate",
                                        "title",
                                        "abstract",
                                        "part",
                                        "relatesTo",
                                        "publicationForm",
                                        "webLocation",
                                        "classification",
                                        "contributorship",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CitationCitedArtifact {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#related_identifier: r#related_identifier.unwrap_or(vec![]),
                        r#date_accessed,
                        r#version,
                        r#current_state: r#current_state.unwrap_or(vec![]),
                        r#status_date: r#status_date.unwrap_or(vec![]),
                        r#title: r#title.unwrap_or(vec![]),
                        r#abstract: r#abstract.unwrap_or(vec![]),
                        r#part,
                        r#relates_to: r#relates_to.unwrap_or(vec![]),
                        r#publication_form: r#publication_form.unwrap_or(vec![]),
                        r#web_location: r#web_location.unwrap_or(vec![]),
                        r#classification: r#classification.unwrap_or(vec![]),
                        r#contributorship,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The Citation Resource enables reference to any knowledge artifact for purposes of identification and attribution. The Citation Resource supports existing reference structures and developing publication practices such as versioning, expressing complex contributorship roles, and referencing computable resources."]
#[derive(Default, Debug, Clone)]
pub struct Citation {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An absolute URI that is used to identify this citation when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this citation when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the citation when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the citation author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the citation. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the citation."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this citation is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the citation was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the citation changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the citation."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the citation from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate citation instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the citation is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this citation is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "Use and/or publishing restrictions for the Citation, not for the cited artifact."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the citation content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Who authored the Citation."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Who edited the Citation."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Who reviewed the Citation."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Who endorsed the Citation."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A human-readable display of the citation."]
    pub r#summary: Vec<CitationSummary>,
    #[doc = "The assignment to an organizing scheme."]
    pub r#classification: Vec<CitationClassification>,
    #[doc = "Used for general notes and annotations not coded elsewhere."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The status of the citation."]
    pub r#current_state: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An effective date or period for a status of the citation."]
    pub r#status_date: Vec<CitationStatusDate>,
    #[doc = "Artifact related to the Citation Resource."]
    pub r#relates_to: Vec<CitationRelatesTo>,
    #[doc = "The article or artifact being described."]
    pub r#cited_artifact: Option<CitationCitedArtifact>,
}
impl crate::AnyResource for Citation {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Citation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Citation")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
                }
            }
            if let Some(some) = self.r#text.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if !self.r#contained.is_empty() {
                state.serialize_entry("contained", &self.r#contained)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#url.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("url", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_url", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#url.as_ref() {
                    state.serialize_entry("url", some)?;
                }
            }
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#version.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("version", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_version", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version.as_ref() {
                    state.serialize_entry("version", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#name.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("name", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_name", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#name.as_ref() {
                    state.serialize_entry("name", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#experimental.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("experimental", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_experimental", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#experimental.as_ref() {
                    state.serialize_entry("experimental", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#publisher.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("publisher", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_publisher", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#publisher.as_ref() {
                    state.serialize_entry("publisher", some)?;
                }
            }
            if !self.r#contact.is_empty() {
                state.serialize_entry("contact", &self.r#contact)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#use_context.is_empty() {
                state.serialize_entry("useContext", &self.r#use_context)?;
            }
            if !self.r#jurisdiction.is_empty() {
                state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#purpose.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("purpose", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_purpose", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#purpose.as_ref() {
                    state.serialize_entry("purpose", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#copyright.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("copyright", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_copyright", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#copyright.as_ref() {
                    state.serialize_entry("copyright", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#approval_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("approvalDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_approvalDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#approval_date.as_ref() {
                    state.serialize_entry("approvalDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastReviewDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastReviewDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_review_date.as_ref() {
                    state.serialize_entry("lastReviewDate", some)?;
                }
            }
            if let Some(some) = self.r#effective_period.as_ref() {
                state.serialize_entry("effectivePeriod", some)?;
            }
            if !self.r#author.is_empty() {
                state.serialize_entry("author", &self.r#author)?;
            }
            if !self.r#editor.is_empty() {
                state.serialize_entry("editor", &self.r#editor)?;
            }
            if !self.r#reviewer.is_empty() {
                state.serialize_entry("reviewer", &self.r#reviewer)?;
            }
            if !self.r#endorser.is_empty() {
                state.serialize_entry("endorser", &self.r#endorser)?;
            }
            if !self.r#summary.is_empty() {
                state.serialize_entry("summary", &self.r#summary)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#current_state.is_empty() {
                state.serialize_entry("currentState", &self.r#current_state)?;
            }
            if !self.r#status_date.is_empty() {
                state.serialize_entry("statusDate", &self.r#status_date)?;
            }
            if !self.r#relates_to.is_empty() {
                state.serialize_entry("relatesTo", &self.r#relates_to)?;
            }
            if let Some(some) = self.r#cited_artifact.as_ref() {
                state.serialize_entry("citedArtifact", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Citation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            #[serde(rename = "approvalDate")]
            ApprovalDate,
            #[serde(rename = "_approvalDate")]
            ApprovalDatePrimitiveElement,
            #[serde(rename = "lastReviewDate")]
            LastReviewDate,
            #[serde(rename = "_lastReviewDate")]
            LastReviewDatePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "editor")]
            Editor,
            #[serde(rename = "reviewer")]
            Reviewer,
            #[serde(rename = "endorser")]
            Endorser,
            #[serde(rename = "summary")]
            Summary,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "currentState")]
            CurrentState,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "relatesTo")]
            RelatesTo,
            #[serde(rename = "citedArtifact")]
            CitedArtifact,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Citation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Citation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Citation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#approval_date: Option<super::super::types::Date> = None;
                let mut r#last_review_date: Option<super::super::types::Date> = None;
                let mut r#effective_period: Option<Box<super::super::types::Period>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#editor: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#reviewer: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#endorser: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#summary: Option<Vec<CitationSummary>> = None;
                let mut r#classification: Option<Vec<CitationClassification>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#current_state: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#status_date: Option<Vec<CitationStatusDate>> = None;
                let mut r#relates_to: Option<Vec<CitationRelatesTo>> = None;
                let mut r#cited_artifact: Option<CitationCitedArtifact> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Citation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Citation",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
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
                                        "implicitRules",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_language"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
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
                            Field::Url => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#url.is_some() {
                                        return Err(serde::de::Error::duplicate_field("url"));
                                    }
                                    r#url = Some(map_access.next_value()?);
                                }
                            }
                            Field::UrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#url.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_url"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "url",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Version => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version.is_some() {
                                        return Err(serde::de::Error::duplicate_field("version"));
                                    }
                                    r#version = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_version"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "version",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_title"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_status"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Experimental => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#experimental.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "experimental",
                                        ));
                                    }
                                    r#experimental = Some(map_access.next_value()?);
                                }
                            }
                            Field::ExperimentalPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#experimental.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_experimental",
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
                                        "experimental",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_date"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Publisher => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#publisher.is_some() {
                                        return Err(serde::de::Error::duplicate_field("publisher"));
                                    }
                                    r#publisher = Some(map_access.next_value()?);
                                }
                            }
                            Field::PublisherPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#publisher.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_publisher",
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
                                        "publisher",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
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
                                        "description",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::UseContext => {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::Purpose => {
                                if _ctx.from_json {
                                    let some = r#purpose.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#purpose.is_some() {
                                        return Err(serde::de::Error::duplicate_field("purpose"));
                                    }
                                    r#purpose = Some(map_access.next_value()?);
                                }
                            }
                            Field::PurposePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#purpose.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_purpose"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "purpose",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::Copyright => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#copyright.is_some() {
                                        return Err(serde::de::Error::duplicate_field("copyright"));
                                    }
                                    r#copyright = Some(map_access.next_value()?);
                                }
                            }
                            Field::CopyrightPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#copyright.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_copyright",
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
                                        "copyright",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::ApprovalDate => {
                                if _ctx.from_json {
                                    let some = r#approval_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "approvalDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#approval_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "approvalDate",
                                        ));
                                    }
                                    r#approval_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ApprovalDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#approval_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_approvalDate",
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
                                        "approvalDate",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::LastReviewDate => {
                                if _ctx.from_json {
                                    let some = r#last_review_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastReviewDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_review_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastReviewDate",
                                        ));
                                    }
                                    r#last_review_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastReviewDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#last_review_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lastReviewDate",
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
                                        "lastReviewDate",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "url",
                                            "identifier",
                                            "version",
                                            "name",
                                            "title",
                                            "status",
                                            "experimental",
                                            "date",
                                            "publisher",
                                            "contact",
                                            "description",
                                            "useContext",
                                            "jurisdiction",
                                            "purpose",
                                            "copyright",
                                            "approvalDate",
                                            "lastReviewDate",
                                            "effectivePeriod",
                                            "author",
                                            "editor",
                                            "reviewer",
                                            "endorser",
                                            "summary",
                                            "classification",
                                            "note",
                                            "currentState",
                                            "statusDate",
                                            "relatesTo",
                                            "citedArtifact",
                                        ],
                                    ));
                                }
                            }
                            Field::EffectivePeriod => {
                                if r#effective_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectivePeriod",
                                    ));
                                }
                                r#effective_period = Some(map_access.next_value()?);
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Editor => {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                r#editor = Some(map_access.next_value()?);
                            }
                            Field::Reviewer => {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                r#reviewer = Some(map_access.next_value()?);
                            }
                            Field::Endorser => {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                r#endorser = Some(map_access.next_value()?);
                            }
                            Field::Summary => {
                                if r#summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("summary"));
                                }
                                r#summary = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::CurrentState => {
                                if r#current_state.is_some() {
                                    return Err(serde::de::Error::duplicate_field("currentState"));
                                }
                                r#current_state = Some(map_access.next_value()?);
                            }
                            Field::StatusDate => {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date = Some(map_access.next_value()?);
                            }
                            Field::RelatesTo => {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to = Some(map_access.next_value()?);
                            }
                            Field::CitedArtifact => {
                                if r#cited_artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("citedArtifact"));
                                }
                                r#cited_artifact = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "url",
                                        "identifier",
                                        "version",
                                        "name",
                                        "title",
                                        "status",
                                        "experimental",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "useContext",
                                        "jurisdiction",
                                        "purpose",
                                        "copyright",
                                        "approvalDate",
                                        "lastReviewDate",
                                        "effectivePeriod",
                                        "author",
                                        "editor",
                                        "reviewer",
                                        "endorser",
                                        "summary",
                                        "classification",
                                        "note",
                                        "currentState",
                                        "statusDate",
                                        "relatesTo",
                                        "citedArtifact",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Citation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#version,
                        r#name,
                        r#title,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#experimental,
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#purpose,
                        r#copyright,
                        r#approval_date,
                        r#last_review_date,
                        r#effective_period,
                        r#author: r#author.unwrap_or(vec![]),
                        r#editor: r#editor.unwrap_or(vec![]),
                        r#reviewer: r#reviewer.unwrap_or(vec![]),
                        r#endorser: r#endorser.unwrap_or(vec![]),
                        r#summary: r#summary.unwrap_or(vec![]),
                        r#classification: r#classification.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#current_state: r#current_state.unwrap_or(vec![]),
                        r#status_date: r#status_date.unwrap_or(vec![]),
                        r#relates_to: r#relates_to.unwrap_or(vec![]),
                        r#cited_artifact,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
