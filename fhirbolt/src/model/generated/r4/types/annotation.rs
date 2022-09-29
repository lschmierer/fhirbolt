// Generated on 2022-09-28 by fhirbolt-codegen v0.1.0
#[doc = "The individual responsible for making the annotation."]
#[derive(Debug, Clone)]
pub enum AnnotationAuthor {
    Reference(Box<super::super::types::Reference>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl crate::model::ResourceOrElement for AnnotationAuthor {}
impl Default for AnnotationAuthor {
    fn default() -> AnnotationAuthor {
        AnnotationAuthor::Invalid
    }
}
#[doc = "Base StructureDefinition for Annotation Type: A  text note which also  contains information about who made the statement and when."]
#[derive(Default, Debug, Clone)]
pub struct Annotation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The individual responsible for making the annotation."]
    pub r#author: Option<AnnotationAuthor>,
    #[doc = "Indicates when this particular annotation was made."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "The text of the annotation in markdown format."]
    pub r#text: super::super::types::Markdown,
}
impl crate::model::ResourceOrElement for Annotation {}
impl serde::ser::Serialize for Annotation {
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
        if let Some(some) = self.r#author.as_ref() {
            match some {
                AnnotationAuthor::Reference(ref value) => {
                    state.serialize_entry("authorReference", value)?;
                }
                AnnotationAuthor::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("authorString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_authorString", &primitive_element)?;
                    }
                }
                AnnotationAuthor::Invalid => {
                    return Err(serde::ser::Error::custom("author is invalid"))
                }
            }
        }
        if let Some(some) = self.r#time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("time", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_time", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("text", &some)?;
        }
        if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#text.id,
                extension: &self.r#text.extension,
            };
            state.serialize_entry("_text", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Annotation {
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
            #[serde(rename = "authorReference")]
            AuthorReference,
            #[serde(rename = "authorString")]
            AuthorString,
            #[serde(rename = "_authorString")]
            AuthorStringPrimitiveElement,
            #[serde(rename = "time")]
            Time,
            #[serde(rename = "_time")]
            TimePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Annotation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Annotation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Annotation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#author: Option<AnnotationAuthor> = None;
                let mut r#time: Option<super::super::types::DateTime> = None;
                let mut r#text: Option<super::super::types::Markdown> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::AuthorReference => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorReference",
                                    ));
                                }
                                r#author =
                                    Some(AnnotationAuthor::Reference(map_access.next_value()?));
                            }
                            Field::AuthorString => {
                                let r#enum = r#author
                                    .get_or_insert(AnnotationAuthor::String(Default::default()));
                                if let AnnotationAuthor::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "authorString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("author[x]"));
                                }
                            }
                            Field::AuthorStringPrimitiveElement => {
                                let r#enum = r#author
                                    .get_or_insert(AnnotationAuthor::String(Default::default()));
                                if let AnnotationAuthor::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_authorString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_author[x]"));
                                }
                            }
                            Field::Time => {
                                let some = r#time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("time"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TimePrimitiveElement => {
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
                            }
                            Field::Text => {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TextPrimitiveElement => {
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
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "authorReference",
                                            "authorString",
                                            "time",
                                            "text",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Annotation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#author,
                        r#time,
                        r#text: if config.mode == crate::DeserializationMode::Lax {
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