// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::types::Attachment;
impl serde::ser::Serialize for SerializationContext<&Attachment> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Attachment", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#content_type.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("contentType", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_contentType", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#content_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("contentType", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#data.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("data", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_data", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#data.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("data", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("url", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#size.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| Ok(v.to_string())) {
                    state.serialize_entry("size", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_size", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#size.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("size", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#hash.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("hash", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_hash", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#hash.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("hash", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#title.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("title", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_title", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#title.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("title", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#creation.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("creation", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_creation", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#creation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("creation", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#height.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("height", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_height", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#height.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("height", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#width.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("width", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_width", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#width.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("width", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#frames.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("frames", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_frames", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#frames.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("frames", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#duration.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("duration", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_duration", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#duration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("duration", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#pages.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("pages", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_pages", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#pages.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("pages", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Attachment>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Attachment>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Attachment> {
    type Value = Attachment;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Attachment>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Attachment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Attachment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Attachment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "contentType")]
                    ContentType,
                    #[serde(rename = "_contentType")]
                    ContentTypePrimitiveElement,
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "_language")]
                    LanguagePrimitiveElement,
                    #[serde(rename = "data")]
                    Data,
                    #[serde(rename = "_data")]
                    DataPrimitiveElement,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "size")]
                    Size,
                    #[serde(rename = "_size")]
                    SizePrimitiveElement,
                    #[serde(rename = "hash")]
                    Hash,
                    #[serde(rename = "_hash")]
                    HashPrimitiveElement,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "creation")]
                    Creation,
                    #[serde(rename = "_creation")]
                    CreationPrimitiveElement,
                    #[serde(rename = "height")]
                    Height,
                    #[serde(rename = "_height")]
                    HeightPrimitiveElement,
                    #[serde(rename = "width")]
                    Width,
                    #[serde(rename = "_width")]
                    WidthPrimitiveElement,
                    #[serde(rename = "frames")]
                    Frames,
                    #[serde(rename = "_frames")]
                    FramesPrimitiveElement,
                    #[serde(rename = "duration")]
                    Duration,
                    #[serde(rename = "_duration")]
                    DurationPrimitiveElement,
                    #[serde(rename = "pages")]
                    Pages,
                    #[serde(rename = "_pages")]
                    PagesPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "contentType",
                            "language",
                            "data",
                            "url",
                            "size",
                            "hash",
                            "title",
                            "creation",
                            "height",
                            "width",
                            "frames",
                            "duration",
                            "pages",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#content_type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#data: Option<fhirbolt_model::r5::types::Base64Binary> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::Url> = None;
                let mut r#size: Option<fhirbolt_model::r5::types::Integer64> = None;
                let mut r#hash: Option<fhirbolt_model::r5::types::Base64Binary> = None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#creation: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#height: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#width: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#frames: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#duration: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#pages: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ContentType => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#content_type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contentType"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#content_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contentType"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#content_type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ContentTypePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#content_type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_contentType"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contentType");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Data => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Base64Binary,
                                > = self.0.transmute();
                                r#data = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DataPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#data.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_data"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("data");
                            }
                        }
                        Field::Url => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Url,
                                > = self.0.transmute();
                                r#url = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Size => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#size.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("size"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#size.is_some() {
                                    return Err(serde::de::Error::duplicate_field("size"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer64,
                                > = self.0.transmute();
                                r#size = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SizePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#size.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_size"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("size");
                            }
                        }
                        Field::Hash => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#hash.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("hash"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#hash.is_some() {
                                    return Err(serde::de::Error::duplicate_field("hash"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Base64Binary,
                                > = self.0.transmute();
                                r#hash = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HashPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#hash.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_hash"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("hash");
                            }
                        }
                        Field::Title => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#title = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TitlePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::Creation => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#creation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("creation"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#creation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("creation"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#creation = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CreationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#creation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_creation"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("creation");
                            }
                        }
                        Field::Height => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#height.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("height"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#height.is_some() {
                                    return Err(serde::de::Error::duplicate_field("height"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#height = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HeightPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#height.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_height"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("height");
                            }
                        }
                        Field::Width => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#width.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("width"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#width.is_some() {
                                    return Err(serde::de::Error::duplicate_field("width"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#width = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WidthPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#width.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_width"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("width");
                            }
                        }
                        Field::Frames => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frames.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frames"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#frames.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frames"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#frames = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FramesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#frames.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frames"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frames");
                            }
                        }
                        Field::Duration => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#duration = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DurationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_duration"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("duration");
                            }
                        }
                        Field::Pages => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#pages.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pages"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#pages.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pages"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#pages = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PagesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#pages.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_pages"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("pages");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Attachment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#content_type,
                    r#language,
                    r#data,
                    r#url,
                    r#size,
                    r#hash,
                    r#title,
                    r#creation,
                    r#height,
                    r#width,
                    r#frames,
                    r#duration,
                    r#pages,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Attachment>> {
    type Value = Box<Attachment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Attachment>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Attachment>> {
    type Value = Vec<Attachment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Attachment>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Attachment>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Attachment> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
