// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapStructure,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.structure", field
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
        if self.output_json {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            if let Some(some) = self.value.r#url.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("url", &some)?;
            }
            if self.value.r#url.id.is_some() || !self.value.r#url.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#url.id.as_ref(),
                    extension: &self.value.r#url.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_url", ctx))?;
            }
        } else {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            self.with_context(&self.value.r#url, |ctx| state.serialize_entry("url", ctx))?;
        }
        if self.output_json {
            if self.value.r#mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("mode");
            }
            if let Some(some) = self.value.r#mode.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("mode", &some)?;
            }
            if self.value.r#mode.id.is_some() || !self.value.r#mode.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#mode.id.as_ref(),
                    extension: &self.value.r#mode.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_mode", ctx)
                })?;
            }
        } else {
            if self.value.r#mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("mode");
            }
            self.with_context(&self.value.r#mode, |ctx| state.serialize_entry("mode", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#alias.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("alias", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_alias", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#alias.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("alias", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#documentation.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("documentation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_documentation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#documentation.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("documentation", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapStructure>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapStructure>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapStructure>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapStructure,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapStructure;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapStructure,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapStructure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapStructure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapStructure, V::Error>
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
                    #[serde(rename = "mode")]
                    Mode,
                    #[serde(rename = "_mode")]
                    ModePrimitiveElement,
                    #[serde(rename = "alias")]
                    Alias,
                    #[serde(rename = "_alias")]
                    AliasPrimitiveElement,
                    #[serde(rename = "documentation")]
                    Documentation,
                    #[serde(rename = "_documentation")]
                    DocumentationPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "url",
                            "mode",
                            "alias",
                            "documentation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#url: Option<fhirbolt_model::r4::types::Canonical> = None;
                let mut r#mode: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#alias: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#documentation: Option<fhirbolt_model::r4::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Url => {
                            if self.0.from_json {
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
                                r#url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Mode => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                r#mode = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::ModePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("mode");
                            }
                        }
                        Field::Alias => {
                            if self.0.from_json {
                                let some = r#alias.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#alias.is_some() {
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                r#alias = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::AliasPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#alias.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_alias"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("alias");
                            }
                        }
                        Field::Documentation => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::DocumentationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("documentation");
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
                Ok(fhirbolt_model::r4::resources::StructureMapStructure {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#url.unwrap_or(Default::default())
                    } else {
                        r#url.ok_or(serde::de::Error::missing_field("url"))?
                    },
                    r#mode: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#mode.unwrap_or(Default::default())
                    } else {
                        r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                    },
                    r#alias,
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapStructure>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapStructure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapStructure>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapStructure>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapStructure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapStructure>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapStructure>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapStructure>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapStructure>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapStructure>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapStructure>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapStructure>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::StructureMapStructure>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupInput,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.input", field
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
        if self.output_json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_type", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#type.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("mode");
            }
            if let Some(some) = self.value.r#mode.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("mode", &some)?;
            }
            if self.value.r#mode.id.is_some() || !self.value.r#mode.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#mode.id.as_ref(),
                    extension: &self.value.r#mode.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_mode", ctx)
                })?;
            }
        } else {
            if self.value.r#mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("mode");
            }
            self.with_context(&self.value.r#mode, |ctx| state.serialize_entry("mode", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#documentation.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("documentation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_documentation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#documentation.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("documentation", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupInput>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupInput,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupInput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupInput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupInput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupInput, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "mode")]
                    Mode,
                    #[serde(rename = "_mode")]
                    ModePrimitiveElement,
                    #[serde(rename = "documentation")]
                    Documentation,
                    #[serde(rename = "_documentation")]
                    DocumentationPrimitiveElement,
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
                            "type",
                            "mode",
                            "documentation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#mode: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#documentation: Option<fhirbolt_model::r4::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
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
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Mode => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                r#mode = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::ModePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("mode");
                            }
                        }
                        Field::Documentation => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::DocumentationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("documentation");
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
                Ok(fhirbolt_model::r4::resources::StructureMapGroupInput {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#name.unwrap_or(Default::default())
                    } else {
                        r#name.ok_or(serde::de::Error::missing_field("name"))?
                    },
                    r#type,
                    r#mode: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#mode.unwrap_or(Default::default())
                    } else {
                        r#mode.ok_or(serde::de::Error::missing_field("mode"))?
                    },
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupInput>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupInput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroupInput>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::StructureMapGroupInput>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupRuleSource,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.rule.source", field
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
        if self.output_json {
            if self.value.r#context.id.as_deref() == Some("$invalid") {
                return missing_field_error("context");
            }
            if let Some(some) = self.value.r#context.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("context", &some)?;
            }
            if self.value.r#context.id.is_some() || !self.value.r#context.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#context.id.as_ref(),
                    extension: &self.value.r#context.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_context", ctx)
                })?;
            }
        } else {
            if self.value.r#context.id.as_deref() == Some("$invalid") {
                return missing_field_error("context");
            }
            self.with_context(&self.value.r#context, |ctx| {
                state.serialize_entry("context", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#min.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("min", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_min", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#min.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("min", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#max.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("max", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_max", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#max.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("max", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_type", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#type.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
            }
        }
        if let Some(some) = self.value.r#default_value.as_ref() {
            match some { fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueBase64Binary" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueBase64Binary" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueBase64Binary" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueBoolean" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueBoolean" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueBoolean" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueCanonical" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueCanonical" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueCanonical" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueCode" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueCode" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueCode" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueDate" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueDate" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDate" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueDateTime" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueDateTime" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDateTime" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = some . parse :: < serde_json :: Number > () . map_err (| _ | serde :: ser :: Error :: custom ("error serializing decimal")) ? ; state . serialize_entry ("defaultValueDecimal" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueDecimal" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDecimal" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueId" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueId" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueId" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueInstant" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueInstant" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueInstant" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueInteger" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueInteger" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueInteger" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueMarkdown" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueMarkdown" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueMarkdown" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueOid" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueOid" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueOid" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValuePositiveInt" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValuePositiveInt" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValuePositiveInt" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueString" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueString" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueString" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueTime" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueTime" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueTime" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueUnsignedInt" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueUnsignedInt" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueUnsignedInt" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueUri" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueUri" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueUri" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueUrl" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueUrl" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueUrl" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("defaultValueUuid" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_defaultValueUuid" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("defaultValueUuid" , ctx)) ? ; } } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Address (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueAddress" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Age (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueAge" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Annotation (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueAnnotation" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Attachment (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueAttachment" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Coding (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueCoding" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactPoint (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueContactPoint" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Count (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueCount" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Distance (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDistance" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Duration (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDuration" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: HumanName (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueHumanName" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Identifier (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueIdentifier" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Money (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueMoney" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Period (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValuePeriod" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueQuantity" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Range (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueRange" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Ratio (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueRatio" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Reference (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueReference" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: SampledData (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueSampledData" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Signature (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueSignature" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Timing (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueTiming" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactDetail (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueContactDetail" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Contributor (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueContributor" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DataRequirement (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDataRequirement" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Expression (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueExpression" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ParameterDefinition (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueParameterDefinition" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: RelatedArtifact (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueRelatedArtifact" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: TriggerDefinition (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueTriggerDefinition" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UsageContext (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueUsageContext" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Dosage (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueDosage" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Meta (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("defaultValueMeta" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("default_value is invalid")) } }
        }
        if self.output_json {
            if let Some(some) = self.value.r#element.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("element", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_element", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#element.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("element", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#list_mode.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("listMode", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_listMode", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#list_mode.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("listMode", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#variable.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("variable", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_variable", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#variable.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("variable", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#condition.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("condition", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_condition", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#condition.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("condition", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#check.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("check", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_check", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#check.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("check", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#log_message.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("logMessage", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_logMessage", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#log_message.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("logMessage", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupRuleSource,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleSource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupRuleSource,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleSource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupRuleSource, V::Error>
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
                    #[serde(rename = "context")]
                    Context,
                    #[serde(rename = "_context")]
                    ContextPrimitiveElement,
                    #[serde(rename = "min")]
                    Min,
                    #[serde(rename = "_min")]
                    MinPrimitiveElement,
                    #[serde(rename = "max")]
                    Max,
                    #[serde(rename = "_max")]
                    MaxPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "defaultValueBase64Binary")]
                    DefaultValueBase64Binary,
                    #[serde(rename = "_defaultValueBase64Binary")]
                    DefaultValueBase64BinaryPrimitiveElement,
                    #[serde(rename = "defaultValueBoolean")]
                    DefaultValueBoolean,
                    #[serde(rename = "_defaultValueBoolean")]
                    DefaultValueBooleanPrimitiveElement,
                    #[serde(rename = "defaultValueCanonical")]
                    DefaultValueCanonical,
                    #[serde(rename = "_defaultValueCanonical")]
                    DefaultValueCanonicalPrimitiveElement,
                    #[serde(rename = "defaultValueCode")]
                    DefaultValueCode,
                    #[serde(rename = "_defaultValueCode")]
                    DefaultValueCodePrimitiveElement,
                    #[serde(rename = "defaultValueDate")]
                    DefaultValueDate,
                    #[serde(rename = "_defaultValueDate")]
                    DefaultValueDatePrimitiveElement,
                    #[serde(rename = "defaultValueDateTime")]
                    DefaultValueDateTime,
                    #[serde(rename = "_defaultValueDateTime")]
                    DefaultValueDateTimePrimitiveElement,
                    #[serde(rename = "defaultValueDecimal")]
                    DefaultValueDecimal,
                    #[serde(rename = "_defaultValueDecimal")]
                    DefaultValueDecimalPrimitiveElement,
                    #[serde(rename = "defaultValueId")]
                    DefaultValueId,
                    #[serde(rename = "_defaultValueId")]
                    DefaultValueIdPrimitiveElement,
                    #[serde(rename = "defaultValueInstant")]
                    DefaultValueInstant,
                    #[serde(rename = "_defaultValueInstant")]
                    DefaultValueInstantPrimitiveElement,
                    #[serde(rename = "defaultValueInteger")]
                    DefaultValueInteger,
                    #[serde(rename = "_defaultValueInteger")]
                    DefaultValueIntegerPrimitiveElement,
                    #[serde(rename = "defaultValueMarkdown")]
                    DefaultValueMarkdown,
                    #[serde(rename = "_defaultValueMarkdown")]
                    DefaultValueMarkdownPrimitiveElement,
                    #[serde(rename = "defaultValueOid")]
                    DefaultValueOid,
                    #[serde(rename = "_defaultValueOid")]
                    DefaultValueOidPrimitiveElement,
                    #[serde(rename = "defaultValuePositiveInt")]
                    DefaultValuePositiveInt,
                    #[serde(rename = "_defaultValuePositiveInt")]
                    DefaultValuePositiveIntPrimitiveElement,
                    #[serde(rename = "defaultValueString")]
                    DefaultValueString,
                    #[serde(rename = "_defaultValueString")]
                    DefaultValueStringPrimitiveElement,
                    #[serde(rename = "defaultValueTime")]
                    DefaultValueTime,
                    #[serde(rename = "_defaultValueTime")]
                    DefaultValueTimePrimitiveElement,
                    #[serde(rename = "defaultValueUnsignedInt")]
                    DefaultValueUnsignedInt,
                    #[serde(rename = "_defaultValueUnsignedInt")]
                    DefaultValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "defaultValueUri")]
                    DefaultValueUri,
                    #[serde(rename = "_defaultValueUri")]
                    DefaultValueUriPrimitiveElement,
                    #[serde(rename = "defaultValueUrl")]
                    DefaultValueUrl,
                    #[serde(rename = "_defaultValueUrl")]
                    DefaultValueUrlPrimitiveElement,
                    #[serde(rename = "defaultValueUuid")]
                    DefaultValueUuid,
                    #[serde(rename = "_defaultValueUuid")]
                    DefaultValueUuidPrimitiveElement,
                    #[serde(rename = "defaultValueAddress")]
                    DefaultValueAddress,
                    #[serde(rename = "defaultValueAge")]
                    DefaultValueAge,
                    #[serde(rename = "defaultValueAnnotation")]
                    DefaultValueAnnotation,
                    #[serde(rename = "defaultValueAttachment")]
                    DefaultValueAttachment,
                    #[serde(rename = "defaultValueCodeableConcept")]
                    DefaultValueCodeableConcept,
                    #[serde(rename = "defaultValueCoding")]
                    DefaultValueCoding,
                    #[serde(rename = "defaultValueContactPoint")]
                    DefaultValueContactPoint,
                    #[serde(rename = "defaultValueCount")]
                    DefaultValueCount,
                    #[serde(rename = "defaultValueDistance")]
                    DefaultValueDistance,
                    #[serde(rename = "defaultValueDuration")]
                    DefaultValueDuration,
                    #[serde(rename = "defaultValueHumanName")]
                    DefaultValueHumanName,
                    #[serde(rename = "defaultValueIdentifier")]
                    DefaultValueIdentifier,
                    #[serde(rename = "defaultValueMoney")]
                    DefaultValueMoney,
                    #[serde(rename = "defaultValuePeriod")]
                    DefaultValuePeriod,
                    #[serde(rename = "defaultValueQuantity")]
                    DefaultValueQuantity,
                    #[serde(rename = "defaultValueRange")]
                    DefaultValueRange,
                    #[serde(rename = "defaultValueRatio")]
                    DefaultValueRatio,
                    #[serde(rename = "defaultValueReference")]
                    DefaultValueReference,
                    #[serde(rename = "defaultValueSampledData")]
                    DefaultValueSampledData,
                    #[serde(rename = "defaultValueSignature")]
                    DefaultValueSignature,
                    #[serde(rename = "defaultValueTiming")]
                    DefaultValueTiming,
                    #[serde(rename = "defaultValueContactDetail")]
                    DefaultValueContactDetail,
                    #[serde(rename = "defaultValueContributor")]
                    DefaultValueContributor,
                    #[serde(rename = "defaultValueDataRequirement")]
                    DefaultValueDataRequirement,
                    #[serde(rename = "defaultValueExpression")]
                    DefaultValueExpression,
                    #[serde(rename = "defaultValueParameterDefinition")]
                    DefaultValueParameterDefinition,
                    #[serde(rename = "defaultValueRelatedArtifact")]
                    DefaultValueRelatedArtifact,
                    #[serde(rename = "defaultValueTriggerDefinition")]
                    DefaultValueTriggerDefinition,
                    #[serde(rename = "defaultValueUsageContext")]
                    DefaultValueUsageContext,
                    #[serde(rename = "defaultValueDosage")]
                    DefaultValueDosage,
                    #[serde(rename = "defaultValueMeta")]
                    DefaultValueMeta,
                    #[serde(rename = "element")]
                    Element,
                    #[serde(rename = "_element")]
                    ElementPrimitiveElement,
                    #[serde(rename = "listMode")]
                    ListMode,
                    #[serde(rename = "_listMode")]
                    ListModePrimitiveElement,
                    #[serde(rename = "variable")]
                    Variable,
                    #[serde(rename = "_variable")]
                    VariablePrimitiveElement,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "_condition")]
                    ConditionPrimitiveElement,
                    #[serde(rename = "check")]
                    Check,
                    #[serde(rename = "_check")]
                    CheckPrimitiveElement,
                    #[serde(rename = "logMessage")]
                    LogMessage,
                    #[serde(rename = "_logMessage")]
                    LogMessagePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "context",
                            "min",
                            "max",
                            "type",
                            "defaultValueBase64Binary",
                            "defaultValueBoolean",
                            "defaultValueCanonical",
                            "defaultValueCode",
                            "defaultValueDate",
                            "defaultValueDateTime",
                            "defaultValueDecimal",
                            "defaultValueId",
                            "defaultValueInstant",
                            "defaultValueInteger",
                            "defaultValueMarkdown",
                            "defaultValueOid",
                            "defaultValuePositiveInt",
                            "defaultValueString",
                            "defaultValueTime",
                            "defaultValueUnsignedInt",
                            "defaultValueUri",
                            "defaultValueUrl",
                            "defaultValueUuid",
                            "defaultValueAddress",
                            "defaultValueAge",
                            "defaultValueAnnotation",
                            "defaultValueAttachment",
                            "defaultValueCodeableConcept",
                            "defaultValueCoding",
                            "defaultValueContactPoint",
                            "defaultValueCount",
                            "defaultValueDistance",
                            "defaultValueDuration",
                            "defaultValueHumanName",
                            "defaultValueIdentifier",
                            "defaultValueMoney",
                            "defaultValuePeriod",
                            "defaultValueQuantity",
                            "defaultValueRange",
                            "defaultValueRatio",
                            "defaultValueReference",
                            "defaultValueSampledData",
                            "defaultValueSignature",
                            "defaultValueTiming",
                            "defaultValueContactDetail",
                            "defaultValueContributor",
                            "defaultValueDataRequirement",
                            "defaultValueExpression",
                            "defaultValueParameterDefinition",
                            "defaultValueRelatedArtifact",
                            "defaultValueTriggerDefinition",
                            "defaultValueUsageContext",
                            "defaultValueDosage",
                            "defaultValueMeta",
                            "element",
                            "listMode",
                            "variable",
                            "condition",
                            "check",
                            "logMessage",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#context: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#min: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#max: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#default_value: Option<
                    fhirbolt_model::r4::resources::StructureMapGroupRuleSourceDefaultValue,
                > = None;
                let mut r#element: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#list_mode: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#variable: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#condition: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#check: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#log_message: Option<fhirbolt_model::r4::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Context => {
                            if self.0.from_json {
                                let some = r#context.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::ContextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#context.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_context"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("context");
                            }
                        }
                        Field::Min => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#min.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                r#min = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Integer>(),
                                )?);
                            }
                        }
                        Field::MinPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_min"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("min");
                            }
                        }
                        Field::Max => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                r#max = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::MaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_max"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("max");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::DefaultValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBase64Binary",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBase64Binary");
                            }
                        }
                        Field::DefaultValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBoolean",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBoolean");
                            }
                        }
                        Field::DefaultValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCanonical",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCanonical");
                            }
                        }
                        Field::DefaultValueCode => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCode")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCode",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Code > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCode")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCode");
                            }
                        }
                        Field::DefaultValueDate => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDate",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDate");
                            }
                        }
                        Field::DefaultValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDateTime",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDateTime");
                            }
                        }
                        Field::DefaultValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDecimal",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDecimal");
                            }
                        }
                        Field::DefaultValueId => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueId",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Id > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueId");
                            }
                        }
                        Field::DefaultValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInstant",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInstant");
                            }
                        }
                        Field::DefaultValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInteger",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInteger");
                            }
                        }
                        Field::DefaultValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMarkdown",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueMarkdown");
                            }
                        }
                        Field::DefaultValueOid => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueOid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueOid",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Oid > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueOid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueOid");
                            }
                        }
                        Field::DefaultValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePositiveInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValuePositiveInt");
                            }
                        }
                        Field::DefaultValueString => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueString",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueString");
                            }
                        }
                        Field::DefaultValueTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTime",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Time > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueTime");
                            }
                        }
                        Field::DefaultValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUnsignedInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUnsignedInt");
                            }
                        }
                        Field::DefaultValueUri => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUri",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUri");
                            }
                        }
                        Field::DefaultValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUrl")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUrl",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Url > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUrl")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUrl");
                            }
                        }
                        Field::DefaultValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUuid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUuid",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Uuid > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUuid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUuid");
                            }
                        }
                        Field::DefaultValueAddress => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAddress",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Address (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Address > > ()) ?)) ;
                        }
                        Field::DefaultValueAge => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueAge"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Age (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Age > > ()) ?)) ;
                        }
                        Field::DefaultValueAnnotation => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAnnotation",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::DefaultValueAttachment => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAttachment",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::DefaultValueCodeableConcept => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableConcept",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::DefaultValueCoding => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCoding",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Coding (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Coding > > ()) ?)) ;
                        }
                        Field::DefaultValueContactPoint => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactPoint",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::DefaultValueCount => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueCount"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Count (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Count > > ()) ?)) ;
                        }
                        Field::DefaultValueDistance => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDistance",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Distance (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Distance > > ()) ?)) ;
                        }
                        Field::DefaultValueDuration => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDuration",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Duration (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Duration > > ()) ?)) ;
                        }
                        Field::DefaultValueHumanName => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueHumanName",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::DefaultValueIdentifier => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueIdentifier",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::DefaultValueMoney => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMoney"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Money (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Money > > ()) ?)) ;
                        }
                        Field::DefaultValuePeriod => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValuePeriod",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Period (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Period > > ()) ?)) ;
                        }
                        Field::DefaultValueQuantity => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueQuantity",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::DefaultValueRange => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRange"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Range > > ()) ?)) ;
                        }
                        Field::DefaultValueRatio => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRatio"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Ratio (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Ratio > > ()) ?)) ;
                        }
                        Field::DefaultValueReference => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueReference",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Reference > > ()) ?)) ;
                        }
                        Field::DefaultValueSampledData => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSampledData",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::DefaultValueSignature => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSignature",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Signature > > ()) ?)) ;
                        }
                        Field::DefaultValueTiming => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTiming",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Timing (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Timing > > ()) ?)) ;
                        }
                        Field::DefaultValueContactDetail => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactDetail",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::DefaultValueContributor => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContributor",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::DefaultValueDataRequirement => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDataRequirement",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::DefaultValueExpression => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueExpression",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Expression > > ()) ?)) ;
                        }
                        Field::DefaultValueParameterDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueParameterDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueRelatedArtifact => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRelatedArtifact",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::DefaultValueTriggerDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTriggerDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueUsageContext => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueUsageContext",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::DefaultValueDosage => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDosage",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Dosage (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Dosage > > ()) ?)) ;
                        }
                        Field::DefaultValueMeta => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMeta"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSourceDefaultValue :: Meta (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Meta > > ()) ?)) ;
                        }
                        Field::Element => {
                            if self.0.from_json {
                                let some = r#element.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#element.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                r#element = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ElementPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#element.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_element"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("element");
                            }
                        }
                        Field::ListMode => {
                            if self.0.from_json {
                                let some = r#list_mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listMode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#list_mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listMode"));
                                }
                                r#list_mode = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::ListModePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#list_mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_listMode"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("listMode");
                            }
                        }
                        Field::Variable => {
                            if self.0.from_json {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#variable.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                r#variable = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::VariablePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("variable");
                            }
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("condition");
                            }
                        }
                        Field::Check => {
                            if self.0.from_json {
                                let some = r#check.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("check"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#check.is_some() {
                                    return Err(serde::de::Error::duplicate_field("check"));
                                }
                                r#check = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::CheckPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#check.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_check"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("check");
                            }
                        }
                        Field::LogMessage => {
                            if self.0.from_json {
                                let some = r#log_message.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("logMessage"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#log_message.is_some() {
                                    return Err(serde::de::Error::duplicate_field("logMessage"));
                                }
                                r#log_message = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::LogMessagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#log_message.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_logMessage"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("logMessage");
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
                Ok(fhirbolt_model::r4::resources::StructureMapGroupRuleSource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#context: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#context.unwrap_or(Default::default())
                    } else {
                        r#context.ok_or(serde::de::Error::missing_field("context"))?
                    },
                    r#min,
                    r#max,
                    r#type,
                    r#default_value,
                    r#element,
                    r#list_mode,
                    r#variable,
                    r#condition,
                    r#check,
                    r#log_message,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSource >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.rule.target.parameter", field
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
        match self.value.r#value {
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::Id(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueId", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueId", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueId", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::String(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueString", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueString", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::Boolean(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueBoolean", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::Integer(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueInteger", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueInteger", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueInteger", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::Decimal(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("valueDecimal", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDecimal", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDecimal", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTargetParameter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter, V::Error>
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
                    #[serde(rename = "valueId")]
                    ValueId,
                    #[serde(rename = "_valueId")]
                    ValueIdPrimitiveElement,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueDecimal")]
                    ValueDecimal,
                    #[serde(rename = "_valueDecimal")]
                    ValueDecimalPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "valueId",
                            "valueString",
                            "valueBoolean",
                            "valueInteger",
                            "valueDecimal",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#value: Option<
                    fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameterValue,
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Id > > ()) ?)) ;
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDecimal");
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
                Ok(
                    fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameter > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(
                        self.0.transmute::<Box<
                            fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter,
                        >>(),
                    )?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupRuleTarget,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.rule.target", field
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
        if self.output_json {
            if let Some(some) = self.value.r#context.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("context", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_context", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#context.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("context", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#context_type.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("contextType", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_contextType", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#context_type.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("contextType", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#element.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("element", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_element", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#element.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("element", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#variable.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("variable", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_variable", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#variable.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("variable", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#list_mode.is_empty() {
                let values = self
                    .value
                    .r#list_mode
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("listMode", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#list_mode
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#list_mode
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(super::super::serde_helpers::PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_listMode", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#list_mode.is_empty() {
                self.with_context(&self.value.r#list_mode, |ctx| {
                    state.serialize_entry("listMode", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#list_rule_id.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("listRuleId", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_listRuleId", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#list_rule_id.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("listRuleId", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#transform.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("transform", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_transform", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#transform.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("transform", ctx))?;
            }
        }
        if !self.value.r#parameter.is_empty() {
            self.with_context(&self.value.r#parameter, |ctx| {
                state.serialize_entry("parameter", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupRuleTarget,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleTarget;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupRuleTarget,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTarget")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget, V::Error>
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
                    #[serde(rename = "context")]
                    Context,
                    #[serde(rename = "_context")]
                    ContextPrimitiveElement,
                    #[serde(rename = "contextType")]
                    ContextType,
                    #[serde(rename = "_contextType")]
                    ContextTypePrimitiveElement,
                    #[serde(rename = "element")]
                    Element,
                    #[serde(rename = "_element")]
                    ElementPrimitiveElement,
                    #[serde(rename = "variable")]
                    Variable,
                    #[serde(rename = "_variable")]
                    VariablePrimitiveElement,
                    #[serde(rename = "listMode")]
                    ListMode,
                    #[serde(rename = "_listMode")]
                    ListModePrimitiveElement,
                    #[serde(rename = "listRuleId")]
                    ListRuleId,
                    #[serde(rename = "_listRuleId")]
                    ListRuleIdPrimitiveElement,
                    #[serde(rename = "transform")]
                    Transform,
                    #[serde(rename = "_transform")]
                    TransformPrimitiveElement,
                    #[serde(rename = "parameter")]
                    Parameter,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "context",
                            "contextType",
                            "element",
                            "variable",
                            "listMode",
                            "listRuleId",
                            "transform",
                            "parameter",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#context: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#context_type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#element: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#variable: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#list_mode: Option<Vec<fhirbolt_model::r4::types::Code>> = None;
                let mut r#list_rule_id: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#transform: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#parameter: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTargetParameter>,
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Context => {
                            if self.0.from_json {
                                let some = r#context.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::ContextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#context.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_context"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("context");
                            }
                        }
                        Field::ContextType => {
                            if self.0.from_json {
                                let some = r#context_type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contextType"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#context_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contextType"));
                                }
                                r#context_type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::ContextTypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#context_type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_contextType"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contextType");
                            }
                        }
                        Field::Element => {
                            if self.0.from_json {
                                let some = r#element.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#element.is_some() {
                                    return Err(serde::de::Error::duplicate_field("element"));
                                }
                                r#element = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ElementPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#element.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_element"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("element");
                            }
                        }
                        Field::Variable => {
                            if self.0.from_json {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#variable.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                r#variable = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::VariablePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#variable.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("variable");
                            }
                        }
                        Field::ListMode => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#list_mode.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("listMode"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#list_mode.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::ListModePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#list_mode.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_listMode"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("listMode");
                            }
                        }
                        Field::ListRuleId => {
                            if self.0.from_json {
                                let some = r#list_rule_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listRuleId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#list_rule_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listRuleId"));
                                }
                                r#list_rule_id = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::ListRuleIdPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#list_rule_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_listRuleId"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("listRuleId");
                            }
                        }
                        Field::Transform => {
                            if self.0.from_json {
                                let some = r#transform.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("transform"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#transform.is_some() {
                                    return Err(serde::de::Error::duplicate_field("transform"));
                                }
                                r#transform = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::TransformPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#transform.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_transform"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("transform");
                            }
                        }
                        Field::Parameter => {
                            if self.0.from_json {
                                if r#parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parameter"));
                                }
                                r#parameter = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameter >> ()) ?) ;
                            } else {
                                let vec = r#parameter.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTargetParameter > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::StructureMapGroupRuleTarget {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#context,
                    r#context_type,
                    r#element,
                    r#variable,
                    r#list_mode: r#list_mode.unwrap_or(vec![]),
                    r#list_rule_id,
                    r#transform,
                    r#parameter: r#parameter.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTarget >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupRuleDependent,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.rule.dependent", field
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
        if self.output_json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if !self.value.r#variable.is_empty() {
                let values = self
                    .value
                    .r#variable
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("variable", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#variable
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#variable
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(super::super::serde_helpers::PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_variable", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#variable.is_empty() {
                self.with_context(&self.value.r#variable, |ctx| {
                    state.serialize_entry("variable", ctx)
                })?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupRuleDependent,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleDependent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupRuleDependent,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupRuleDependent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleDependent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent, V::Error>
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
                    #[serde(rename = "variable")]
                    Variable,
                    #[serde(rename = "_variable")]
                    VariablePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "name", "variable"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#variable: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
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
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Variable => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#variable.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("variable"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#variable.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::VariablePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#variable.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_variable"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("variable");
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
                Ok(
                    fhirbolt_model::r4::resources::StructureMapGroupRuleDependent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#variable: r#variable.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>(
                        ),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleDependent >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::StructureMapGroupRule,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group.rule", field
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
        if self.output_json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if !self.value.r#source.is_empty() {
            self.with_context(&self.value.r#source, |ctx| {
                state.serialize_entry("source", ctx)
            })?;
        }
        if !self.value.r#target.is_empty() {
            self.with_context(&self.value.r#target, |ctx| {
                state.serialize_entry("target", ctx)
            })?;
        }
        if !self.value.r#rule.is_empty() {
            self.with_context(&self.value.r#rule, |ctx| state.serialize_entry("rule", ctx))?;
        }
        if !self.value.r#dependent.is_empty() {
            self.with_context(&self.value.r#dependent, |ctx| {
                state.serialize_entry("dependent", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#documentation.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("documentation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_documentation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#documentation.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("documentation", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroupRule>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroupRule,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroupRule;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroupRule,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroupRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRule")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroupRule, V::Error>
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
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "target")]
                    Target,
                    #[serde(rename = "rule")]
                    Rule,
                    #[serde(rename = "dependent")]
                    Dependent,
                    #[serde(rename = "documentation")]
                    Documentation,
                    #[serde(rename = "_documentation")]
                    DocumentationPrimitiveElement,
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
                            "source",
                            "target",
                            "rule",
                            "dependent",
                            "documentation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#source: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleSource>,
                > = None;
                let mut r#target: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleTarget>,
                > = None;
                let mut r#rule: Option<Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>> =
                    None;
                let mut r#dependent: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapGroupRuleDependent>,
                > = None;
                let mut r#documentation: Option<fhirbolt_model::r4::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
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
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Source => {
                            if self.0.from_json {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::StructureMapGroupRuleSource,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#source.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleSource > ()) ?) ;
                            }
                        }
                        Field::Target => {
                            if self.0.from_json {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::StructureMapGroupRuleTarget,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#target.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleTarget > ()) ?) ;
                            }
                        }
                        Field::Rule => {
                            if self.0.from_json {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::StructureMapGroupRule,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#rule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRule > ()) ?) ;
                            }
                        }
                        Field::Dependent => {
                            if self.0.from_json {
                                if r#dependent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dependent"));
                                }
                                r#dependent = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleDependent >> ()) ?) ;
                            } else {
                                let vec = r#dependent.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRuleDependent > ()) ?) ;
                            }
                        }
                        Field::Documentation => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::DocumentationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("documentation");
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
                Ok(fhirbolt_model::r4::resources::StructureMapGroupRule {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#name.unwrap_or(Default::default())
                    } else {
                        r#name.ok_or(serde::de::Error::missing_field("name"))?
                    },
                    r#source: r#source.unwrap_or(vec![]),
                    r#target: r#target.unwrap_or(vec![]),
                    r#rule: r#rule.unwrap_or(vec![]),
                    r#dependent: r#dependent.unwrap_or(vec![]),
                    r#documentation,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroupRule>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroupRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroupRule>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroupRule>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::StructureMapGroupRule>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::StructureMapGroup>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap.group", field
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
        if self.output_json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#extends.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("extends", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_extends", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#extends.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("extends", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#type_mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("typeMode");
            }
            if let Some(some) = self.value.r#type_mode.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("typeMode", &some)?;
            }
            if self.value.r#type_mode.id.is_some() || !self.value.r#type_mode.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#type_mode.id.as_ref(),
                    extension: &self.value.r#type_mode.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_typeMode", ctx)
                })?;
            }
        } else {
            if self.value.r#type_mode.id.as_deref() == Some("$invalid") {
                return missing_field_error("typeMode");
            }
            self.with_context(&self.value.r#type_mode, |ctx| {
                state.serialize_entry("typeMode", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#documentation.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("documentation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_documentation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#documentation.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("documentation", ctx))?;
            }
        }
        if !self.value.r#input.is_empty() {
            self.with_context(&self.value.r#input, |ctx| {
                state.serialize_entry("input", ctx)
            })?;
        }
        if !self.value.r#rule.is_empty() {
            self.with_context(&self.value.r#rule, |ctx| state.serialize_entry("rule", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::StructureMapGroup>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4::resources::StructureMapGroup>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMapGroup>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::StructureMapGroup,
    >
{
    type Value = fhirbolt_model::r4::resources::StructureMapGroup;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMapGroup,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMapGroup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroup")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMapGroup, V::Error>
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
                    #[serde(rename = "extends")]
                    Extends,
                    #[serde(rename = "_extends")]
                    ExtendsPrimitiveElement,
                    #[serde(rename = "typeMode")]
                    TypeMode,
                    #[serde(rename = "_typeMode")]
                    TypeModePrimitiveElement,
                    #[serde(rename = "documentation")]
                    Documentation,
                    #[serde(rename = "_documentation")]
                    DocumentationPrimitiveElement,
                    #[serde(rename = "input")]
                    Input,
                    #[serde(rename = "rule")]
                    Rule,
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
                            "extends",
                            "typeMode",
                            "documentation",
                            "input",
                            "rule",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#extends: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#type_mode: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#documentation: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#input: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapGroupInput>,
                > = None;
                let mut r#rule: Option<Vec<fhirbolt_model::r4::resources::StructureMapGroupRule>> =
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
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
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Extends => {
                            if self.0.from_json {
                                let some = r#extends.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extends"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#extends.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extends"));
                                }
                                r#extends = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Id>(),
                                )?);
                            }
                        }
                        Field::ExtendsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#extends.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_extends"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("extends");
                            }
                        }
                        Field::TypeMode => {
                            if self.0.from_json {
                                let some = r#type_mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeMode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#type_mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeMode"));
                                }
                                r#type_mode = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::TypeModePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type_mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_typeMode"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("typeMode");
                            }
                        }
                        Field::Documentation => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::DocumentationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#documentation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_documentation",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("documentation");
                            }
                        }
                        Field::Input => {
                            if self.0.from_json {
                                if r#input.is_some() {
                                    return Err(serde::de::Error::duplicate_field("input"));
                                }
                                r#input =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::StructureMapGroupInput,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#input.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupInput > ()) ?) ;
                            }
                        }
                        Field::Rule => {
                            if self.0.from_json {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::StructureMapGroupRule,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#rule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroupRule > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::StructureMapGroup {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#name.unwrap_or(Default::default())
                    } else {
                        r#name.ok_or(serde::de::Error::missing_field("name"))?
                    },
                    r#extends,
                    r#type_mode: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#type_mode.unwrap_or(Default::default())
                    } else {
                        r#type_mode.ok_or(serde::de::Error::missing_field("typeMode"))?
                    },
                    r#documentation,
                    r#input: r#input.unwrap_or(vec![]),
                    r#rule: r#rule.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMapGroup>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMapGroup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMapGroup>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMapGroup>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMapGroup>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMapGroup>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMapGroup>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMapGroup>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroup>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMapGroup>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMapGroup>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::StructureMapGroup>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::StructureMap {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::StructureMap>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "StructureMap", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "StructureMap")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
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
        if self.output_json {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            if let Some(some) = self.value.r#url.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("url", &some)?;
            }
            if self.value.r#url.id.is_some() || !self.value.r#url.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#url.id.as_ref(),
                    extension: &self.value.r#url.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_url", ctx))?;
            }
        } else {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            self.with_context(&self.value.r#url, |ctx| state.serialize_entry("url", ctx))?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("version", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#version.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            if let Some(some) = self.value.r#name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if self.value.r#name.id.is_some() || !self.value.r#name.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#name.id.as_ref(),
                    extension: &self.value.r#name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_name", ctx)
                })?;
            }
        } else {
            if self.value.r#name.id.as_deref() == Some("$invalid") {
                return missing_field_error("name");
            }
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#title.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("title", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_title", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#title.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("title", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#experimental.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("experimental", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_experimental", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#experimental.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("experimental", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("date", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_date", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("date", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#publisher.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("publisher", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_publisher", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#publisher.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
            }
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#description.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
            }
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
        if self.output_json {
            if let Some(some) = self.value.r#purpose.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("purpose", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_purpose", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#purpose.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("purpose", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#copyright.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("copyright", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_copyright", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#copyright.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("copyright", ctx))?;
            }
        }
        if !self.value.r#structure.is_empty() {
            self.with_context(&self.value.r#structure, |ctx| {
                state.serialize_entry("structure", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#import.is_empty() {
                let values = self
                    .value
                    .r#import
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("import", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#import
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#import
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(super::super::serde_helpers::PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_import", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#import.is_empty() {
                self.with_context(&self.value.r#import, |ctx| {
                    state.serialize_entry("import", ctx)
                })?;
            }
        }
        if !self.value.r#group.is_empty() {
            self.with_context(&self.value.r#group, |ctx| {
                state.serialize_entry("group", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::StructureMap>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::StructureMap>>
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::StructureMap>>,
    >
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::StructureMap>
{
    type Value = fhirbolt_model::r4::resources::StructureMap;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::StructureMap>
{
    type Value = fhirbolt_model::r4::resources::StructureMap;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::StructureMap,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::StructureMap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMap")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::StructureMap, V::Error>
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
                    #[serde(rename = "structure")]
                    Structure,
                    #[serde(rename = "import")]
                    Import,
                    #[serde(rename = "_import")]
                    ImportPrimitiveElement,
                    #[serde(rename = "group")]
                    Group,
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
                            "structure",
                            "import",
                            "group",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#url: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#version: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#experimental: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#date: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#publisher: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#contact: Option<Vec<Box<fhirbolt_model::r4::types::ContactDetail>>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r4::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<fhirbolt_model::r4::types::UsageContext>>> =
                    None;
                let mut r#jurisdiction: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#purpose: Option<fhirbolt_model::r4::types::Markdown> = None;
                let mut r#copyright: Option<fhirbolt_model::r4::types::Markdown> = None;
                let mut r#structure: Option<
                    Vec<fhirbolt_model::r4::resources::StructureMapStructure>,
                > = None;
                let mut r#import: Option<Vec<fhirbolt_model::r4::types::Canonical>> = None;
                let mut r#group: Option<Vec<fhirbolt_model::r4::resources::StructureMapGroup>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "StructureMap" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"StructureMap",
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
                            r#meta = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Meta>>(),
                            )?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                r#implicit_rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
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
                                r#language = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r4::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Url => {
                            if self.0.from_json {
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
                                r#url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
                                )?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::Version => {
                            if self.0.from_json {
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
                                r#version = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
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
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Title => {
                            if self.0.from_json {
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
                                r#title = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TitlePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::Status => {
                            if self.0.from_json {
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
                                r#status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Experimental => {
                            if self.0.from_json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#experimental.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                r#experimental = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ExperimentalPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_experimental"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("experimental");
                            }
                        }
                        Field::Date => {
                            if self.0.from_json {
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
                                r#date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::DatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("date");
                            }
                        }
                        Field::Publisher => {
                            if self.0.from_json {
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
                                r#publisher = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::PublisherPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publisher");
                            }
                        }
                        Field::Contact => {
                            if self.0.from_json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::UseContext => {
                            if self.0.from_json {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: UsageContext > >> ()) ?) ;
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: UsageContext > > ()) ?) ;
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from_json {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#jurisdiction.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Purpose => {
                            if self.0.from_json {
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
                                r#purpose = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::PurposePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Copyright => {
                            if self.0.from_json {
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
                                r#copyright = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::CopyrightPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyright");
                            }
                        }
                        Field::Structure => {
                            if self.0.from_json {
                                if r#structure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("structure"));
                                }
                                r#structure =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::StructureMapStructure,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#structure.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapStructure > ()) ?) ;
                            }
                        }
                        Field::Import => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#import.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("import"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#import.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::ImportPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#import.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_import"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("import");
                            }
                        }
                        Field::Group => {
                            if self.0.from_json {
                                if r#group.is_some() {
                                    return Err(serde::de::Error::duplicate_field("group"));
                                }
                                r#group =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4::resources::StructureMapGroup,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#group.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: StructureMapGroup > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::StructureMap {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#url.unwrap_or(Default::default())
                    } else {
                        r#url.ok_or(serde::de::Error::missing_field("url"))?
                    },
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#version,
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
                    r#structure: r#structure.unwrap_or(vec![]),
                    r#import: r#import.unwrap_or(vec![]),
                    r#group: r#group.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::StructureMap>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::StructureMap>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::StructureMap>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::StructureMap>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::StructureMap>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::StructureMap>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::StructureMap>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4::resources::StructureMap>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::StructureMap>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMap>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::StructureMap>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::StructureMap>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4::resources::StructureMap>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
