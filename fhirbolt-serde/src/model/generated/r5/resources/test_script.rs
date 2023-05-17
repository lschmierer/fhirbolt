// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::TestScriptOrigin;
impl serde::ser::Serialize for SerializationContext<&TestScriptOrigin> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.origin", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#index.id.as_deref() == Some("$invalid") {
                return missing_field_error("index");
            }
            if let Some(some) = self.value.r#index.value.as_ref().map(Ok) {
                state.serialize_entry("index", &some?)?;
            }
            if self.value.r#index.id.is_some() || !self.value.r#index.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#index.id.as_ref(),
                    extension: &self.value.r#index.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_index", ctx)
                })?;
            }
        } else if self.value.r#index.id.as_deref() == Some("$invalid") {
            return missing_field_error("index");
        } else {
            self.with_context(&self.value.r#index, |ctx| {
                state.serialize_entry("index", ctx)
            })?;
        }
        if self.value.r#profile.id.as_deref() == Some("$invalid") {
            return missing_field_error("profile");
        } else {
            self.with_context(&self.value.r#profile, |ctx| {
                state.serialize_entry("profile", ctx)
            })?;
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptOrigin>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptOrigin>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptOrigin> {
    type Value = TestScriptOrigin;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptOrigin>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptOrigin;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptOrigin")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptOrigin, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "index")]
                    Index,
                    #[serde(rename = "_index")]
                    IndexPrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "index",
                            "profile",
                            "url",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#index: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#profile: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::Url> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Index => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#index.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("index"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#index.is_some() {
                                    return Err(serde::de::Error::duplicate_field("index"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#index = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IndexPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#index.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_index"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("index");
                            }
                        }
                        Field::Profile => {
                            if r#profile.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#profile = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptOrigin {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#index: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#index.unwrap_or(Default::default())
                    } else {
                        r#index.ok_or(serde::de::Error::missing_field("index"))?
                    },
                    r#profile: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#profile.unwrap_or(Default::default())
                    } else {
                        r#profile.ok_or(serde::de::Error::missing_field("profile"))?
                    },
                    r#url,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptOrigin>> {
    type Value = Box<TestScriptOrigin>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptOrigin>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptOrigin>> {
    type Value = Vec<TestScriptOrigin>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptOrigin>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptOrigin>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptOrigin> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptDestination;
impl serde::ser::Serialize for SerializationContext<&TestScriptDestination> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.destination", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#index.id.as_deref() == Some("$invalid") {
                return missing_field_error("index");
            }
            if let Some(some) = self.value.r#index.value.as_ref().map(Ok) {
                state.serialize_entry("index", &some?)?;
            }
            if self.value.r#index.id.is_some() || !self.value.r#index.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#index.id.as_ref(),
                    extension: &self.value.r#index.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_index", ctx)
                })?;
            }
        } else if self.value.r#index.id.as_deref() == Some("$invalid") {
            return missing_field_error("index");
        } else {
            self.with_context(&self.value.r#index, |ctx| {
                state.serialize_entry("index", ctx)
            })?;
        }
        if self.value.r#profile.id.as_deref() == Some("$invalid") {
            return missing_field_error("profile");
        } else {
            self.with_context(&self.value.r#profile, |ctx| {
                state.serialize_entry("profile", ctx)
            })?;
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptDestination>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptDestination>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptDestination> {
    type Value = TestScriptDestination;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptDestination>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptDestination;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptDestination")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptDestination, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "index")]
                    Index,
                    #[serde(rename = "_index")]
                    IndexPrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "index",
                            "profile",
                            "url",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#index: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#profile: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::Url> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Index => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#index.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("index"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#index.is_some() {
                                    return Err(serde::de::Error::duplicate_field("index"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#index = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IndexPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#index.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_index"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("index");
                            }
                        }
                        Field::Profile => {
                            if r#profile.is_some() {
                                return Err(serde::de::Error::duplicate_field("profile"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#profile = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptDestination {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#index: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#index.unwrap_or(Default::default())
                    } else {
                        r#index.ok_or(serde::de::Error::missing_field("index"))?
                    },
                    r#profile: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#profile.unwrap_or(Default::default())
                    } else {
                        r#profile.ok_or(serde::de::Error::missing_field("profile"))?
                    },
                    r#url,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptDestination>>
{
    type Value = Box<TestScriptDestination>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptDestination>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptDestination>>
{
    type Value = Vec<TestScriptDestination>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptDestination>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptDestination>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptDestination> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptMetadataLink;
impl serde::ser::Serialize for SerializationContext<&TestScriptMetadataLink> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.metadata.link", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            if let Some(some) = self.value.r#url.value.as_ref().map(Ok) {
                state.serialize_entry("url", &some?)?;
            }
            if self.value.r#url.id.is_some() || !self.value.r#url.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#url.id.as_ref(),
                    extension: &self.value.r#url.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_url", ctx))?;
            }
        } else if self.value.r#url.id.as_deref() == Some("$invalid") {
            return missing_field_error("url");
        } else {
            self.with_context(&self.value.r#url, |ctx| state.serialize_entry("url", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptMetadataLink>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptMetadataLink>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptMetadataLink> {
    type Value = TestScriptMetadataLink;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptMetadataLink>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptMetadataLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadataLink")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptMetadataLink, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "url", "description"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#url: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    fhirbolt_model::r5::types::Uri,
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
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
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
                Ok(TestScriptMetadataLink {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#url.unwrap_or(Default::default())
                    } else {
                        r#url.ok_or(serde::de::Error::missing_field("url"))?
                    },
                    r#description,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptMetadataLink>>
{
    type Value = Box<TestScriptMetadataLink>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptMetadataLink>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptMetadataLink>>
{
    type Value = Vec<TestScriptMetadataLink>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptMetadataLink>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptMetadataLink>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptMetadataLink> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptMetadataCapability;
impl serde::ser::Serialize for SerializationContext<&TestScriptMetadataCapability> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.metadata.capability", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#required.id.as_deref() == Some("$invalid") {
                return missing_field_error("required");
            }
            if let Some(some) = self.value.r#required.value.as_ref().map(Ok) {
                state.serialize_entry("required", &some?)?;
            }
            if self.value.r#required.id.is_some() || !self.value.r#required.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#required.id.as_ref(),
                    extension: &self.value.r#required.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_required", ctx)
                })?;
            }
        } else if self.value.r#required.id.as_deref() == Some("$invalid") {
            return missing_field_error("required");
        } else {
            self.with_context(&self.value.r#required, |ctx| {
                state.serialize_entry("required", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#validated.id.as_deref() == Some("$invalid") {
                return missing_field_error("validated");
            }
            if let Some(some) = self.value.r#validated.value.as_ref().map(Ok) {
                state.serialize_entry("validated", &some?)?;
            }
            if self.value.r#validated.id.is_some() || !self.value.r#validated.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#validated.id.as_ref(),
                    extension: &self.value.r#validated.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_validated", ctx)
                })?;
            }
        } else if self.value.r#validated.id.as_deref() == Some("$invalid") {
            return missing_field_error("validated");
        } else {
            self.with_context(&self.value.r#validated, |ctx| {
                state.serialize_entry("validated", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#origin.is_empty() {
                let values = self
                    .value
                    .r#origin
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("origin", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#origin
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#origin
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_origin", ctx)
                    })?;
                }
            }
        } else if !self.value.r#origin.is_empty() {
            self.with_context(&self.value.r#origin, |ctx| {
                state.serialize_entry("origin", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#destination.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("destination", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_destination", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#destination.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("destination", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#link.is_empty() {
                let values = self
                    .value
                    .r#link
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("link", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#link
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#link
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_link", ctx)
                    })?;
                }
            }
        } else if !self.value.r#link.is_empty() {
            self.with_context(&self.value.r#link, |ctx| state.serialize_entry("link", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#capabilities.id.as_deref() == Some("$invalid") {
                return missing_field_error("capabilities");
            }
            if let Some(some) = self.value.r#capabilities.value.as_ref().map(Ok) {
                state.serialize_entry("capabilities", &some?)?;
            }
            if self.value.r#capabilities.id.is_some()
                || !self.value.r#capabilities.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#capabilities.id.as_ref(),
                    extension: &self.value.r#capabilities.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_capabilities", ctx)
                })?;
            }
        } else if self.value.r#capabilities.id.as_deref() == Some("$invalid") {
            return missing_field_error("capabilities");
        } else {
            self.with_context(&self.value.r#capabilities, |ctx| {
                state.serialize_entry("capabilities", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptMetadataCapability>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptMetadataCapability>> {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptMetadataCapability>
{
    type Value = TestScriptMetadataCapability;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptMetadataCapability>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptMetadataCapability;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadataCapability")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptMetadataCapability, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "required")]
                    Required,
                    #[serde(rename = "_required")]
                    RequiredPrimitiveElement,
                    #[serde(rename = "validated")]
                    Validated,
                    #[serde(rename = "_validated")]
                    ValidatedPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "origin")]
                    Origin,
                    #[serde(rename = "_origin")]
                    OriginPrimitiveElement,
                    #[serde(rename = "destination")]
                    Destination,
                    #[serde(rename = "_destination")]
                    DestinationPrimitiveElement,
                    #[serde(rename = "link")]
                    Link,
                    #[serde(rename = "_link")]
                    LinkPrimitiveElement,
                    #[serde(rename = "capabilities")]
                    Capabilities,
                    #[serde(rename = "_capabilities")]
                    CapabilitiesPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "required",
                            "validated",
                            "description",
                            "origin",
                            "destination",
                            "link",
                            "capabilities",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#required: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#validated: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#origin: Option<Vec<fhirbolt_model::r5::types::Integer>> = None;
                let mut r#destination: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#link: Option<Vec<fhirbolt_model::r5::types::Uri>> = None;
                let mut r#capabilities: Option<fhirbolt_model::r5::types::Canonical> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Required => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("required"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#required.is_some() {
                                    return Err(serde::de::Error::duplicate_field("required"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#required = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequiredPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_required"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("required");
                            }
                        }
                        Field::Validated => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#validated.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("validated"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#validated.is_some() {
                                    return Err(serde::de::Error::duplicate_field("validated"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#validated = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValidatedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#validated.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_validated"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("validated");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Origin => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#origin.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#origin.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OriginPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#origin.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_origin"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("origin");
                            }
                        }
                        Field::Destination => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#destination.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#destination = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DestinationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#destination.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_destination"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("destination");
                            }
                        }
                        Field::Link => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#link.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("link"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#link.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LinkPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#link.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_link"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("link");
                            }
                        }
                        Field::Capabilities => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#capabilities.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capabilities"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#capabilities.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capabilities"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                r#capabilities = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CapabilitiesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#capabilities.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_capabilities"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("capabilities");
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
                Ok(TestScriptMetadataCapability {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#required: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#required.unwrap_or(Default::default())
                    } else {
                        r#required.ok_or(serde::de::Error::missing_field("required"))?
                    },
                    r#validated: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#validated.unwrap_or(Default::default())
                    } else {
                        r#validated.ok_or(serde::de::Error::missing_field("validated"))?
                    },
                    r#description,
                    r#origin: r#origin.unwrap_or(vec![]),
                    r#destination,
                    r#link: r#link.unwrap_or(vec![]),
                    r#capabilities: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#capabilities.unwrap_or(Default::default())
                    } else {
                        r#capabilities.ok_or(serde::de::Error::missing_field("capabilities"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptMetadataCapability>>
{
    type Value = Box<TestScriptMetadataCapability>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptMetadataCapability>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptMetadataCapability>>
{
    type Value = Vec<TestScriptMetadataCapability>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptMetadataCapability>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptMetadataCapability>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptMetadataCapability> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptMetadata;
impl serde::ser::Serialize for SerializationContext<&TestScriptMetadata> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.metadata", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#link.is_empty() {
            self.with_context(&self.value.r#link, |ctx| state.serialize_entry("link", ctx))?;
        }
        if !self.value.r#capability.is_empty() {
            self.with_context(&self.value.r#capability, |ctx| {
                state.serialize_entry("capability", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptMetadata>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptMetadata>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptMetadata> {
    type Value = TestScriptMetadata;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptMetadata>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptMetadata;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptMetadata")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptMetadata, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "link")]
                    Link,
                    #[serde(rename = "capability")]
                    Capability,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "link", "capability"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#link: Option<Vec<fhirbolt_model::r5::resources::TestScriptMetadataLink>> =
                    None;
                let mut r#capability: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptMetadataCapability>,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Link => {
                            if self.0.from == crate::context::Format::Json {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("link"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptMetadataLink>,
                                > = self.0.transmute();
                                r#link = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#link.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptMetadataLink,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Capability => {
                            if self.0.from == crate::context::Format::Json {
                                if r#capability.is_some() {
                                    return Err(serde::de::Error::duplicate_field("capability"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::TestScriptMetadataCapability,
                                    >,
                                > = self.0.transmute();
                                r#capability = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#capability.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptMetadataCapability,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(TestScriptMetadata {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#link: r#link.unwrap_or(vec![]),
                    r#capability: r#capability.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptMetadata>> {
    type Value = Box<TestScriptMetadata>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptMetadata>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptMetadata>> {
    type Value = Vec<TestScriptMetadata>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptMetadata>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptMetadata>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptMetadata> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptScope;
impl serde::ser::Serialize for SerializationContext<&TestScriptScope> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.scope", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#artifact.id.as_deref() == Some("$invalid") {
                return missing_field_error("artifact");
            }
            if let Some(some) = self.value.r#artifact.value.as_ref().map(Ok) {
                state.serialize_entry("artifact", &some?)?;
            }
            if self.value.r#artifact.id.is_some() || !self.value.r#artifact.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#artifact.id.as_ref(),
                    extension: &self.value.r#artifact.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_artifact", ctx)
                })?;
            }
        } else if self.value.r#artifact.id.as_deref() == Some("$invalid") {
            return missing_field_error("artifact");
        } else {
            self.with_context(&self.value.r#artifact, |ctx| {
                state.serialize_entry("artifact", ctx)
            })?;
        }
        if let Some(some) = self.value.r#conformance.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("conformance", ctx))?;
        }
        if let Some(some) = self.value.r#phase.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("phase", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptScope>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptScope>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptScope> {
    type Value = TestScriptScope;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptScope>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptScope;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptScope")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptScope, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "artifact")]
                    Artifact,
                    #[serde(rename = "_artifact")]
                    ArtifactPrimitiveElement,
                    #[serde(rename = "conformance")]
                    Conformance,
                    #[serde(rename = "phase")]
                    Phase,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "artifact",
                            "conformance",
                            "phase",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#artifact: Option<fhirbolt_model::r5::types::Canonical> = None;
                let mut r#conformance: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#phase: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Artifact => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#artifact.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("artifact"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("artifact"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                r#artifact = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ArtifactPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#artifact.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_artifact"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("artifact");
                            }
                        }
                        Field::Conformance => {
                            if r#conformance.is_some() {
                                return Err(serde::de::Error::duplicate_field("conformance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#conformance = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Phase => {
                            if r#phase.is_some() {
                                return Err(serde::de::Error::duplicate_field("phase"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#phase = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptScope {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#artifact: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#artifact.unwrap_or(Default::default())
                    } else {
                        r#artifact.ok_or(serde::de::Error::missing_field("artifact"))?
                    },
                    r#conformance,
                    r#phase,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptScope>> {
    type Value = Box<TestScriptScope>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptScope>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptScope>> {
    type Value = Vec<TestScriptScope>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptScope>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptScope>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptScope> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptFixture;
impl serde::ser::Serialize for SerializationContext<&TestScriptFixture> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.fixture", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#autocreate.id.as_deref() == Some("$invalid") {
                return missing_field_error("autocreate");
            }
            if let Some(some) = self.value.r#autocreate.value.as_ref().map(Ok) {
                state.serialize_entry("autocreate", &some?)?;
            }
            if self.value.r#autocreate.id.is_some() || !self.value.r#autocreate.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#autocreate.id.as_ref(),
                    extension: &self.value.r#autocreate.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_autocreate", ctx)
                })?;
            }
        } else if self.value.r#autocreate.id.as_deref() == Some("$invalid") {
            return missing_field_error("autocreate");
        } else {
            self.with_context(&self.value.r#autocreate, |ctx| {
                state.serialize_entry("autocreate", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#autodelete.id.as_deref() == Some("$invalid") {
                return missing_field_error("autodelete");
            }
            if let Some(some) = self.value.r#autodelete.value.as_ref().map(Ok) {
                state.serialize_entry("autodelete", &some?)?;
            }
            if self.value.r#autodelete.id.is_some() || !self.value.r#autodelete.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#autodelete.id.as_ref(),
                    extension: &self.value.r#autodelete.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_autodelete", ctx)
                })?;
            }
        } else if self.value.r#autodelete.id.as_deref() == Some("$invalid") {
            return missing_field_error("autodelete");
        } else {
            self.with_context(&self.value.r#autodelete, |ctx| {
                state.serialize_entry("autodelete", ctx)
            })?;
        }
        if let Some(some) = self.value.r#resource.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("resource", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptFixture>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptFixture>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptFixture> {
    type Value = TestScriptFixture;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptFixture>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptFixture;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptFixture")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptFixture, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "autocreate")]
                    Autocreate,
                    #[serde(rename = "_autocreate")]
                    AutocreatePrimitiveElement,
                    #[serde(rename = "autodelete")]
                    Autodelete,
                    #[serde(rename = "_autodelete")]
                    AutodeletePrimitiveElement,
                    #[serde(rename = "resource")]
                    Resource,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "autocreate",
                            "autodelete",
                            "resource",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#autocreate: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#autodelete: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#resource: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Autocreate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#autocreate.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("autocreate"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#autocreate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("autocreate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#autocreate = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AutocreatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#autocreate.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_autocreate"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("autocreate");
                            }
                        }
                        Field::Autodelete => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#autodelete.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("autodelete"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#autodelete.is_some() {
                                    return Err(serde::de::Error::duplicate_field("autodelete"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#autodelete = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AutodeletePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#autodelete.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_autodelete"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("autodelete");
                            }
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#resource = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptFixture {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#autocreate: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#autocreate.unwrap_or(Default::default())
                    } else {
                        r#autocreate.ok_or(serde::de::Error::missing_field("autocreate"))?
                    },
                    r#autodelete: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#autodelete.unwrap_or(Default::default())
                    } else {
                        r#autodelete.ok_or(serde::de::Error::missing_field("autodelete"))?
                    },
                    r#resource,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptFixture>> {
    type Value = Box<TestScriptFixture>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptFixture>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptFixture>> {
    type Value = Vec<TestScriptFixture>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptFixture>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptFixture>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptFixture> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptVariable;
impl serde::ser::Serialize for SerializationContext<&TestScriptVariable> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.variable", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref().map(Ok) {
                state.serialize_entry("name", &some?)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else if self.value.r#name.id.as_deref() == Some("$invalid") {
            return missing_field_error("name");
        } else {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#default_value.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("defaultValue", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_defaultValue", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#default_value.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("defaultValue", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#expression.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("expression", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expression", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#expression.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#header_field.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("headerField", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_headerField", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#header_field.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("headerField", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#hint.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("hint", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_hint", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#hint.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("hint", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("path", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_path", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#path.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("path", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#source_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sourceId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sourceId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#source_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sourceId", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptVariable>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptVariable>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptVariable> {
    type Value = TestScriptVariable;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptVariable>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptVariable;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptVariable")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptVariable, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "defaultValue")]
                    DefaultValue,
                    #[serde(rename = "_defaultValue")]
                    DefaultValuePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    #[serde(rename = "_expression")]
                    ExpressionPrimitiveElement,
                    #[serde(rename = "headerField")]
                    HeaderField,
                    #[serde(rename = "_headerField")]
                    HeaderFieldPrimitiveElement,
                    #[serde(rename = "hint")]
                    Hint,
                    #[serde(rename = "_hint")]
                    HintPrimitiveElement,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "sourceId")]
                    SourceId,
                    #[serde(rename = "_sourceId")]
                    SourceIdPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "name",
                            "defaultValue",
                            "description",
                            "expression",
                            "headerField",
                            "hint",
                            "path",
                            "sourceId",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#default_value: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#expression: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#header_field: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#hint: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#source_id: Option<fhirbolt_model::r5::types::Id> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::DefaultValue => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#default_value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("defaultValue"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("defaultValue"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#default_value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DefaultValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#default_value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_defaultValue"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("defaultValue");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Expression => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#expression = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExpressionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_expression"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expression");
                            }
                        }
                        Field::HeaderField => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#header_field.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("headerField"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#header_field.is_some() {
                                    return Err(serde::de::Error::duplicate_field("headerField"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#header_field = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HeaderFieldPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#header_field.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_headerField"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("headerField");
                            }
                        }
                        Field::Hint => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#hint.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("hint"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#hint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("hint"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#hint = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HintPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#hint.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_hint"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("hint");
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::SourceId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#source_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#source_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SourceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sourceId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sourceId");
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
                Ok(TestScriptVariable {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#name.unwrap_or(Default::default())
                    } else {
                        r#name.ok_or(serde::de::Error::missing_field("name"))?
                    },
                    r#default_value,
                    r#description,
                    r#expression,
                    r#header_field,
                    r#hint,
                    r#path,
                    r#source_id,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptVariable>> {
    type Value = Box<TestScriptVariable>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptVariable>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptVariable>> {
    type Value = Vec<TestScriptVariable>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptVariable>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptVariable>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptVariable> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetupActionOperationRequestHeader;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetupActionOperationRequestHeader> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup.action.operation.requestHeader", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#field.id.as_deref() == Some("$invalid") {
                return missing_field_error("field");
            }
            if let Some(some) = self.value.r#field.value.as_ref().map(Ok) {
                state.serialize_entry("field", &some?)?;
            }
            if self.value.r#field.id.is_some() || !self.value.r#field.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#field.id.as_ref(),
                    extension: &self.value.r#field.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_field", ctx)
                })?;
            }
        } else if self.value.r#field.id.as_deref() == Some("$invalid") {
            return missing_field_error("field");
        } else {
            self.with_context(&self.value.r#field, |ctx| {
                state.serialize_entry("field", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#value.id.as_deref() == Some("$invalid") {
                return missing_field_error("value");
            }
            if let Some(some) = self.value.r#value.value.as_ref().map(Ok) {
                state.serialize_entry("value", &some?)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else if self.value.r#value.id.as_deref() == Some("$invalid") {
            return missing_field_error("value");
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<TestScriptSetupActionOperationRequestHeader>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<TestScriptSetupActionOperationRequestHeader>>
{
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptSetupActionOperationRequestHeader>
{
    type Value = TestScriptSetupActionOperationRequestHeader;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<TestScriptSetupActionOperationRequestHeader>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetupActionOperationRequestHeader;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionOperationRequestHeader")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionOperationRequestHeader, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "field")]
                    Field,
                    #[serde(rename = "_field")]
                    FieldPrimitiveElement,
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "field", "value"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#field: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#value: Option<fhirbolt_model::r5::types::String> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Field => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#field.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("field"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#field.is_some() {
                                    return Err(serde::de::Error::duplicate_field("field"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#field = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FieldPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#field.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_field"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("field");
                            }
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
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
                Ok(TestScriptSetupActionOperationRequestHeader {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#field: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#field.unwrap_or(Default::default())
                    } else {
                        r#field.ok_or(serde::de::Error::missing_field("field"))?
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptSetupActionOperationRequestHeader>>
{
    type Value = Box<TestScriptSetupActionOperationRequestHeader>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetupActionOperationRequestHeader>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptSetupActionOperationRequestHeader>>
{
    type Value = Vec<TestScriptSetupActionOperationRequestHeader>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<TestScriptSetupActionOperationRequestHeader>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetupActionOperationRequestHeader>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    TestScriptSetupActionOperationRequestHeader,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetupActionOperation;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetupActionOperation> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup.action.operation", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#resource.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("resource", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_resource", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#resource.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("resource", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#label.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("label", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_label", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#label.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("label", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#accept.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("accept", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_accept", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#accept.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("accept", ctx))?;
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
            if let Some(some) = self.value.r#destination.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("destination", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_destination", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#destination.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("destination", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#encode_request_url.id.as_deref() == Some("$invalid") {
                return missing_field_error("encodeRequestUrl");
            }
            if let Some(some) = self.value.r#encode_request_url.value.as_ref().map(Ok) {
                state.serialize_entry("encodeRequestUrl", &some?)?;
            }
            if self.value.r#encode_request_url.id.is_some()
                || !self.value.r#encode_request_url.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#encode_request_url.id.as_ref(),
                    extension: &self.value.r#encode_request_url.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_encodeRequestUrl", ctx)
                })?;
            }
        } else if self.value.r#encode_request_url.id.as_deref() == Some("$invalid") {
            return missing_field_error("encodeRequestUrl");
        } else {
            self.with_context(&self.value.r#encode_request_url, |ctx| {
                state.serialize_entry("encodeRequestUrl", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#method.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("method", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_method", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#origin.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("origin", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_origin", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#origin.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("origin", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#params.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("params", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_params", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#params.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("params", ctx))?;
        }
        if !self.value.r#request_header.is_empty() {
            self.with_context(&self.value.r#request_header, |ctx| {
                state.serialize_entry("requestHeader", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#request_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requestId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requestId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#request_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#response_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("responseId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_responseId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#response_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responseId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#source_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sourceId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sourceId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#source_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sourceId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#target_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("targetId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_targetId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#target_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("targetId", ctx))?;
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptSetupActionOperation>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptSetupActionOperation>> {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptSetupActionOperation>
{
    type Value = TestScriptSetupActionOperation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptSetupActionOperation>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetupActionOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionOperation, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "accept")]
                    Accept,
                    #[serde(rename = "_accept")]
                    AcceptPrimitiveElement,
                    #[serde(rename = "contentType")]
                    ContentType,
                    #[serde(rename = "_contentType")]
                    ContentTypePrimitiveElement,
                    #[serde(rename = "destination")]
                    Destination,
                    #[serde(rename = "_destination")]
                    DestinationPrimitiveElement,
                    #[serde(rename = "encodeRequestUrl")]
                    EncodeRequestUrl,
                    #[serde(rename = "_encodeRequestUrl")]
                    EncodeRequestUrlPrimitiveElement,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "_method")]
                    MethodPrimitiveElement,
                    #[serde(rename = "origin")]
                    Origin,
                    #[serde(rename = "_origin")]
                    OriginPrimitiveElement,
                    #[serde(rename = "params")]
                    Params,
                    #[serde(rename = "_params")]
                    ParamsPrimitiveElement,
                    #[serde(rename = "requestHeader")]
                    RequestHeader,
                    #[serde(rename = "requestId")]
                    RequestId,
                    #[serde(rename = "_requestId")]
                    RequestIdPrimitiveElement,
                    #[serde(rename = "responseId")]
                    ResponseId,
                    #[serde(rename = "_responseId")]
                    ResponseIdPrimitiveElement,
                    #[serde(rename = "sourceId")]
                    SourceId,
                    #[serde(rename = "_sourceId")]
                    SourceIdPrimitiveElement,
                    #[serde(rename = "targetId")]
                    TargetId,
                    #[serde(rename = "_targetId")]
                    TargetIdPrimitiveElement,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "resource",
                            "label",
                            "description",
                            "accept",
                            "contentType",
                            "destination",
                            "encodeRequestUrl",
                            "method",
                            "origin",
                            "params",
                            "requestHeader",
                            "requestId",
                            "responseId",
                            "sourceId",
                            "targetId",
                            "url",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#label: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#accept: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#content_type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#destination: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#encode_request_url: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#method: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#origin: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#params: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#request_header: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptSetupActionOperationRequestHeader>,
                > = None;
                let mut r#request_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#response_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#source_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#target_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#url: Option<fhirbolt_model::r5::types::String> = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
                            }
                        }
                        Field::Label => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#label = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Accept => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#accept.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accept"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#accept.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accept"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#accept = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AcceptPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#accept.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_accept"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("accept");
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
                        Field::Destination => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#destination.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#destination = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DestinationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#destination.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_destination"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("destination");
                            }
                        }
                        Field::EncodeRequestUrl => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#encode_request_url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "encodeRequestUrl",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#encode_request_url.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "encodeRequestUrl",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#encode_request_url =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EncodeRequestUrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#encode_request_url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_encodeRequestUrl",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("encodeRequestUrl");
                            }
                        }
                        Field::Method => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#method.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#method = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MethodPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#method.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_method"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("method");
                            }
                        }
                        Field::Origin => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#origin.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#origin = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OriginPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#origin.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_origin"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("origin");
                            }
                        }
                        Field::Params => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#params.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("params"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#params.is_some() {
                                    return Err(serde::de::Error::duplicate_field("params"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#params = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ParamsPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#params.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_params"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("params");
                            }
                        }
                        Field::RequestHeader => {
                            if self.0.from == crate::context::Format::Json {
                                if r#request_header.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestHeader"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: TestScriptSetupActionOperationRequestHeader >> = self . 0 . transmute () ;
                                r#request_header =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#request_header.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: TestScriptSetupActionOperationRequestHeader > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequestId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#request_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#request_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequestIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requestId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requestId");
                            }
                        }
                        Field::ResponseId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responseId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#response_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responseId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#response_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResponseIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_responseId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("responseId");
                            }
                        }
                        Field::SourceId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#source_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#source_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SourceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sourceId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sourceId");
                            }
                        }
                        Field::TargetId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#target_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#target_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#target_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TargetIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#target_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_targetId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("targetId");
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
                                    fhirbolt_model::r5::types::String,
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptSetupActionOperation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#resource,
                    r#label,
                    r#description,
                    r#accept,
                    r#content_type,
                    r#destination,
                    r#encode_request_url: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#encode_request_url.unwrap_or(Default::default())
                    } else {
                        r#encode_request_url
                            .ok_or(serde::de::Error::missing_field("encodeRequestUrl"))?
                    },
                    r#method,
                    r#origin,
                    r#params,
                    r#request_header: r#request_header.unwrap_or(vec![]),
                    r#request_id,
                    r#response_id,
                    r#source_id,
                    r#target_id,
                    r#url,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptSetupActionOperation>>
{
    type Value = Box<TestScriptSetupActionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetupActionOperation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptSetupActionOperation>>
{
    type Value = Vec<TestScriptSetupActionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptSetupActionOperation>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetupActionOperation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptSetupActionOperation> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirement;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetupActionAssertRequirement> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup.action.assert.requirement", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        {
            use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink as _Enum;
            if let Some(some) = self.value.r#link.as_ref() {
                match some {
                    _Enum::Uri(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("linkUri", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_linkUri", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| state.serialize_entry("linkUri", ctx))?;
                        }
                    }
                    _Enum::Canonical(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("linkCanonical", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_linkCanonical", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("linkCanonical", ctx)
                            })?;
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("link is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptSetupActionAssertRequirement>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptSetupActionAssertRequirement>> {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptSetupActionAssertRequirement>
{
    type Value = TestScriptSetupActionAssertRequirement;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptSetupActionAssertRequirement>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetupActionAssertRequirement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionAssertRequirement")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionAssertRequirement, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "linkUri")]
                    LinkUri,
                    #[serde(rename = "_linkUri")]
                    LinkUriPrimitiveElement,
                    #[serde(rename = "linkCanonical")]
                    LinkCanonical,
                    #[serde(rename = "_linkCanonical")]
                    LinkCanonicalPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "linkUri",
                            "linkCanonical",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#link: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LinkUri => {
                            use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#link.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("linkUri"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("link[x]"));
                                }
                            } else {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkUri"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Uri>,
                                > = self.0.transmute();
                                r#link =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::LinkUriPrimitiveElement => {
                            use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#link.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_linkUri"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_link[x]"));
                                }
                            } else {
                                return unknown_field_error("linkUri");
                            }
                        }
                        Field::LinkCanonical => {
                            use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#link.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "linkCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("link[x]"));
                                }
                            } else {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkCanonical"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Canonical>,
                                > = self.0.transmute();
                                r#link = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::LinkCanonicalPrimitiveElement => {
                            use fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirementLink as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#link.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_linkCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_link[x]"));
                                }
                            } else {
                                return unknown_field_error("linkCanonical");
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
                Ok(TestScriptSetupActionAssertRequirement {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#link,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptSetupActionAssertRequirement>>
{
    type Value = Box<TestScriptSetupActionAssertRequirement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetupActionAssertRequirement>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptSetupActionAssertRequirement>>
{
    type Value = Vec<TestScriptSetupActionAssertRequirement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<TestScriptSetupActionAssertRequirement>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetupActionAssertRequirement>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptSetupActionAssertRequirement> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetupActionAssert;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetupActionAssert> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup.action.assert", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#label.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("label", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_label", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#label.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("label", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#direction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("direction", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_direction", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#direction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("direction", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#compare_to_source_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("compareToSourceId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_compareToSourceId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#compare_to_source_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("compareToSourceId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#compare_to_source_expression.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("compareToSourceExpression", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_compareToSourceExpression", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#compare_to_source_expression.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("compareToSourceExpression", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#compare_to_source_path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("compareToSourcePath", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_compareToSourcePath", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#compare_to_source_path.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("compareToSourcePath", ctx)
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
            if let Some(some) = self.value.r#default_manual_completion.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("defaultManualCompletion", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_defaultManualCompletion", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#default_manual_completion.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("defaultManualCompletion", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#expression.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("expression", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expression", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#expression.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#header_field.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("headerField", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_headerField", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#header_field.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("headerField", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#minimum_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("minimumId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_minimumId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#minimum_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("minimumId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#navigation_links.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("navigationLinks", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_navigationLinks", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#navigation_links.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("navigationLinks", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#operator.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("operator", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_operator", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#operator.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("operator", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("path", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_path", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#path.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("path", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#request_method.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requestMethod", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requestMethod", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#request_method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestMethod", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#request_url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requestURL", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requestURL", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#request_url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestURL", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#resource.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("resource", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_resource", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#resource.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("resource", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#response.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("response", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_response", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#response.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("response", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#response_code.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("responseCode", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_responseCode", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#response_code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responseCode", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#source_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sourceId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sourceId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#source_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sourceId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#stop_test_on_fail.id.as_deref() == Some("$invalid") {
                return missing_field_error("stopTestOnFail");
            }
            if let Some(some) = self.value.r#stop_test_on_fail.value.as_ref().map(Ok) {
                state.serialize_entry("stopTestOnFail", &some?)?;
            }
            if self.value.r#stop_test_on_fail.id.is_some()
                || !self.value.r#stop_test_on_fail.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#stop_test_on_fail.id.as_ref(),
                    extension: &self.value.r#stop_test_on_fail.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_stopTestOnFail", ctx)
                })?;
            }
        } else if self.value.r#stop_test_on_fail.id.as_deref() == Some("$invalid") {
            return missing_field_error("stopTestOnFail");
        } else {
            self.with_context(&self.value.r#stop_test_on_fail, |ctx| {
                state.serialize_entry("stopTestOnFail", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#validate_profile_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("validateProfileId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_validateProfileId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#validate_profile_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("validateProfileId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#value.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("value", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_value", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#value.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("value", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#warning_only.id.as_deref() == Some("$invalid") {
                return missing_field_error("warningOnly");
            }
            if let Some(some) = self.value.r#warning_only.value.as_ref().map(Ok) {
                state.serialize_entry("warningOnly", &some?)?;
            }
            if self.value.r#warning_only.id.is_some()
                || !self.value.r#warning_only.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#warning_only.id.as_ref(),
                    extension: &self.value.r#warning_only.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_warningOnly", ctx)
                })?;
            }
        } else if self.value.r#warning_only.id.as_deref() == Some("$invalid") {
            return missing_field_error("warningOnly");
        } else {
            self.with_context(&self.value.r#warning_only, |ctx| {
                state.serialize_entry("warningOnly", ctx)
            })?;
        }
        if !self.value.r#requirement.is_empty() {
            self.with_context(&self.value.r#requirement, |ctx| {
                state.serialize_entry("requirement", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptSetupActionAssert>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptSetupActionAssert>> {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptSetupActionAssert>
{
    type Value = TestScriptSetupActionAssert;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptSetupActionAssert>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetupActionAssert;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupActionAssert")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<TestScriptSetupActionAssert, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "direction")]
                    Direction,
                    #[serde(rename = "_direction")]
                    DirectionPrimitiveElement,
                    #[serde(rename = "compareToSourceId")]
                    CompareToSourceId,
                    #[serde(rename = "_compareToSourceId")]
                    CompareToSourceIdPrimitiveElement,
                    #[serde(rename = "compareToSourceExpression")]
                    CompareToSourceExpression,
                    #[serde(rename = "_compareToSourceExpression")]
                    CompareToSourceExpressionPrimitiveElement,
                    #[serde(rename = "compareToSourcePath")]
                    CompareToSourcePath,
                    #[serde(rename = "_compareToSourcePath")]
                    CompareToSourcePathPrimitiveElement,
                    #[serde(rename = "contentType")]
                    ContentType,
                    #[serde(rename = "_contentType")]
                    ContentTypePrimitiveElement,
                    #[serde(rename = "defaultManualCompletion")]
                    DefaultManualCompletion,
                    #[serde(rename = "_defaultManualCompletion")]
                    DefaultManualCompletionPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    #[serde(rename = "_expression")]
                    ExpressionPrimitiveElement,
                    #[serde(rename = "headerField")]
                    HeaderField,
                    #[serde(rename = "_headerField")]
                    HeaderFieldPrimitiveElement,
                    #[serde(rename = "minimumId")]
                    MinimumId,
                    #[serde(rename = "_minimumId")]
                    MinimumIdPrimitiveElement,
                    #[serde(rename = "navigationLinks")]
                    NavigationLinks,
                    #[serde(rename = "_navigationLinks")]
                    NavigationLinksPrimitiveElement,
                    #[serde(rename = "operator")]
                    Operator,
                    #[serde(rename = "_operator")]
                    OperatorPrimitiveElement,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "requestMethod")]
                    RequestMethod,
                    #[serde(rename = "_requestMethod")]
                    RequestMethodPrimitiveElement,
                    #[serde(rename = "requestURL")]
                    RequestUrl,
                    #[serde(rename = "_requestURL")]
                    RequestUrlPrimitiveElement,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    #[serde(rename = "response")]
                    Response,
                    #[serde(rename = "_response")]
                    ResponsePrimitiveElement,
                    #[serde(rename = "responseCode")]
                    ResponseCode,
                    #[serde(rename = "_responseCode")]
                    ResponseCodePrimitiveElement,
                    #[serde(rename = "sourceId")]
                    SourceId,
                    #[serde(rename = "_sourceId")]
                    SourceIdPrimitiveElement,
                    #[serde(rename = "stopTestOnFail")]
                    StopTestOnFail,
                    #[serde(rename = "_stopTestOnFail")]
                    StopTestOnFailPrimitiveElement,
                    #[serde(rename = "validateProfileId")]
                    ValidateProfileId,
                    #[serde(rename = "_validateProfileId")]
                    ValidateProfileIdPrimitiveElement,
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    #[serde(rename = "warningOnly")]
                    WarningOnly,
                    #[serde(rename = "_warningOnly")]
                    WarningOnlyPrimitiveElement,
                    #[serde(rename = "requirement")]
                    Requirement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "label",
                            "description",
                            "direction",
                            "compareToSourceId",
                            "compareToSourceExpression",
                            "compareToSourcePath",
                            "contentType",
                            "defaultManualCompletion",
                            "expression",
                            "headerField",
                            "minimumId",
                            "navigationLinks",
                            "operator",
                            "path",
                            "requestMethod",
                            "requestURL",
                            "resource",
                            "response",
                            "responseCode",
                            "sourceId",
                            "stopTestOnFail",
                            "validateProfileId",
                            "value",
                            "warningOnly",
                            "requirement",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#label: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#direction: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#compare_to_source_id: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#compare_to_source_expression: Option<fhirbolt_model::r5::types::String> =
                    None;
                let mut r#compare_to_source_path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#content_type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#default_manual_completion: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#expression: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#header_field: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#minimum_id: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#navigation_links: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#operator: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#request_method: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#request_url: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#response: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#response_code: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#source_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#stop_test_on_fail: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#validate_profile_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#value: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#warning_only: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#requirement: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptSetupActionAssertRequirement>,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Label => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#label = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Direction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#direction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("direction"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#direction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("direction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#direction = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DirectionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#direction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_direction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("direction");
                            }
                        }
                        Field::CompareToSourceId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#compare_to_source_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourceId",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#compare_to_source_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourceId",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#compare_to_source_id =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CompareToSourceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#compare_to_source_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_compareToSourceId",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("compareToSourceId");
                            }
                        }
                        Field::CompareToSourceExpression => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#compare_to_source_expression
                                    .get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourceExpression",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#compare_to_source_expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourceExpression",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#compare_to_source_expression =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CompareToSourceExpressionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#compare_to_source_expression
                                    .get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_compareToSourceExpression",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("compareToSourceExpression");
                            }
                        }
                        Field::CompareToSourcePath => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#compare_to_source_path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourcePath",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#compare_to_source_path.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "compareToSourcePath",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#compare_to_source_path =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CompareToSourcePathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#compare_to_source_path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_compareToSourcePath",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("compareToSourcePath");
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
                        Field::DefaultManualCompletion => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#default_manual_completion.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultManualCompletion",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#default_manual_completion.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultManualCompletion",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#default_manual_completion =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DefaultManualCompletionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#default_manual_completion.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultManualCompletion",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("defaultManualCompletion");
                            }
                        }
                        Field::Expression => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#expression = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExpressionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_expression"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expression");
                            }
                        }
                        Field::HeaderField => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#header_field.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("headerField"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#header_field.is_some() {
                                    return Err(serde::de::Error::duplicate_field("headerField"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#header_field = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HeaderFieldPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#header_field.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_headerField"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("headerField");
                            }
                        }
                        Field::MinimumId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#minimum_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minimumId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#minimum_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minimumId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#minimum_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MinimumIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#minimum_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_minimumId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("minimumId");
                            }
                        }
                        Field::NavigationLinks => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#navigation_links.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "navigationLinks",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#navigation_links.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "navigationLinks",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#navigation_links =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NavigationLinksPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#navigation_links.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_navigationLinks",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("navigationLinks");
                            }
                        }
                        Field::Operator => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#operator.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operator"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#operator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operator"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#operator = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OperatorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#operator.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_operator"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("operator");
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::RequestMethod => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_method.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestMethod"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#request_method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestMethod"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#request_method =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequestMethodPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_method.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_requestMethod",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requestMethod");
                            }
                        }
                        Field::RequestUrl => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestURL"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#request_url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestURL"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#request_url = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequestUrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#request_url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requestURL"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requestURL");
                            }
                        }
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
                            }
                        }
                        Field::Response => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("response"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#response.is_some() {
                                    return Err(serde::de::Error::duplicate_field("response"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#response = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResponsePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_response"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("response");
                            }
                        }
                        Field::ResponseCode => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response_code.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responseCode"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#response_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responseCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#response_code = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResponseCodePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#response_code.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_responseCode"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("responseCode");
                            }
                        }
                        Field::SourceId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#source_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#source_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SourceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#source_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sourceId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sourceId");
                            }
                        }
                        Field::StopTestOnFail => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#stop_test_on_fail.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "stopTestOnFail",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#stop_test_on_fail.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "stopTestOnFail",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#stop_test_on_fail =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StopTestOnFailPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#stop_test_on_fail.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_stopTestOnFail",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("stopTestOnFail");
                            }
                        }
                        Field::ValidateProfileId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#validate_profile_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validateProfileId",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#validate_profile_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validateProfileId",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#validate_profile_id =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValidateProfileIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#validate_profile_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_validateProfileId",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("validateProfileId");
                            }
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::WarningOnly => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#warning_only.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("warningOnly"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#warning_only.is_some() {
                                    return Err(serde::de::Error::duplicate_field("warningOnly"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#warning_only = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WarningOnlyPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#warning_only.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_warningOnly"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("warningOnly");
                            }
                        }
                        Field::Requirement => {
                            if self.0.from == crate::context::Format::Json {
                                if r#requirement.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirement"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: TestScriptSetupActionAssertRequirement >> = self . 0 . transmute () ;
                                r#requirement = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#requirement.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: TestScriptSetupActionAssertRequirement > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(TestScriptSetupActionAssert {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#label,
                    r#description,
                    r#direction,
                    r#compare_to_source_id,
                    r#compare_to_source_expression,
                    r#compare_to_source_path,
                    r#content_type,
                    r#default_manual_completion,
                    r#expression,
                    r#header_field,
                    r#minimum_id,
                    r#navigation_links,
                    r#operator,
                    r#path,
                    r#request_method,
                    r#request_url,
                    r#resource,
                    r#response,
                    r#response_code,
                    r#source_id,
                    r#stop_test_on_fail: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#stop_test_on_fail.unwrap_or(Default::default())
                    } else {
                        r#stop_test_on_fail
                            .ok_or(serde::de::Error::missing_field("stopTestOnFail"))?
                    },
                    r#validate_profile_id,
                    r#value,
                    r#warning_only: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#warning_only.unwrap_or(Default::default())
                    } else {
                        r#warning_only.ok_or(serde::de::Error::missing_field("warningOnly"))?
                    },
                    r#requirement: r#requirement.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptSetupActionAssert>>
{
    type Value = Box<TestScriptSetupActionAssert>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetupActionAssert>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptSetupActionAssert>>
{
    type Value = Vec<TestScriptSetupActionAssert>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptSetupActionAssert>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetupActionAssert>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptSetupActionAssert> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetupAction;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetupAction> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup.action", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#operation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("operation", ctx))?;
        }
        if let Some(some) = self.value.r#assert.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("assert", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptSetupAction>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptSetupAction>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptSetupAction> {
    type Value = TestScriptSetupAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptSetupAction>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetupAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetupAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptSetupAction, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "operation")]
                    Operation,
                    #[serde(rename = "assert")]
                    Assert,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "operation",
                            "assert",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#operation: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                > = None;
                let mut r#assert: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionAssert,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Operation => {
                            if r#operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                            > = self.0.transmute();
                            r#operation = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Assert => {
                            if r#assert.is_some() {
                                return Err(serde::de::Error::duplicate_field("assert"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetupActionAssert,
                            > = self.0.transmute();
                            r#assert = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptSetupAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#operation,
                    r#assert,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptSetupAction>>
{
    type Value = Box<TestScriptSetupAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetupAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptSetupAction>>
{
    type Value = Vec<TestScriptSetupAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptSetupAction>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetupAction>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptSetupAction> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptSetup;
impl serde::ser::Serialize for SerializationContext<&TestScriptSetup> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.setup", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#action.is_empty() {
            self.with_context(&self.value.r#action, |ctx| {
                state.serialize_entry("action", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptSetup>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptSetup>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptSetup> {
    type Value = TestScriptSetup;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptSetup>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptSetup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptSetup")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptSetup, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "action")]
                    Action,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "action"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#action: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptSetupAction>,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Action => {
                            if self.0.from == crate::context::Format::Json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptSetupAction>,
                                > = self.0.transmute();
                                r#action = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptSetupAction,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(TestScriptSetup {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptSetup>> {
    type Value = Box<TestScriptSetup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptSetup>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptSetup>> {
    type Value = Vec<TestScriptSetup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptSetup>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptSetup>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptSetup> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptTestAction;
impl serde::ser::Serialize for SerializationContext<&TestScriptTestAction> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.test.action", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#operation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("operation", ctx))?;
        }
        if let Some(some) = self.value.r#assert.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("assert", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptTestAction>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptTestAction>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptTestAction> {
    type Value = TestScriptTestAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptTestAction>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptTestAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTestAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTestAction, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "operation")]
                    Operation,
                    #[serde(rename = "assert")]
                    Assert,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "operation",
                            "assert",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#operation: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                > = None;
                let mut r#assert: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionAssert,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Operation => {
                            if r#operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                            > = self.0.transmute();
                            r#operation = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Assert => {
                            if r#assert.is_some() {
                                return Err(serde::de::Error::duplicate_field("assert"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetupActionAssert,
                            > = self.0.transmute();
                            r#assert = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptTestAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#operation,
                    r#assert,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptTestAction>>
{
    type Value = Box<TestScriptTestAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptTestAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptTestAction>>
{
    type Value = Vec<TestScriptTestAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptTestAction>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptTestAction>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptTestAction> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptTest;
impl serde::ser::Serialize for SerializationContext<&TestScriptTest> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.test", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("name", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if !self.value.r#action.is_empty() {
            self.with_context(&self.value.r#action, |ctx| {
                state.serialize_entry("action", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptTest>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptTest>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptTest> {
    type Value = TestScriptTest;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptTest>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptTest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTest, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "action")]
                    Action,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "name",
                            "description",
                            "action",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#action: Option<Vec<fhirbolt_model::r5::resources::TestScriptTestAction>> =
                    None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Action => {
                            if self.0.from == crate::context::Format::Json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptTestAction>,
                                > = self.0.transmute();
                                r#action = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptTestAction,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(TestScriptTest {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name,
                    r#description,
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptTest>> {
    type Value = Box<TestScriptTest>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptTest>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptTest>> {
    type Value = Vec<TestScriptTest>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptTest>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptTest>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptTest> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptTeardownAction;
impl serde::ser::Serialize for SerializationContext<&TestScriptTeardownAction> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.teardown.action", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.value.r#operation.id.as_deref() == Some("$invalid") {
            return missing_field_error("operation");
        } else {
            self.with_context(&self.value.r#operation, |ctx| {
                state.serialize_entry("operation", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptTeardownAction>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptTeardownAction>> {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<TestScriptTeardownAction>
{
    type Value = TestScriptTeardownAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptTeardownAction>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptTeardownAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTeardownAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTeardownAction, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "operation")]
                    Operation,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "operation"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#operation: Option<
                    fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Operation => {
                            if r#operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetupActionOperation,
                            > = self.0.transmute();
                            r#operation = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScriptTeardownAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#operation: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#operation.unwrap_or(Default::default())
                    } else {
                        r#operation.ok_or(serde::de::Error::missing_field("operation"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<TestScriptTeardownAction>>
{
    type Value = Box<TestScriptTeardownAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptTeardownAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<TestScriptTeardownAction>>
{
    type Value = Vec<TestScriptTeardownAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptTeardownAction>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptTeardownAction>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptTeardownAction> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScriptTeardown;
impl serde::ser::Serialize for SerializationContext<&TestScriptTeardown> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript.teardown", field
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#action.is_empty() {
            self.with_context(&self.value.r#action, |ctx| {
                state.serialize_entry("action", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScriptTeardown>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScriptTeardown>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScriptTeardown> {
    type Value = TestScriptTeardown;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScriptTeardown>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScriptTeardown;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScriptTeardown")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScriptTeardown, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "action")]
                    Action,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "action"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#action: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptTeardownAction>,
                > = None;
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Action => {
                            if self.0.from == crate::context::Format::Json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptTeardownAction>,
                                > = self.0.transmute();
                                r#action = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptTeardownAction,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(TestScriptTeardown {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScriptTeardown>> {
    type Value = Box<TestScriptTeardown>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScriptTeardown>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScriptTeardown>> {
    type Value = Vec<TestScriptTeardown>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScriptTeardown>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScriptTeardown>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScriptTeardown> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::TestScript;
impl crate::Resource for TestScript {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&TestScript> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "TestScript", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "TestScript")?;
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
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
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
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
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("version", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#version.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::TestScriptVersionAlgorithm as _Enum;
            if let Some(some) = self.value.r#version_algorithm.as_ref() {
                match some {
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("versionAlgorithmString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_versionAlgorithmString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("versionAlgorithmString", ctx)
                            })?;
                        }
                    }
                    _Enum::Coding(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("versionAlgorithmCoding", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("version_algorithm is invalid"))
                    }
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref().map(Ok) {
                state.serialize_entry("name", &some?)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else if self.value.r#name.id.as_deref() == Some("$invalid") {
            return missing_field_error("name");
        } else {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
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
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                state.serialize_entry("status", &some?)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#experimental.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("experimental", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_experimental", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#experimental.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("experimental", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("date", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_date", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("date", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#publisher.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("publisher", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_publisher", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#publisher.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if !self.value.r#use_context.is_empty() {
            self.with_context(&self.value.r#use_context, |ctx| {
                state.serialize_entry("useContext", ctx)
            })?;
        }
        if !self.value.r#jurisdiction.is_empty() {
            self.with_context(&self.value.r#jurisdiction, |ctx| {
                state.serialize_entry("jurisdiction", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#purpose.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("purpose", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_purpose", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#purpose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("purpose", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#copyright.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("copyright", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_copyright", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#copyright.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("copyright", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#copyright_label.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("copyrightLabel", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_copyrightLabel", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#copyright_label.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("copyrightLabel", ctx))?;
        }
        if !self.value.r#origin.is_empty() {
            self.with_context(&self.value.r#origin, |ctx| {
                state.serialize_entry("origin", ctx)
            })?;
        }
        if !self.value.r#destination.is_empty() {
            self.with_context(&self.value.r#destination, |ctx| {
                state.serialize_entry("destination", ctx)
            })?;
        }
        if let Some(some) = self.value.r#metadata.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("metadata", ctx))?;
        }
        if !self.value.r#scope.is_empty() {
            self.with_context(&self.value.r#scope, |ctx| {
                state.serialize_entry("scope", ctx)
            })?;
        }
        if !self.value.r#fixture.is_empty() {
            self.with_context(&self.value.r#fixture, |ctx| {
                state.serialize_entry("fixture", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#profile.is_empty() {
                let values = self
                    .value
                    .r#profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("profile", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#profile
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#profile
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_profile", ctx)
                    })?;
                }
            }
        } else if !self.value.r#profile.is_empty() {
            self.with_context(&self.value.r#profile, |ctx| {
                state.serialize_entry("profile", ctx)
            })?;
        }
        if !self.value.r#variable.is_empty() {
            self.with_context(&self.value.r#variable, |ctx| {
                state.serialize_entry("variable", ctx)
            })?;
        }
        if let Some(some) = self.value.r#setup.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("setup", ctx))?;
        }
        if !self.value.r#test.is_empty() {
            self.with_context(&self.value.r#test, |ctx| state.serialize_entry("test", ctx))?;
        }
        if let Some(some) = self.value.r#teardown.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("teardown", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<TestScript>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<TestScript>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<TestScript> {
    type Value = TestScript;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<TestScript> {
    type Value = TestScript;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<TestScript>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = TestScript;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TestScript")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TestScript, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "resourceType")]
                    ResourceType,
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                    #[serde(rename = "versionAlgorithmString")]
                    VersionAlgorithmString,
                    #[serde(rename = "_versionAlgorithmString")]
                    VersionAlgorithmStringPrimitiveElement,
                    #[serde(rename = "versionAlgorithmCoding")]
                    VersionAlgorithmCoding,
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
                    #[serde(rename = "copyrightLabel")]
                    CopyrightLabel,
                    #[serde(rename = "_copyrightLabel")]
                    CopyrightLabelPrimitiveElement,
                    #[serde(rename = "origin")]
                    Origin,
                    #[serde(rename = "destination")]
                    Destination,
                    #[serde(rename = "metadata")]
                    Metadata,
                    #[serde(rename = "scope")]
                    Scope,
                    #[serde(rename = "fixture")]
                    Fixture,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "_profile")]
                    ProfilePrimitiveElement,
                    #[serde(rename = "variable")]
                    Variable,
                    #[serde(rename = "setup")]
                    Setup,
                    #[serde(rename = "test")]
                    Test,
                    #[serde(rename = "teardown")]
                    Teardown,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                            "versionAlgorithmString",
                            "versionAlgorithmCoding",
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
                            "copyrightLabel",
                            "origin",
                            "destination",
                            "metadata",
                            "scope",
                            "fixture",
                            "profile",
                            "variable",
                            "setup",
                            "test",
                            "teardown",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#url: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#version: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#version_algorithm: Option<
                    fhirbolt_model::r5::resources::TestScriptVersionAlgorithm,
                > = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#experimental: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#publisher: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r5::types::ContactDetail>> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#use_context: Option<Vec<fhirbolt_model::r5::types::UsageContext>> = None;
                let mut r#jurisdiction: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#purpose: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#copyright: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#copyright_label: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#origin: Option<Vec<fhirbolt_model::r5::resources::TestScriptOrigin>> =
                    None;
                let mut r#destination: Option<
                    Vec<fhirbolt_model::r5::resources::TestScriptDestination>,
                > = None;
                let mut r#metadata: Option<fhirbolt_model::r5::resources::TestScriptMetadata> =
                    None;
                let mut r#scope: Option<Vec<fhirbolt_model::r5::resources::TestScriptScope>> = None;
                let mut r#fixture: Option<Vec<fhirbolt_model::r5::resources::TestScriptFixture>> =
                    None;
                let mut r#profile: Option<Vec<fhirbolt_model::r5::types::Canonical>> = None;
                let mut r#variable: Option<Vec<fhirbolt_model::r5::resources::TestScriptVariable>> =
                    None;
                let mut r#setup: Option<fhirbolt_model::r5::resources::TestScriptSetup> = None;
                let mut r#test: Option<Vec<fhirbolt_model::r5::resources::TestScriptTest>> = None;
                let mut r#teardown: Option<fhirbolt_model::r5::resources::TestScriptTeardown> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "TestScript" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"TestScript",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
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
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
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
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    fhirbolt_model::r5::types::Uri,
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
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#version = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::VersionAlgorithmString => {
                            use fhirbolt_model::r5::resources::TestScriptVersionAlgorithm as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#version_algorithm
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versionAlgorithmString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "versionAlgorithm[x]",
                                    ));
                                }
                            } else {
                                if r#version_algorithm.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "versionAlgorithmString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::String>,
                                > = self.0.transmute();
                                r#version_algorithm = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::VersionAlgorithmStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::TestScriptVersionAlgorithm as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#version_algorithm
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_versionAlgorithmString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_versionAlgorithm[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("versionAlgorithmString");
                            }
                        }
                        Field::VersionAlgorithmCoding => {
                            use fhirbolt_model::r5::resources::TestScriptVersionAlgorithm as _Enum;
                            if r#version_algorithm.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "versionAlgorithmCoding",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#version_algorithm =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
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
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Experimental => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#experimental.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#experimental = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExperimentalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_experimental"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("experimental");
                            }
                        }
                        Field::Date => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("date");
                            }
                        }
                        Field::Publisher => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#publisher = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PublisherPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publisher");
                            }
                        }
                        Field::Contact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::ContactDetail>,
                                > = self.0.transmute();
                                r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::UseContext => {
                            if self.0.from == crate::context::Format::Json {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::UsageContext>,
                                > = self.0.transmute();
                                r#use_context = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::UsageContext,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from == crate::context::Format::Json {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#jurisdiction = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#jurisdiction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Purpose => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#purpose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#purpose = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PurposePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Copyright => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#copyright.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#copyright = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CopyrightPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyright");
                            }
                        }
                        Field::CopyrightLabel => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright_label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "copyrightLabel",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#copyright_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "copyrightLabel",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#copyright_label =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CopyrightLabelPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright_label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_copyrightLabel",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyrightLabel");
                            }
                        }
                        Field::Origin => {
                            if self.0.from == crate::context::Format::Json {
                                if r#origin.is_some() {
                                    return Err(serde::de::Error::duplicate_field("origin"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptOrigin>,
                                > = self.0.transmute();
                                r#origin = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#origin.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptOrigin,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Destination => {
                            if self.0.from == crate::context::Format::Json {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptDestination>,
                                > = self.0.transmute();
                                r#destination = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#destination.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptDestination,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Metadata => {
                            if r#metadata.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptMetadata,
                            > = self.0.transmute();
                            r#metadata = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Scope => {
                            if self.0.from == crate::context::Format::Json {
                                if r#scope.is_some() {
                                    return Err(serde::de::Error::duplicate_field("scope"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptScope>,
                                > = self.0.transmute();
                                r#scope = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#scope.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptScope,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Fixture => {
                            if self.0.from == crate::context::Format::Json {
                                if r#fixture.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixture"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptFixture>,
                                > = self.0.transmute();
                                r#fixture = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#fixture.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptFixture,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Profile => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#profile.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#profile.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#profile.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_profile"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("profile");
                            }
                        }
                        Field::Variable => {
                            if self.0.from == crate::context::Format::Json {
                                if r#variable.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptVariable>,
                                > = self.0.transmute();
                                r#variable = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#variable.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptVariable,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Setup => {
                            if r#setup.is_some() {
                                return Err(serde::de::Error::duplicate_field("setup"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptSetup,
                            > = self.0.transmute();
                            r#setup = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Test => {
                            if self.0.from == crate::context::Format::Json {
                                if r#test.is_some() {
                                    return Err(serde::de::Error::duplicate_field("test"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::TestScriptTest>,
                                > = self.0.transmute();
                                r#test = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#test.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::TestScriptTest,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Teardown => {
                            if r#teardown.is_some() {
                                return Err(serde::de::Error::duplicate_field("teardown"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::TestScriptTeardown,
                            > = self.0.transmute();
                            r#teardown = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(TestScript {
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
                    r#version_algorithm,
                    r#name: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#name.unwrap_or(Default::default())
                    } else {
                        r#name.ok_or(serde::de::Error::missing_field("name"))?
                    },
                    r#title,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
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
                    r#copyright_label,
                    r#origin: r#origin.unwrap_or(vec![]),
                    r#destination: r#destination.unwrap_or(vec![]),
                    r#metadata,
                    r#scope: r#scope.unwrap_or(vec![]),
                    r#fixture: r#fixture.unwrap_or(vec![]),
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#variable: r#variable.unwrap_or(vec![]),
                    r#setup,
                    r#test: r#test.unwrap_or(vec![]),
                    r#teardown,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<TestScript>> {
    type Value = Box<TestScript>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<TestScript>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<TestScript>> {
    type Value = Vec<TestScript>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<TestScript>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<TestScript>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<TestScript> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
