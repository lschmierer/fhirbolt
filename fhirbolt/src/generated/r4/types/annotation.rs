// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum AnnotationAuthor {
    Reference(Box<super::super::types::Reference>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for AnnotationAuthor {
    fn default() -> AnnotationAuthor {
        AnnotationAuthor::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct Annotation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#author: Option<AnnotationAuthor>,
    pub r#time: Option<super::super::types::DateTime>,
    pub r#text: super::super::types::Markdown,
}
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
                        state.serialize_entry("authorString", some)?;
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
                state.serialize_entry("time", some)?;
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
            state.serialize_entry("text", some)?;
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
                        "authorReference" => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorReference"));
                            }
                            r#author = Some(AnnotationAuthor::Reference(map_access.next_value()?));
                        }
                        "authorString" => {
                            let r#enum = r#author
                                .get_or_insert(AnnotationAuthor::String(Default::default()));
                            if let AnnotationAuthor::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authorString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("author[x]"));
                            }
                        }
                        "_authorString" => {
                            let r#enum = r#author
                                .get_or_insert(AnnotationAuthor::String(Default::default()));
                            if let AnnotationAuthor::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_authorString"));
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
                        "time" => {
                            let some = r#time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_time" => {
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
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
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
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "authorReference",
                                    "authorString",
                                    "time",
                                    "text",
                                ],
                            ))
                        }
                    }
                }
                Ok(Annotation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#author,
                    r#time,
                    r#text: r#text.ok_or(serde::de::Error::missing_field("text"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
