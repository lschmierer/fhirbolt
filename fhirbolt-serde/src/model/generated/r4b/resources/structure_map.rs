// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMapStructure,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapStructure;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapStructure,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapStructure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapStructure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapStructure, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#url: Option<fhirbolt_model::r4b::types::Canonical> = None;
                let mut r#mode: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#alias: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#documentation: Option<fhirbolt_model::r4b::types::String> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                Ok(fhirbolt_model::r4b::resources::StructureMapStructure {
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
        Box<fhirbolt_model::r4b::resources::StructureMapStructure>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapStructure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapStructure>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapStructure>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapStructure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapStructure>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapStructure>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapStructure>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapStructure>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapStructure>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapStructure>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapStructure>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureMapStructure>>(),
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
        fhirbolt_model::r4b::resources::StructureMapGroupInput,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupInput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupInput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupInput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroupInput, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#type: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#mode: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#documentation: Option<fhirbolt_model::r4b::types::String> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                Ok(fhirbolt_model::r4b::resources::StructureMapGroupInput {
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
        Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupInput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupInput>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupInput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupInput>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapGroupInput>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureMapGroupInput>>(),
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
        fhirbolt_model::r4b::resources::StructureMapGroupRuleSource,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleSource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupRuleSource,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleSource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#context: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#min: Option<fhirbolt_model::r4b::types::Integer> = None;
                let mut r#max: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#type: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#default_value: Option<
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleSourceDefaultValue,
                > = None;
                let mut r#element: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#list_mode: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#variable: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#condition: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#check: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#log_message: Option<fhirbolt_model::r4b::types::String> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Integer>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBase64Binary",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBase64Binary");
                            }
                        }
                        Field::DefaultValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBoolean",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBoolean");
                            }
                        }
                        Field::DefaultValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCanonical",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCanonical");
                            }
                        }
                        Field::DefaultValueCode => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCode")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCode",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Code > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Code (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCode")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCode");
                            }
                        }
                        Field::DefaultValueDate => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDate",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDate");
                            }
                        }
                        Field::DefaultValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDateTime",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDateTime");
                            }
                        }
                        Field::DefaultValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDecimal",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDecimal");
                            }
                        }
                        Field::DefaultValueId => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueId",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Id > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueId");
                            }
                        }
                        Field::DefaultValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInstant",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInstant");
                            }
                        }
                        Field::DefaultValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInteger",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInteger");
                            }
                        }
                        Field::DefaultValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMarkdown",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueMarkdown");
                            }
                        }
                        Field::DefaultValueOid => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueOid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueOid",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Oid > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Oid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueOid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueOid");
                            }
                        }
                        Field::DefaultValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePositiveInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValuePositiveInt");
                            }
                        }
                        Field::DefaultValueString => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueString",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueString");
                            }
                        }
                        Field::DefaultValueTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTime",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Time > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Time (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueTime");
                            }
                        }
                        Field::DefaultValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUnsignedInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUnsignedInt");
                            }
                        }
                        Field::DefaultValueUri => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUri",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUri");
                            }
                        }
                        Field::DefaultValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUrl")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUrl",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Url > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Url (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUrl")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUrl");
                            }
                        }
                        Field::DefaultValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUuid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUuid",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Uuid > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Uuid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUuid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
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
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Address (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Address > > ()) ?)) ;
                        }
                        Field::DefaultValueAge => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueAge"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Age (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Age > > ()) ?)) ;
                        }
                        Field::DefaultValueAnnotation => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAnnotation",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::DefaultValueAttachment => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAttachment",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::DefaultValueCodeableConcept => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableConcept",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::DefaultValueCoding => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCoding",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Coding (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Coding > > ()) ?)) ;
                        }
                        Field::DefaultValueContactPoint => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactPoint",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::DefaultValueCount => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueCount"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Count (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Count > > ()) ?)) ;
                        }
                        Field::DefaultValueDistance => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDistance",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Distance (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Distance > > ()) ?)) ;
                        }
                        Field::DefaultValueDuration => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDuration",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Duration (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Duration > > ()) ?)) ;
                        }
                        Field::DefaultValueHumanName => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueHumanName",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::DefaultValueIdentifier => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueIdentifier",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::DefaultValueMoney => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMoney"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Money (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Money > > ()) ?)) ;
                        }
                        Field::DefaultValuePeriod => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValuePeriod",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Period (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Period > > ()) ?)) ;
                        }
                        Field::DefaultValueQuantity => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueQuantity",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::DefaultValueRange => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRange"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Range > > ()) ?)) ;
                        }
                        Field::DefaultValueRatio => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRatio"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Ratio (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Ratio > > ()) ?)) ;
                        }
                        Field::DefaultValueReference => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueReference",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::DefaultValueSampledData => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSampledData",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::DefaultValueSignature => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSignature",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::DefaultValueTiming => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTiming",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Timing (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Timing > > ()) ?)) ;
                        }
                        Field::DefaultValueContactDetail => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactDetail",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::DefaultValueContributor => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContributor",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::DefaultValueDataRequirement => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDataRequirement",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::DefaultValueExpression => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueExpression",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::DefaultValueParameterDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueParameterDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueRelatedArtifact => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRelatedArtifact",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::DefaultValueTriggerDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTriggerDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueUsageContext => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueUsageContext",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::DefaultValueDosage => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDosage",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Dosage (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Dosage > > ()) ?)) ;
                        }
                        Field::DefaultValueMeta => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMeta"));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSourceDefaultValue :: Meta (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Meta > > ()) ?)) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                Ok(
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleSource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#context: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
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
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSource >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTargetParameter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter,
                V::Error,
            >
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#value: Option<
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameterValue,
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Id > > ()) ?)) ;
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameterValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
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
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter {
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
        Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameter > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>>;
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
                            fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter,
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleTarget")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#context: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#context_type: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#element: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#variable: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#list_mode: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#list_rule_id: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#transform: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#parameter: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTargetParameter>,
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                r#parameter = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameter >> ()) ?) ;
                            } else {
                                let vec = r#parameter.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTargetParameter > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget {
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
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTarget >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRuleDependent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#variable: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                    fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent {
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
        Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleDependent > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleDependent >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMapGroupRule,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroupRule;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroupRule,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroupRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroupRule")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroupRule, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#source: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleSource>,
                > = None;
                let mut r#target: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget>,
                > = None;
                let mut r#rule: Option<Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>> =
                    None;
                let mut r#dependent: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapGroupRuleDependent>,
                > = None;
                let mut r#documentation: Option<fhirbolt_model::r4b::types::String> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                        fhirbolt_model::r4b::resources::StructureMapGroupRuleSource,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#source.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleSource > ()) ?) ;
                            }
                        }
                        Field::Target => {
                            if self.0.from_json {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::StructureMapGroupRuleTarget,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#target.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleTarget > ()) ?) ;
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
                                            fhirbolt_model::r4b::resources::StructureMapGroupRule,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#rule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRule > ()) ?) ;
                            }
                        }
                        Field::Dependent => {
                            if self.0.from_json {
                                if r#dependent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dependent"));
                                }
                                r#dependent = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleDependent >> ()) ?) ;
                            } else {
                                let vec = r#dependent.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRuleDependent > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                Ok(fhirbolt_model::r4b::resources::StructureMapGroupRule {
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
        Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRule>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapGroupRule>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureMapGroupRule>>(),
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
        fhirbolt_model::r4b::resources::StructureMapGroup,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMapGroup;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMapGroup,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMapGroup;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMapGroup")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMapGroup, V::Error>
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#extends: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#type_mode: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#documentation: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#input: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapGroupInput>,
                > = None;
                let mut r#rule: Option<Vec<fhirbolt_model::r4b::resources::StructureMapGroupRule>> =
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                            fhirbolt_model::r4b::resources::StructureMapGroupInput,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#input.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupInput > ()) ?) ;
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
                                            fhirbolt_model::r4b::resources::StructureMapGroupRule,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#rule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroupRule > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::StructureMapGroup {
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
        Box<fhirbolt_model::r4b::resources::StructureMapGroup>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMapGroup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMapGroup>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMapGroup>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroup>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMapGroup>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMapGroup>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMapGroup>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroup>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroup>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroup>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMapGroup>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureMapGroup>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::StructureMap {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::StructureMap>
{
    type Value = fhirbolt_model::r4b::resources::StructureMap;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::StructureMap,
    >
{
    type Value = fhirbolt_model::r4b::resources::StructureMap;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::StructureMap,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::StructureMap;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("StructureMap")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::StructureMap, V::Error>
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
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4b::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#url: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#version: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#experimental: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#publisher: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#contact: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<fhirbolt_model::r4b::types::UsageContext>>> =
                    None;
                let mut r#jurisdiction: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#purpose: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#copyright: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#structure: Option<
                    Vec<fhirbolt_model::r4b::resources::StructureMapStructure>,
                > = None;
                let mut r#import: Option<Vec<fhirbolt_model::r4b::types::Canonical>> = None;
                let mut r#group: Option<Vec<fhirbolt_model::r4b::resources::StructureMapGroup>> =
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
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r4b::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<Box<fhirbolt_model::r4b::Resource>>>(),
                                    )?,
                                );
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
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
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                                r#contact = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
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
                                r#use_context = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: UsageContext > >> ()) ?) ;
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?) ;
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
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#jurisdiction.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
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
                                            fhirbolt_model::r4b::resources::StructureMapStructure,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#structure.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapStructure > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
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
                                                fhirbolt_model::r4b::resources::StructureMapGroup,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#group.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: StructureMapGroup > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::StructureMap {
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
        Box<fhirbolt_model::r4b::resources::StructureMap>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::StructureMap>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::StructureMap>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::StructureMap>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::StructureMap>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::StructureMap>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::StructureMap>;
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
                        .transmute::<fhirbolt_model::r4b::resources::StructureMap>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::StructureMap>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMap>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::StructureMap>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::StructureMap>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureMap>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
