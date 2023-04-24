// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator,
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
                "ElementDefinition.slicing.discriminator", field
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
        if self.output_json {
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            if let Some(some) = self.value.r#type.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("type", &some)?;
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_type", ctx)
                })?;
            }
        } else {
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicingDiscriminator")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "type", "path"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#type: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                r#path = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
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
                    fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#path: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#path.unwrap_or(Default::default())
                        } else {
                            r#path.ok_or(serde::de::Error::missing_field("path"))?
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
        Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4b::types::ElementDefinitionSlicing,
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
                "ElementDefinition.slicing", field
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
        if !self.value.r#discriminator.is_empty() {
            self.with_context(&self.value.r#discriminator, |ctx| {
                state.serialize_entry("discriminator", ctx)
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
        if self.output_json {
            if let Some(some) = self.value.r#ordered.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("ordered", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_ordered", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#ordered.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("ordered", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#rules.id.as_deref() == Some("$invalid") {
                return missing_field_error("rules");
            }
            if let Some(some) = self.value.r#rules.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("rules", &some)?;
            }
            if self.value.r#rules.id.is_some() || !self.value.r#rules.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#rules.id.as_ref(),
                    extension: &self.value.r#rules.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_rules", ctx)
                })?;
            }
        } else {
            if self.value.r#rules.id.as_deref() == Some("$invalid") {
                return missing_field_error("rules");
            }
            self.with_context(&self.value.r#rules, |ctx| {
                state.serialize_entry("rules", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicing>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionSlicing,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionSlicing;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionSlicing,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionSlicing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicing")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionSlicing, V::Error>
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
                    #[serde(rename = "discriminator")]
                    Discriminator,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "ordered")]
                    Ordered,
                    #[serde(rename = "_ordered")]
                    OrderedPrimitiveElement,
                    #[serde(rename = "rules")]
                    Rules,
                    #[serde(rename = "_rules")]
                    RulesPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "discriminator",
                            "description",
                            "ordered",
                            "rules",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#discriminator: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
                > = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#ordered: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#rules: Option<fhirbolt_model::r4b::types::Code> = None;
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
                        Field::Discriminator => {
                            if self.0.from_json {
                                if r#discriminator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("discriminator"));
                                }
                                r#discriminator = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator >> ()) ?) ;
                            } else {
                                let vec = r#discriminator.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                        Field::Ordered => {
                            if self.0.from_json {
                                let some = r#ordered.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ordered"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#ordered.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ordered"));
                                }
                                r#ordered = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::OrderedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#ordered.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_ordered"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("ordered");
                            }
                        }
                        Field::Rules => {
                            if self.0.from_json {
                                let some = r#rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rules"));
                                }
                                r#rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::RulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_rules"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("rules");
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
                Ok(fhirbolt_model::r4b::types::ElementDefinitionSlicing {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#discriminator: r#discriminator.unwrap_or(vec![]),
                    r#description,
                    r#ordered,
                    r#rules: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#rules.unwrap_or(Default::default())
                    } else {
                        r#rules.ok_or(serde::de::Error::missing_field("rules"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionSlicing>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicing>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicing>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicing>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionSlicing>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionSlicing>>(),
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
        &fhirbolt_model::r4b::types::ElementDefinitionBase,
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
                "ElementDefinition.base", field
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
        if self.output_json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        }
        if self.output_json {
            if self.value.r#min.id.as_deref() == Some("$invalid") {
                return missing_field_error("min");
            }
            if let Some(some) = self.value.r#min.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("min", &some)?;
            }
            if self.value.r#min.id.is_some() || !self.value.r#min.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#min.id.as_ref(),
                    extension: &self.value.r#min.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_min", ctx))?;
            }
        } else {
            if self.value.r#min.id.as_deref() == Some("$invalid") {
                return missing_field_error("min");
            }
            self.with_context(&self.value.r#min, |ctx| state.serialize_entry("min", ctx))?;
        }
        if self.output_json {
            if self.value.r#max.id.as_deref() == Some("$invalid") {
                return missing_field_error("max");
            }
            if let Some(some) = self.value.r#max.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("max", &some)?;
            }
            if self.value.r#max.id.is_some() || !self.value.r#max.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#max.id.as_ref(),
                    extension: &self.value.r#max.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_max", ctx))?;
            }
        } else {
            if self.value.r#max.id.as_deref() == Some("$invalid") {
                return missing_field_error("max");
            }
            self.with_context(&self.value.r#max, |ctx| state.serialize_entry("max", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionBase>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionBase>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionBase,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionBase;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionBase,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionBase;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBase")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionBase, V::Error>
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
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "min")]
                    Min,
                    #[serde(rename = "_min")]
                    MinPrimitiveElement,
                    #[serde(rename = "max")]
                    Max,
                    #[serde(rename = "_max")]
                    MaxPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "path", "min", "max"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#min: Option<fhirbolt_model::r4b::types::UnsignedInt> = None;
                let mut r#max: Option<fhirbolt_model::r4b::types::String> = None;
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
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                r#path = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
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
                                r#min = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::UnsignedInt>(),
                                    )?,
                                );
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::types::ElementDefinitionBase {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        r#path.ok_or(serde::de::Error::missing_field("path"))?
                    },
                    r#min: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#min.unwrap_or(Default::default())
                    } else {
                        r#min.ok_or(serde::de::Error::missing_field("min"))?
                    },
                    r#max: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#max.unwrap_or(Default::default())
                    } else {
                        r#max.ok_or(serde::de::Error::missing_field("max"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionBase>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionBase>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionBase>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionBase>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionBase>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionBase>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionBase>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionBase>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionBase>>(),
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
        &fhirbolt_model::r4b::types::ElementDefinitionType,
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
                "ElementDefinition.type", field
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
        if self.output_json {
            if self.value.r#code.id.as_deref() == Some("$invalid") {
                return missing_field_error("code");
            }
            if let Some(some) = self.value.r#code.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("code", &some)?;
            }
            if self.value.r#code.id.is_some() || !self.value.r#code.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#code.id.as_ref(),
                    extension: &self.value.r#code.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_code", ctx)
                })?;
            }
        } else {
            if self.value.r#code.id.as_deref() == Some("$invalid") {
                return missing_field_error("code");
            }
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if self.output_json {
            if !self.value.r#profile.is_empty() {
                let values = self
                    .value
                    .r#profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
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
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#profile
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
                        state.serialize_entry("_profile", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#profile.is_empty() {
                self.with_context(&self.value.r#profile, |ctx| {
                    state.serialize_entry("profile", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#target_profile.is_empty() {
                let values = self
                    .value
                    .r#target_profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("targetProfile", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#target_profile
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#target_profile
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
                        state.serialize_entry("_targetProfile", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#target_profile.is_empty() {
                self.with_context(&self.value.r#target_profile, |ctx| {
                    state.serialize_entry("targetProfile", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#aggregation.is_empty() {
                let values = self
                    .value
                    .r#aggregation
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("aggregation", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#aggregation
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#aggregation
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
                        state.serialize_entry("_aggregation", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#aggregation.is_empty() {
                self.with_context(&self.value.r#aggregation, |ctx| {
                    state.serialize_entry("aggregation", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#versioning.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("versioning", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_versioning", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#versioning.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("versioning", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionType>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionType>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionType,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionType;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionType,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionType")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionType, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "_code")]
                    CodePrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "_profile")]
                    ProfilePrimitiveElement,
                    #[serde(rename = "targetProfile")]
                    TargetProfile,
                    #[serde(rename = "_targetProfile")]
                    TargetProfilePrimitiveElement,
                    #[serde(rename = "aggregation")]
                    Aggregation,
                    #[serde(rename = "_aggregation")]
                    AggregationPrimitiveElement,
                    #[serde(rename = "versioning")]
                    Versioning,
                    #[serde(rename = "_versioning")]
                    VersioningPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "code",
                            "profile",
                            "targetProfile",
                            "aggregation",
                            "versioning",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#code: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#profile: Option<Vec<fhirbolt_model::r4b::types::Canonical>> = None;
                let mut r#target_profile: Option<Vec<fhirbolt_model::r4b::types::Canonical>> = None;
                let mut r#aggregation: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#versioning: Option<fhirbolt_model::r4b::types::Code> = None;
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
                        Field::Code => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::CodePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_code"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("code");
                            }
                        }
                        Field::Profile => {
                            if self.0.from_json {
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
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
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
                        Field::TargetProfile => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#target_profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("targetProfile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#target_profile.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::TargetProfilePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#target_profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_targetProfile",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("targetProfile");
                            }
                        }
                        Field::Aggregation => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#aggregation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("aggregation"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#aggregation.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::AggregationPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#aggregation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_aggregation"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("aggregation");
                            }
                        }
                        Field::Versioning => {
                            if self.0.from_json {
                                let some = r#versioning.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versioning"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#versioning.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versioning"));
                                }
                                r#versioning = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::VersioningPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#versioning.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_versioning"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("versioning");
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
                Ok(fhirbolt_model::r4b::types::ElementDefinitionType {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#target_profile: r#target_profile.unwrap_or(vec![]),
                    r#aggregation: r#aggregation.unwrap_or(vec![]),
                    r#versioning,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionType>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionType>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionType>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionType>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionType>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionType>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionType>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionType>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionType>>(),
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
        &fhirbolt_model::r4b::types::ElementDefinitionExample,
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
                "ElementDefinition.example", field
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
        if self.output_json {
            if self.value.r#label.id.as_deref() == Some("$invalid") {
                return missing_field_error("label");
            }
            if let Some(some) = self.value.r#label.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("label", &some)?;
            }
            if self.value.r#label.id.is_some() || !self.value.r#label.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#label.id.as_ref(),
                    extension: &self.value.r#label.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_label", ctx)
                })?;
            }
        } else {
            if self.value.r#label.id.as_deref() == Some("$invalid") {
                return missing_field_error("label");
            }
            self.with_context(&self.value.r#label, |ctx| {
                state.serialize_entry("label", ctx)
            })?;
        }
        match self.value.r#value {
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Base64Binary(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBase64Binary", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueBase64Binary", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueBase64Binary", ctx)
                    })?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Boolean(ref value) => {
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
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Canonical(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueCanonical", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueCanonical", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueCanonical", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Code(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueCode", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueCode", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueCode", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Date(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDate", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDate", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::DateTime(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDateTime", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDateTime", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Decimal(ref value) => {
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
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Id(ref value) => {
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
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Instant(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueInstant", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueInstant", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueInstant", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Integer(ref value) => {
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
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Markdown(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueMarkdown", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueMarkdown", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueMarkdown", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Oid(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueOid", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueOid", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueOid", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::PositiveInt(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valuePositiveInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valuePositiveInt", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valuePositiveInt", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::String(ref value) => {
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
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Time(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueTime", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueTime", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::UnsignedInt(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueUnsignedInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueUnsignedInt", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueUnsignedInt", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uri(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueUri", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueUri", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueUri", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Url(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueUrl", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueUrl", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueUrl", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uuid(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueUuid", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueUuid", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueUuid", ctx))?;
                }
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Address(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Age(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Annotation(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAnnotation", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::CodeableConcept(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::CodeableReference(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueCodeableReference", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Coding(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::ContactPoint(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContactPoint", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Count(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Distance(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDistance", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Duration(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDuration", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::HumanName(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueHumanName", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Identifier(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueIdentifier", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Money(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Period(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Quantity(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Range(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Ratio(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::RatioRange(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRatioRange", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::SampledData(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSampledData", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Signature(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSignature", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Timing(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::ContactDetail(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueContactDetail", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Contributor(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContributor", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::DataRequirement(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueDataRequirement", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Expression(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueExpression", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::ParameterDefinition(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueParameterDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::RelatedArtifact(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueRelatedArtifact", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::TriggerDefinition(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueTriggerDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::UsageContext(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueUsageContext", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Dosage(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
            }
            fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionExample>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionExample,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionExample;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionExample,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionExample;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionExample")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionExample, V::Error>
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
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "valueBase64Binary")]
                    ValueBase64Binary,
                    #[serde(rename = "_valueBase64Binary")]
                    ValueBase64BinaryPrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueCanonical")]
                    ValueCanonical,
                    #[serde(rename = "_valueCanonical")]
                    ValueCanonicalPrimitiveElement,
                    #[serde(rename = "valueCode")]
                    ValueCode,
                    #[serde(rename = "_valueCode")]
                    ValueCodePrimitiveElement,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueDateTime")]
                    ValueDateTime,
                    #[serde(rename = "_valueDateTime")]
                    ValueDateTimePrimitiveElement,
                    #[serde(rename = "valueDecimal")]
                    ValueDecimal,
                    #[serde(rename = "_valueDecimal")]
                    ValueDecimalPrimitiveElement,
                    #[serde(rename = "valueId")]
                    ValueId,
                    #[serde(rename = "_valueId")]
                    ValueIdPrimitiveElement,
                    #[serde(rename = "valueInstant")]
                    ValueInstant,
                    #[serde(rename = "_valueInstant")]
                    ValueInstantPrimitiveElement,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueMarkdown")]
                    ValueMarkdown,
                    #[serde(rename = "_valueMarkdown")]
                    ValueMarkdownPrimitiveElement,
                    #[serde(rename = "valueOid")]
                    ValueOid,
                    #[serde(rename = "_valueOid")]
                    ValueOidPrimitiveElement,
                    #[serde(rename = "valuePositiveInt")]
                    ValuePositiveInt,
                    #[serde(rename = "_valuePositiveInt")]
                    ValuePositiveIntPrimitiveElement,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueTime")]
                    ValueTime,
                    #[serde(rename = "_valueTime")]
                    ValueTimePrimitiveElement,
                    #[serde(rename = "valueUnsignedInt")]
                    ValueUnsignedInt,
                    #[serde(rename = "_valueUnsignedInt")]
                    ValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "valueUri")]
                    ValueUri,
                    #[serde(rename = "_valueUri")]
                    ValueUriPrimitiveElement,
                    #[serde(rename = "valueUrl")]
                    ValueUrl,
                    #[serde(rename = "_valueUrl")]
                    ValueUrlPrimitiveElement,
                    #[serde(rename = "valueUuid")]
                    ValueUuid,
                    #[serde(rename = "_valueUuid")]
                    ValueUuidPrimitiveElement,
                    #[serde(rename = "valueAddress")]
                    ValueAddress,
                    #[serde(rename = "valueAge")]
                    ValueAge,
                    #[serde(rename = "valueAnnotation")]
                    ValueAnnotation,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueCodeableReference")]
                    ValueCodeableReference,
                    #[serde(rename = "valueCoding")]
                    ValueCoding,
                    #[serde(rename = "valueContactPoint")]
                    ValueContactPoint,
                    #[serde(rename = "valueCount")]
                    ValueCount,
                    #[serde(rename = "valueDistance")]
                    ValueDistance,
                    #[serde(rename = "valueDuration")]
                    ValueDuration,
                    #[serde(rename = "valueHumanName")]
                    ValueHumanName,
                    #[serde(rename = "valueIdentifier")]
                    ValueIdentifier,
                    #[serde(rename = "valueMoney")]
                    ValueMoney,
                    #[serde(rename = "valuePeriod")]
                    ValuePeriod,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "valueRatio")]
                    ValueRatio,
                    #[serde(rename = "valueRatioRange")]
                    ValueRatioRange,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    #[serde(rename = "valueSampledData")]
                    ValueSampledData,
                    #[serde(rename = "valueSignature")]
                    ValueSignature,
                    #[serde(rename = "valueTiming")]
                    ValueTiming,
                    #[serde(rename = "valueContactDetail")]
                    ValueContactDetail,
                    #[serde(rename = "valueContributor")]
                    ValueContributor,
                    #[serde(rename = "valueDataRequirement")]
                    ValueDataRequirement,
                    #[serde(rename = "valueExpression")]
                    ValueExpression,
                    #[serde(rename = "valueParameterDefinition")]
                    ValueParameterDefinition,
                    #[serde(rename = "valueRelatedArtifact")]
                    ValueRelatedArtifact,
                    #[serde(rename = "valueTriggerDefinition")]
                    ValueTriggerDefinition,
                    #[serde(rename = "valueUsageContext")]
                    ValueUsageContext,
                    #[serde(rename = "valueDosage")]
                    ValueDosage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "label",
                            "valueBase64Binary",
                            "valueBoolean",
                            "valueCanonical",
                            "valueCode",
                            "valueDate",
                            "valueDateTime",
                            "valueDecimal",
                            "valueId",
                            "valueInstant",
                            "valueInteger",
                            "valueMarkdown",
                            "valueOid",
                            "valuePositiveInt",
                            "valueString",
                            "valueTime",
                            "valueUnsignedInt",
                            "valueUri",
                            "valueUrl",
                            "valueUuid",
                            "valueAddress",
                            "valueAge",
                            "valueAnnotation",
                            "valueAttachment",
                            "valueCodeableConcept",
                            "valueCodeableReference",
                            "valueCoding",
                            "valueContactPoint",
                            "valueCount",
                            "valueDistance",
                            "valueDuration",
                            "valueHumanName",
                            "valueIdentifier",
                            "valueMoney",
                            "valuePeriod",
                            "valueQuantity",
                            "valueRange",
                            "valueRatio",
                            "valueRatioRange",
                            "valueReference",
                            "valueSampledData",
                            "valueSignature",
                            "valueTiming",
                            "valueContactDetail",
                            "valueContributor",
                            "valueDataRequirement",
                            "valueExpression",
                            "valueParameterDefinition",
                            "valueRelatedArtifact",
                            "valueTriggerDefinition",
                            "valueUsageContext",
                            "valueDosage",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#label: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::ElementDefinitionExampleValue> =
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
                        Field::Label => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                r#label = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueCanonical");
                            }
                        }
                        Field::ValueCode => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Code (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueCode")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Code (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueCode")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueCode");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Id(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInstant");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueOid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Oid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueOid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Oid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueOid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueOid");
                            }
                        }
                        Field::ValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valuePositiveInt");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Time (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Time (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUnsignedInt");
                            }
                        }
                        Field::ValueUri => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Url (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUrl")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Url (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUrl")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUrl");
                            }
                        }
                        Field::ValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Uuid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUuid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                r#value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Uuid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUuid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUuid");
                            }
                        }
                        Field::ValueAddress => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueAge => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Age(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueAnnotation => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueCodeableReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableReference",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: CodeableReference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableReference > > ()) ?)) ;
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueContactPoint => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::ValueCount => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Count(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueDistance => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueDuration => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueHumanName => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::ValueIdentifier => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::ValueMoney => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Money(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValuePeriod => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueRatio => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Ratio(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueRatioRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatioRange"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: RatioRange (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RatioRange > > ()) ?)) ;
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::ValueSampledData => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::ValueSignature => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::ValueTiming => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueContactDetail => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::ValueContributor => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::ValueDataRequirement => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::ValueExpression => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::ValueParameterDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::ValueRelatedArtifact => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::ValueTriggerDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::ValueUsageContext => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionExampleValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::ValueDosage => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionExampleValue::Dosage(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::types::ElementDefinitionExample {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#label: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#label.unwrap_or(Default::default())
                    } else {
                        r#label.ok_or(serde::de::Error::missing_field("label"))?
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionExample>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionExample>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionExample>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionExample>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionExample>>(),
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
        &fhirbolt_model::r4b::types::ElementDefinitionConstraint,
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
                "ElementDefinition.constraint", field
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
        if self.output_json {
            if self.value.r#key.id.as_deref() == Some("$invalid") {
                return missing_field_error("key");
            }
            if let Some(some) = self.value.r#key.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("key", &some)?;
            }
            if self.value.r#key.id.is_some() || !self.value.r#key.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#key.id.as_ref(),
                    extension: &self.value.r#key.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_key", ctx))?;
            }
        } else {
            if self.value.r#key.id.as_deref() == Some("$invalid") {
                return missing_field_error("key");
            }
            self.with_context(&self.value.r#key, |ctx| state.serialize_entry("key", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#requirements.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("requirements", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requirements", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#requirements.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("requirements", ctx))?;
            }
        }
        if self.output_json {
            if self.value.r#severity.id.as_deref() == Some("$invalid") {
                return missing_field_error("severity");
            }
            if let Some(some) = self.value.r#severity.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("severity", &some)?;
            }
            if self.value.r#severity.id.is_some() || !self.value.r#severity.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#severity.id.as_ref(),
                    extension: &self.value.r#severity.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_severity", ctx)
                })?;
            }
        } else {
            if self.value.r#severity.id.as_deref() == Some("$invalid") {
                return missing_field_error("severity");
            }
            self.with_context(&self.value.r#severity, |ctx| {
                state.serialize_entry("severity", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#human.id.as_deref() == Some("$invalid") {
                return missing_field_error("human");
            }
            if let Some(some) = self.value.r#human.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("human", &some)?;
            }
            if self.value.r#human.id.is_some() || !self.value.r#human.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#human.id.as_ref(),
                    extension: &self.value.r#human.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_human", ctx)
                })?;
            }
        } else {
            if self.value.r#human.id.as_deref() == Some("$invalid") {
                return missing_field_error("human");
            }
            self.with_context(&self.value.r#human, |ctx| {
                state.serialize_entry("human", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#expression.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("expression", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expression", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#expression.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#xpath.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("xpath", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_xpath", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#xpath.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("xpath", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#source.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("source", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_source", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#source.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("source", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionConstraint,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionConstraint;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionConstraint,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionConstraint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionConstraint")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionConstraint, V::Error>
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
                    #[serde(rename = "key")]
                    Key,
                    #[serde(rename = "_key")]
                    KeyPrimitiveElement,
                    #[serde(rename = "requirements")]
                    Requirements,
                    #[serde(rename = "_requirements")]
                    RequirementsPrimitiveElement,
                    #[serde(rename = "severity")]
                    Severity,
                    #[serde(rename = "_severity")]
                    SeverityPrimitiveElement,
                    #[serde(rename = "human")]
                    Human,
                    #[serde(rename = "_human")]
                    HumanPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    #[serde(rename = "_expression")]
                    ExpressionPrimitiveElement,
                    #[serde(rename = "xpath")]
                    Xpath,
                    #[serde(rename = "_xpath")]
                    XpathPrimitiveElement,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "_source")]
                    SourcePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "key",
                            "requirements",
                            "severity",
                            "human",
                            "expression",
                            "xpath",
                            "source",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#key: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#requirements: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#severity: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#human: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#expression: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#xpath: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#source: Option<fhirbolt_model::r4b::types::Canonical> = None;
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
                        Field::Key => {
                            if self.0.from_json {
                                let some = r#key.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("key"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#key.is_some() {
                                    return Err(serde::de::Error::duplicate_field("key"));
                                }
                                r#key = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
                                )?);
                            }
                        }
                        Field::KeyPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#key.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_key"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("key");
                            }
                        }
                        Field::Requirements => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#requirements.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                r#requirements = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::RequirementsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requirements"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requirements");
                            }
                        }
                        Field::Severity => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#severity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                r#severity = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::SeverityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_severity"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("severity");
                            }
                        }
                        Field::Human => {
                            if self.0.from_json {
                                let some = r#human.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("human"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#human.is_some() {
                                    return Err(serde::de::Error::duplicate_field("human"));
                                }
                                r#human = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::HumanPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#human.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_human"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("human");
                            }
                        }
                        Field::Expression => {
                            if self.0.from_json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                r#expression = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::ExpressionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_expression"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expression");
                            }
                        }
                        Field::Xpath => {
                            if self.0.from_json {
                                let some = r#xpath.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("xpath"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#xpath.is_some() {
                                    return Err(serde::de::Error::duplicate_field("xpath"));
                                }
                                r#xpath = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::XpathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#xpath.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_xpath"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("xpath");
                            }
                        }
                        Field::Source => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::SourcePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_source"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("source");
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
                Ok(fhirbolt_model::r4b::types::ElementDefinitionConstraint {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#key: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#key.unwrap_or(Default::default())
                    } else {
                        r#key.ok_or(serde::de::Error::missing_field("key"))?
                    },
                    r#requirements,
                    r#severity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#severity.unwrap_or(Default::default())
                    } else {
                        r#severity.ok_or(serde::de::Error::missing_field("severity"))?
                    },
                    r#human: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#human.unwrap_or(Default::default())
                    } else {
                        r#human.ok_or(serde::de::Error::missing_field("human"))?
                    },
                    r#expression,
                    r#xpath,
                    r#source,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionConstraint>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionConstraint>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionConstraint>>(
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4b::types::ElementDefinitionBinding,
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
                "ElementDefinition.binding", field
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
        if self.output_json {
            if self.value.r#strength.id.as_deref() == Some("$invalid") {
                return missing_field_error("strength");
            }
            if let Some(some) = self.value.r#strength.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("strength", &some)?;
            }
            if self.value.r#strength.id.is_some() || !self.value.r#strength.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#strength.id.as_ref(),
                    extension: &self.value.r#strength.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_strength", ctx)
                })?;
            }
        } else {
            if self.value.r#strength.id.as_deref() == Some("$invalid") {
                return missing_field_error("strength");
            }
            self.with_context(&self.value.r#strength, |ctx| {
                state.serialize_entry("strength", ctx)
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
        if self.output_json {
            if let Some(some) = self.value.r#value_set.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("valueSet", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_valueSet", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#value_set.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("valueSet", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionBinding>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionBinding,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionBinding;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionBinding,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionBinding;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBinding")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionBinding, V::Error>
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
                    #[serde(rename = "strength")]
                    Strength,
                    #[serde(rename = "_strength")]
                    StrengthPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "valueSet")]
                    ValueSet,
                    #[serde(rename = "_valueSet")]
                    ValueSetPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "strength", "description", "valueSet"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#strength: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#value_set: Option<fhirbolt_model::r4b::types::Canonical> = None;
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
                        Field::Strength => {
                            if self.0.from_json {
                                let some = r#strength.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                r#strength = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::StrengthPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#strength.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_strength"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("strength");
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                        Field::ValueSet => {
                            if self.0.from_json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                r#value_set = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::ValueSetPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueSet"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("valueSet");
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
                Ok(fhirbolt_model::r4b::types::ElementDefinitionBinding {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#strength: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#strength.unwrap_or(Default::default())
                    } else {
                        r#strength.ok_or(serde::de::Error::missing_field("strength"))?
                    },
                    r#description,
                    r#value_set,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionBinding>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionBinding>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionBinding>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionBinding>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionBinding>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionBinding>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionBinding>>(),
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
        &fhirbolt_model::r4b::types::ElementDefinitionMapping,
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
                "ElementDefinition.mapping", field
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
        if self.output_json {
            if self.value.r#identity.id.as_deref() == Some("$invalid") {
                return missing_field_error("identity");
            }
            if let Some(some) = self.value.r#identity.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("identity", &some)?;
            }
            if self.value.r#identity.id.is_some() || !self.value.r#identity.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#identity.id.as_ref(),
                    extension: &self.value.r#identity.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_identity", ctx)
                })?;
            }
        } else {
            if self.value.r#identity.id.as_deref() == Some("$invalid") {
                return missing_field_error("identity");
            }
            self.with_context(&self.value.r#identity, |ctx| {
                state.serialize_entry("identity", ctx)
            })?;
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
        if self.output_json {
            if self.value.r#map.id.as_deref() == Some("$invalid") {
                return missing_field_error("map");
            }
            if let Some(some) = self.value.r#map.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("map", &some)?;
            }
            if self.value.r#map.id.is_some() || !self.value.r#map.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#map.id.as_ref(),
                    extension: &self.value.r#map.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_map", ctx))?;
            }
        } else {
            if self.value.r#map.id.as_deref() == Some("$invalid") {
                return missing_field_error("map");
            }
            self.with_context(&self.value.r#map, |ctx| state.serialize_entry("map", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#comment.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("comment", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_comment", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#comment.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("comment", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>,
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
        fhirbolt_model::r4b::types::ElementDefinitionMapping,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinitionMapping;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinitionMapping,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinitionMapping;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionMapping")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinitionMapping, V::Error>
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
                    #[serde(rename = "identity")]
                    Identity,
                    #[serde(rename = "_identity")]
                    IdentityPrimitiveElement,
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "_language")]
                    LanguagePrimitiveElement,
                    #[serde(rename = "map")]
                    Map,
                    #[serde(rename = "_map")]
                    MapPrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "identity", "language", "map", "comment"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#identity: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#map: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#comment: Option<fhirbolt_model::r4b::types::String> = None;
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
                        Field::Identity => {
                            if self.0.from_json {
                                let some = r#identity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identity"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#identity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identity"));
                                }
                                r#identity = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
                                )?);
                            }
                        }
                        Field::IdentityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#identity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_identity"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("identity");
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
                        Field::Map => {
                            if self.0.from_json {
                                let some = r#map.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("map"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#map.is_some() {
                                    return Err(serde::de::Error::duplicate_field("map"));
                                }
                                r#map = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::MapPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#map.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_map"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("map");
                            }
                        }
                        Field::Comment => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#comment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                r#comment = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comment"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comment");
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
                Ok(fhirbolt_model::r4b::types::ElementDefinitionMapping {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#identity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#identity.unwrap_or(Default::default())
                    } else {
                        r#identity.ok_or(serde::de::Error::missing_field("identity"))?
                    },
                    r#language,
                    r#map: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#map.unwrap_or(Default::default())
                    } else {
                        r#map.ok_or(serde::de::Error::missing_field("map"))?
                    },
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinitionMapping>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinitionMapping>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinitionMapping>>(),
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
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::types::ElementDefinition>
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
                "ElementDefinition", field
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
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        }
        if self.output_json {
            if !self.value.r#representation.is_empty() {
                let values = self
                    .value
                    .r#representation
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("representation", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#representation
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#representation
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
                        state.serialize_entry("_representation", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#representation.is_empty() {
                self.with_context(&self.value.r#representation, |ctx| {
                    state.serialize_entry("representation", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#slice_name.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sliceName", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sliceName", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#slice_name.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("sliceName", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#slice_is_constraining.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sliceIsConstraining", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sliceIsConstraining", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#slice_is_constraining.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("sliceIsConstraining", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#label.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("label", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_label", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#label.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("label", ctx))?;
            }
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if let Some(some) = self.value.r#slicing.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("slicing", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#short.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("short", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_short", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#short.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("short", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#definition.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("definition", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_definition", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#definition.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("definition", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#comment.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("comment", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_comment", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#comment.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("comment", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#requirements.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("requirements", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requirements", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#requirements.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("requirements", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#alias.is_empty() {
                let values = self
                    .value
                    .r#alias
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("alias", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#alias
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#alias
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
                        state.serialize_entry("_alias", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#alias.is_empty() {
                self.with_context(&self.value.r#alias, |ctx| {
                    state.serialize_entry("alias", ctx)
                })?;
            }
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
        if let Some(some) = self.value.r#base.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("base", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#content_reference.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("contentReference", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_contentReference", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#content_reference.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("contentReference", ctx))?;
            }
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#default_value.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Base64Binary(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueBase64Binary", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueBase64Binary", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueBoolean", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Canonical(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCanonical", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Code(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueCode", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueCode", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCode", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDate", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("defaultValueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDecimal", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Id(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueId", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueId", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueId", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueInstant", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueInteger", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Markdown(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueMarkdown", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueMarkdown", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueMarkdown", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Oid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueOid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueOid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueOid", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::PositiveInt(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValuePositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValuePositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValuePositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueString", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::UnsignedInt(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uri(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueUri", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueUrl", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uuid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("defaultValueUuid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_defaultValueUuid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueUuid", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Address(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueAddress", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("defaultValueAge", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Annotation(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueAnnotation", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Attachment(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueAttachment", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::CodeableConcept(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::CodeableReference(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueCodeableReference", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Coding(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueCoding", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::ContactPoint(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueContactPoint", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Count(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueCount", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Distance(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueDistance", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Duration(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueDuration", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::HumanName(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueHumanName", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Identifier(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueIdentifier", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Money(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueMoney", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Period(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValuePeriod", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Quantity(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueQuantity", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Range(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueRange", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Ratio(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueRatio", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::RatioRange(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueRatioRange", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Reference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueReference", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::SampledData(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueSampledData", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Signature(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueSignature", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Timing(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueTiming", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::ContactDetail(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueContactDetail", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Contributor(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueContributor", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::DataRequirement(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueDataRequirement", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Expression(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueExpression", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::ParameterDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueParameterDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::RelatedArtifact(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueRelatedArtifact", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::TriggerDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueTriggerDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::UsageContext(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueUsageContext", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Dosage(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("defaultValueDosage", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Invalid => {
                    return Err(serde::ser::Error::custom("default_value is invalid"))
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#meaning_when_missing.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("meaningWhenMissing", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_meaningWhenMissing", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#meaning_when_missing.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("meaningWhenMissing", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#order_meaning.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("orderMeaning", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_orderMeaning", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#order_meaning.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("orderMeaning", ctx))?;
            }
        }
        if let Some(some) = self.value.r#fixed.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Base64Binary(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedBase64Binary", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedBase64Binary", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedBoolean", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Canonical(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedCanonical", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedCode", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedCode", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedCode", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedDate", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("fixedDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedDecimal", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedId", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedId", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedId", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedInstant", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedInteger", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Markdown(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedMarkdown", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedMarkdown", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedMarkdown", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedOid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedOid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedOid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedPositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedPositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedPositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedString", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedTime", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedUri", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedUrl", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("fixedUuid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_fixedUuid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("fixedUuid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Address(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedAddress", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedAge", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Annotation(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedAnnotation", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Attachment(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedAttachment", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::CodeableReference(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedCodeableReference", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Coding(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedCoding", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::ContactPoint(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedContactPoint", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Count(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedCount", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Distance(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedDistance", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Duration(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedDuration", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::HumanName(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedHumanName", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Identifier(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedIdentifier", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Money(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedMoney", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedPeriod", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedRange", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedRatio", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::RatioRange(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedRatioRange", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedReference", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::SampledData(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedSampledData", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Signature(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedSignature", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedTiming", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::ContactDetail(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedContactDetail", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Contributor(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedContributor", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::DataRequirement(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedDataRequirement", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Expression(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedExpression", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::ParameterDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedParameterDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::RelatedArtifact(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedRelatedArtifact", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::TriggerDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedTriggerDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::UsageContext(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("fixedUsageContext", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Dosage(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("fixedDosage", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionFixed::Invalid => {
                    return Err(serde::ser::Error::custom("fixed is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#pattern.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Base64Binary(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternBase64Binary", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternBase64Binary", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternBoolean", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Canonical(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternCanonical", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternCode", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternCode", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternCode", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternDate", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("patternDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDecimal", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternId", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternId", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternId", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternInstant", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternInteger", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Markdown(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternMarkdown", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternMarkdown", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternMarkdown", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternOid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternOid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternOid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternPositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternPositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternPositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternString", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternTime", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternUri", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternUrl", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("patternUuid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_patternUuid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("patternUuid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Address(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternAddress", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternAge", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Annotation(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternAnnotation", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Attachment(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternAttachment", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::CodeableConcept(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::CodeableReference(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternCodeableReference", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Coding(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternCoding", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::ContactPoint(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternContactPoint", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Count(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternCount", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Distance(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternDistance", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Duration(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternDuration", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::HumanName(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternHumanName", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Identifier(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternIdentifier", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Money(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternMoney", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternPeriod", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternRange", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternRatio", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::RatioRange(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternRatioRange", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternReference", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::SampledData(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternSampledData", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Signature(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternSignature", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternTiming", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::ContactDetail(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternContactDetail", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Contributor(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternContributor", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::DataRequirement(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternDataRequirement", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Expression(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternExpression", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::ParameterDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternParameterDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::RelatedArtifact(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternRelatedArtifact", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::TriggerDefinition(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternTriggerDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::UsageContext(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("patternUsageContext", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Dosage(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("patternDosage", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionPattern::Invalid => {
                    return Err(serde::ser::Error::custom("pattern is invalid"))
                }
            }
        }
        if !self.value.r#example.is_empty() {
            self.with_context(&self.value.r#example, |ctx| {
                state.serialize_entry("example", ctx)
            })?;
        }
        if let Some(some) = self.value.r#min_value.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("minValueDate", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueInstant", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("minValueTime", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("minValueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueDecimal", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueInteger", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValuePositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValuePositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValuePositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("minValueUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_minValueUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("minValueQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Invalid => {
                    return Err(serde::ser::Error::custom("min_value is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#max_value.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("maxValueDate", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueInstant", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("maxValueTime", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("maxValueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueDecimal", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueInteger", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValuePositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValuePositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValuePositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("maxValueUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_maxValueUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("maxValueQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Invalid => {
                    return Err(serde::ser::Error::custom("max_value is invalid"))
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#max_length.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("maxLength", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_maxLength", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#max_length.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("maxLength", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#condition.is_empty() {
                let values = self
                    .value
                    .r#condition
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("condition", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#condition
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#condition
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
                        state.serialize_entry("_condition", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#condition.is_empty() {
                self.with_context(&self.value.r#condition, |ctx| {
                    state.serialize_entry("condition", ctx)
                })?;
            }
        }
        if !self.value.r#constraint.is_empty() {
            self.with_context(&self.value.r#constraint, |ctx| {
                state.serialize_entry("constraint", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#must_support.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("mustSupport", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_mustSupport", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#must_support.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("mustSupport", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_modifier.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("isModifier", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isModifier", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#is_modifier.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("isModifier", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_modifier_reason.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("isModifierReason", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isModifierReason", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#is_modifier_reason.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("isModifierReason", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_summary.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("isSummary", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isSummary", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#is_summary.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("isSummary", ctx))?;
            }
        }
        if let Some(some) = self.value.r#binding.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("binding", ctx))?;
        }
        if !self.value.r#mapping.is_empty() {
            self.with_context(&self.value.r#mapping, |ctx| {
                state.serialize_entry("mapping", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::ElementDefinition>,
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
        &Vec<fhirbolt_model::r4b::types::ElementDefinition>,
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
        &Vec<Box<fhirbolt_model::r4b::types::ElementDefinition>>,
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
        fhirbolt_model::r4b::types::ElementDefinition,
    >
{
    type Value = fhirbolt_model::r4b::types::ElementDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::ElementDefinition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::ElementDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::ElementDefinition, V::Error>
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
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "representation")]
                    Representation,
                    #[serde(rename = "_representation")]
                    RepresentationPrimitiveElement,
                    #[serde(rename = "sliceName")]
                    SliceName,
                    #[serde(rename = "_sliceName")]
                    SliceNamePrimitiveElement,
                    #[serde(rename = "sliceIsConstraining")]
                    SliceIsConstraining,
                    #[serde(rename = "_sliceIsConstraining")]
                    SliceIsConstrainingPrimitiveElement,
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "slicing")]
                    Slicing,
                    #[serde(rename = "short")]
                    Short,
                    #[serde(rename = "_short")]
                    ShortPrimitiveElement,
                    #[serde(rename = "definition")]
                    Definition,
                    #[serde(rename = "_definition")]
                    DefinitionPrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    #[serde(rename = "requirements")]
                    Requirements,
                    #[serde(rename = "_requirements")]
                    RequirementsPrimitiveElement,
                    #[serde(rename = "alias")]
                    Alias,
                    #[serde(rename = "_alias")]
                    AliasPrimitiveElement,
                    #[serde(rename = "min")]
                    Min,
                    #[serde(rename = "_min")]
                    MinPrimitiveElement,
                    #[serde(rename = "max")]
                    Max,
                    #[serde(rename = "_max")]
                    MaxPrimitiveElement,
                    #[serde(rename = "base")]
                    Base,
                    #[serde(rename = "contentReference")]
                    ContentReference,
                    #[serde(rename = "_contentReference")]
                    ContentReferencePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
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
                    #[serde(rename = "defaultValueCodeableReference")]
                    DefaultValueCodeableReference,
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
                    #[serde(rename = "defaultValueRatioRange")]
                    DefaultValueRatioRange,
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
                    #[serde(rename = "meaningWhenMissing")]
                    MeaningWhenMissing,
                    #[serde(rename = "_meaningWhenMissing")]
                    MeaningWhenMissingPrimitiveElement,
                    #[serde(rename = "orderMeaning")]
                    OrderMeaning,
                    #[serde(rename = "_orderMeaning")]
                    OrderMeaningPrimitiveElement,
                    #[serde(rename = "fixedBase64Binary")]
                    FixedBase64Binary,
                    #[serde(rename = "_fixedBase64Binary")]
                    FixedBase64BinaryPrimitiveElement,
                    #[serde(rename = "fixedBoolean")]
                    FixedBoolean,
                    #[serde(rename = "_fixedBoolean")]
                    FixedBooleanPrimitiveElement,
                    #[serde(rename = "fixedCanonical")]
                    FixedCanonical,
                    #[serde(rename = "_fixedCanonical")]
                    FixedCanonicalPrimitiveElement,
                    #[serde(rename = "fixedCode")]
                    FixedCode,
                    #[serde(rename = "_fixedCode")]
                    FixedCodePrimitiveElement,
                    #[serde(rename = "fixedDate")]
                    FixedDate,
                    #[serde(rename = "_fixedDate")]
                    FixedDatePrimitiveElement,
                    #[serde(rename = "fixedDateTime")]
                    FixedDateTime,
                    #[serde(rename = "_fixedDateTime")]
                    FixedDateTimePrimitiveElement,
                    #[serde(rename = "fixedDecimal")]
                    FixedDecimal,
                    #[serde(rename = "_fixedDecimal")]
                    FixedDecimalPrimitiveElement,
                    #[serde(rename = "fixedId")]
                    FixedId,
                    #[serde(rename = "_fixedId")]
                    FixedIdPrimitiveElement,
                    #[serde(rename = "fixedInstant")]
                    FixedInstant,
                    #[serde(rename = "_fixedInstant")]
                    FixedInstantPrimitiveElement,
                    #[serde(rename = "fixedInteger")]
                    FixedInteger,
                    #[serde(rename = "_fixedInteger")]
                    FixedIntegerPrimitiveElement,
                    #[serde(rename = "fixedMarkdown")]
                    FixedMarkdown,
                    #[serde(rename = "_fixedMarkdown")]
                    FixedMarkdownPrimitiveElement,
                    #[serde(rename = "fixedOid")]
                    FixedOid,
                    #[serde(rename = "_fixedOid")]
                    FixedOidPrimitiveElement,
                    #[serde(rename = "fixedPositiveInt")]
                    FixedPositiveInt,
                    #[serde(rename = "_fixedPositiveInt")]
                    FixedPositiveIntPrimitiveElement,
                    #[serde(rename = "fixedString")]
                    FixedString,
                    #[serde(rename = "_fixedString")]
                    FixedStringPrimitiveElement,
                    #[serde(rename = "fixedTime")]
                    FixedTime,
                    #[serde(rename = "_fixedTime")]
                    FixedTimePrimitiveElement,
                    #[serde(rename = "fixedUnsignedInt")]
                    FixedUnsignedInt,
                    #[serde(rename = "_fixedUnsignedInt")]
                    FixedUnsignedIntPrimitiveElement,
                    #[serde(rename = "fixedUri")]
                    FixedUri,
                    #[serde(rename = "_fixedUri")]
                    FixedUriPrimitiveElement,
                    #[serde(rename = "fixedUrl")]
                    FixedUrl,
                    #[serde(rename = "_fixedUrl")]
                    FixedUrlPrimitiveElement,
                    #[serde(rename = "fixedUuid")]
                    FixedUuid,
                    #[serde(rename = "_fixedUuid")]
                    FixedUuidPrimitiveElement,
                    #[serde(rename = "fixedAddress")]
                    FixedAddress,
                    #[serde(rename = "fixedAge")]
                    FixedAge,
                    #[serde(rename = "fixedAnnotation")]
                    FixedAnnotation,
                    #[serde(rename = "fixedAttachment")]
                    FixedAttachment,
                    #[serde(rename = "fixedCodeableConcept")]
                    FixedCodeableConcept,
                    #[serde(rename = "fixedCodeableReference")]
                    FixedCodeableReference,
                    #[serde(rename = "fixedCoding")]
                    FixedCoding,
                    #[serde(rename = "fixedContactPoint")]
                    FixedContactPoint,
                    #[serde(rename = "fixedCount")]
                    FixedCount,
                    #[serde(rename = "fixedDistance")]
                    FixedDistance,
                    #[serde(rename = "fixedDuration")]
                    FixedDuration,
                    #[serde(rename = "fixedHumanName")]
                    FixedHumanName,
                    #[serde(rename = "fixedIdentifier")]
                    FixedIdentifier,
                    #[serde(rename = "fixedMoney")]
                    FixedMoney,
                    #[serde(rename = "fixedPeriod")]
                    FixedPeriod,
                    #[serde(rename = "fixedQuantity")]
                    FixedQuantity,
                    #[serde(rename = "fixedRange")]
                    FixedRange,
                    #[serde(rename = "fixedRatio")]
                    FixedRatio,
                    #[serde(rename = "fixedRatioRange")]
                    FixedRatioRange,
                    #[serde(rename = "fixedReference")]
                    FixedReference,
                    #[serde(rename = "fixedSampledData")]
                    FixedSampledData,
                    #[serde(rename = "fixedSignature")]
                    FixedSignature,
                    #[serde(rename = "fixedTiming")]
                    FixedTiming,
                    #[serde(rename = "fixedContactDetail")]
                    FixedContactDetail,
                    #[serde(rename = "fixedContributor")]
                    FixedContributor,
                    #[serde(rename = "fixedDataRequirement")]
                    FixedDataRequirement,
                    #[serde(rename = "fixedExpression")]
                    FixedExpression,
                    #[serde(rename = "fixedParameterDefinition")]
                    FixedParameterDefinition,
                    #[serde(rename = "fixedRelatedArtifact")]
                    FixedRelatedArtifact,
                    #[serde(rename = "fixedTriggerDefinition")]
                    FixedTriggerDefinition,
                    #[serde(rename = "fixedUsageContext")]
                    FixedUsageContext,
                    #[serde(rename = "fixedDosage")]
                    FixedDosage,
                    #[serde(rename = "patternBase64Binary")]
                    PatternBase64Binary,
                    #[serde(rename = "_patternBase64Binary")]
                    PatternBase64BinaryPrimitiveElement,
                    #[serde(rename = "patternBoolean")]
                    PatternBoolean,
                    #[serde(rename = "_patternBoolean")]
                    PatternBooleanPrimitiveElement,
                    #[serde(rename = "patternCanonical")]
                    PatternCanonical,
                    #[serde(rename = "_patternCanonical")]
                    PatternCanonicalPrimitiveElement,
                    #[serde(rename = "patternCode")]
                    PatternCode,
                    #[serde(rename = "_patternCode")]
                    PatternCodePrimitiveElement,
                    #[serde(rename = "patternDate")]
                    PatternDate,
                    #[serde(rename = "_patternDate")]
                    PatternDatePrimitiveElement,
                    #[serde(rename = "patternDateTime")]
                    PatternDateTime,
                    #[serde(rename = "_patternDateTime")]
                    PatternDateTimePrimitiveElement,
                    #[serde(rename = "patternDecimal")]
                    PatternDecimal,
                    #[serde(rename = "_patternDecimal")]
                    PatternDecimalPrimitiveElement,
                    #[serde(rename = "patternId")]
                    PatternId,
                    #[serde(rename = "_patternId")]
                    PatternIdPrimitiveElement,
                    #[serde(rename = "patternInstant")]
                    PatternInstant,
                    #[serde(rename = "_patternInstant")]
                    PatternInstantPrimitiveElement,
                    #[serde(rename = "patternInteger")]
                    PatternInteger,
                    #[serde(rename = "_patternInteger")]
                    PatternIntegerPrimitiveElement,
                    #[serde(rename = "patternMarkdown")]
                    PatternMarkdown,
                    #[serde(rename = "_patternMarkdown")]
                    PatternMarkdownPrimitiveElement,
                    #[serde(rename = "patternOid")]
                    PatternOid,
                    #[serde(rename = "_patternOid")]
                    PatternOidPrimitiveElement,
                    #[serde(rename = "patternPositiveInt")]
                    PatternPositiveInt,
                    #[serde(rename = "_patternPositiveInt")]
                    PatternPositiveIntPrimitiveElement,
                    #[serde(rename = "patternString")]
                    PatternString,
                    #[serde(rename = "_patternString")]
                    PatternStringPrimitiveElement,
                    #[serde(rename = "patternTime")]
                    PatternTime,
                    #[serde(rename = "_patternTime")]
                    PatternTimePrimitiveElement,
                    #[serde(rename = "patternUnsignedInt")]
                    PatternUnsignedInt,
                    #[serde(rename = "_patternUnsignedInt")]
                    PatternUnsignedIntPrimitiveElement,
                    #[serde(rename = "patternUri")]
                    PatternUri,
                    #[serde(rename = "_patternUri")]
                    PatternUriPrimitiveElement,
                    #[serde(rename = "patternUrl")]
                    PatternUrl,
                    #[serde(rename = "_patternUrl")]
                    PatternUrlPrimitiveElement,
                    #[serde(rename = "patternUuid")]
                    PatternUuid,
                    #[serde(rename = "_patternUuid")]
                    PatternUuidPrimitiveElement,
                    #[serde(rename = "patternAddress")]
                    PatternAddress,
                    #[serde(rename = "patternAge")]
                    PatternAge,
                    #[serde(rename = "patternAnnotation")]
                    PatternAnnotation,
                    #[serde(rename = "patternAttachment")]
                    PatternAttachment,
                    #[serde(rename = "patternCodeableConcept")]
                    PatternCodeableConcept,
                    #[serde(rename = "patternCodeableReference")]
                    PatternCodeableReference,
                    #[serde(rename = "patternCoding")]
                    PatternCoding,
                    #[serde(rename = "patternContactPoint")]
                    PatternContactPoint,
                    #[serde(rename = "patternCount")]
                    PatternCount,
                    #[serde(rename = "patternDistance")]
                    PatternDistance,
                    #[serde(rename = "patternDuration")]
                    PatternDuration,
                    #[serde(rename = "patternHumanName")]
                    PatternHumanName,
                    #[serde(rename = "patternIdentifier")]
                    PatternIdentifier,
                    #[serde(rename = "patternMoney")]
                    PatternMoney,
                    #[serde(rename = "patternPeriod")]
                    PatternPeriod,
                    #[serde(rename = "patternQuantity")]
                    PatternQuantity,
                    #[serde(rename = "patternRange")]
                    PatternRange,
                    #[serde(rename = "patternRatio")]
                    PatternRatio,
                    #[serde(rename = "patternRatioRange")]
                    PatternRatioRange,
                    #[serde(rename = "patternReference")]
                    PatternReference,
                    #[serde(rename = "patternSampledData")]
                    PatternSampledData,
                    #[serde(rename = "patternSignature")]
                    PatternSignature,
                    #[serde(rename = "patternTiming")]
                    PatternTiming,
                    #[serde(rename = "patternContactDetail")]
                    PatternContactDetail,
                    #[serde(rename = "patternContributor")]
                    PatternContributor,
                    #[serde(rename = "patternDataRequirement")]
                    PatternDataRequirement,
                    #[serde(rename = "patternExpression")]
                    PatternExpression,
                    #[serde(rename = "patternParameterDefinition")]
                    PatternParameterDefinition,
                    #[serde(rename = "patternRelatedArtifact")]
                    PatternRelatedArtifact,
                    #[serde(rename = "patternTriggerDefinition")]
                    PatternTriggerDefinition,
                    #[serde(rename = "patternUsageContext")]
                    PatternUsageContext,
                    #[serde(rename = "patternDosage")]
                    PatternDosage,
                    #[serde(rename = "example")]
                    Example,
                    #[serde(rename = "minValueDate")]
                    MinValueDate,
                    #[serde(rename = "_minValueDate")]
                    MinValueDatePrimitiveElement,
                    #[serde(rename = "minValueDateTime")]
                    MinValueDateTime,
                    #[serde(rename = "_minValueDateTime")]
                    MinValueDateTimePrimitiveElement,
                    #[serde(rename = "minValueInstant")]
                    MinValueInstant,
                    #[serde(rename = "_minValueInstant")]
                    MinValueInstantPrimitiveElement,
                    #[serde(rename = "minValueTime")]
                    MinValueTime,
                    #[serde(rename = "_minValueTime")]
                    MinValueTimePrimitiveElement,
                    #[serde(rename = "minValueDecimal")]
                    MinValueDecimal,
                    #[serde(rename = "_minValueDecimal")]
                    MinValueDecimalPrimitiveElement,
                    #[serde(rename = "minValueInteger")]
                    MinValueInteger,
                    #[serde(rename = "_minValueInteger")]
                    MinValueIntegerPrimitiveElement,
                    #[serde(rename = "minValuePositiveInt")]
                    MinValuePositiveInt,
                    #[serde(rename = "_minValuePositiveInt")]
                    MinValuePositiveIntPrimitiveElement,
                    #[serde(rename = "minValueUnsignedInt")]
                    MinValueUnsignedInt,
                    #[serde(rename = "_minValueUnsignedInt")]
                    MinValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "minValueQuantity")]
                    MinValueQuantity,
                    #[serde(rename = "maxValueDate")]
                    MaxValueDate,
                    #[serde(rename = "_maxValueDate")]
                    MaxValueDatePrimitiveElement,
                    #[serde(rename = "maxValueDateTime")]
                    MaxValueDateTime,
                    #[serde(rename = "_maxValueDateTime")]
                    MaxValueDateTimePrimitiveElement,
                    #[serde(rename = "maxValueInstant")]
                    MaxValueInstant,
                    #[serde(rename = "_maxValueInstant")]
                    MaxValueInstantPrimitiveElement,
                    #[serde(rename = "maxValueTime")]
                    MaxValueTime,
                    #[serde(rename = "_maxValueTime")]
                    MaxValueTimePrimitiveElement,
                    #[serde(rename = "maxValueDecimal")]
                    MaxValueDecimal,
                    #[serde(rename = "_maxValueDecimal")]
                    MaxValueDecimalPrimitiveElement,
                    #[serde(rename = "maxValueInteger")]
                    MaxValueInteger,
                    #[serde(rename = "_maxValueInteger")]
                    MaxValueIntegerPrimitiveElement,
                    #[serde(rename = "maxValuePositiveInt")]
                    MaxValuePositiveInt,
                    #[serde(rename = "_maxValuePositiveInt")]
                    MaxValuePositiveIntPrimitiveElement,
                    #[serde(rename = "maxValueUnsignedInt")]
                    MaxValueUnsignedInt,
                    #[serde(rename = "_maxValueUnsignedInt")]
                    MaxValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "maxValueQuantity")]
                    MaxValueQuantity,
                    #[serde(rename = "maxLength")]
                    MaxLength,
                    #[serde(rename = "_maxLength")]
                    MaxLengthPrimitiveElement,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "_condition")]
                    ConditionPrimitiveElement,
                    #[serde(rename = "constraint")]
                    Constraint,
                    #[serde(rename = "mustSupport")]
                    MustSupport,
                    #[serde(rename = "_mustSupport")]
                    MustSupportPrimitiveElement,
                    #[serde(rename = "isModifier")]
                    IsModifier,
                    #[serde(rename = "_isModifier")]
                    IsModifierPrimitiveElement,
                    #[serde(rename = "isModifierReason")]
                    IsModifierReason,
                    #[serde(rename = "_isModifierReason")]
                    IsModifierReasonPrimitiveElement,
                    #[serde(rename = "isSummary")]
                    IsSummary,
                    #[serde(rename = "_isSummary")]
                    IsSummaryPrimitiveElement,
                    #[serde(rename = "binding")]
                    Binding,
                    #[serde(rename = "mapping")]
                    Mapping,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "path",
                            "representation",
                            "sliceName",
                            "sliceIsConstraining",
                            "label",
                            "code",
                            "slicing",
                            "short",
                            "definition",
                            "comment",
                            "requirements",
                            "alias",
                            "min",
                            "max",
                            "base",
                            "contentReference",
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
                            "defaultValueCodeableReference",
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
                            "defaultValueRatioRange",
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
                            "meaningWhenMissing",
                            "orderMeaning",
                            "fixedBase64Binary",
                            "fixedBoolean",
                            "fixedCanonical",
                            "fixedCode",
                            "fixedDate",
                            "fixedDateTime",
                            "fixedDecimal",
                            "fixedId",
                            "fixedInstant",
                            "fixedInteger",
                            "fixedMarkdown",
                            "fixedOid",
                            "fixedPositiveInt",
                            "fixedString",
                            "fixedTime",
                            "fixedUnsignedInt",
                            "fixedUri",
                            "fixedUrl",
                            "fixedUuid",
                            "fixedAddress",
                            "fixedAge",
                            "fixedAnnotation",
                            "fixedAttachment",
                            "fixedCodeableConcept",
                            "fixedCodeableReference",
                            "fixedCoding",
                            "fixedContactPoint",
                            "fixedCount",
                            "fixedDistance",
                            "fixedDuration",
                            "fixedHumanName",
                            "fixedIdentifier",
                            "fixedMoney",
                            "fixedPeriod",
                            "fixedQuantity",
                            "fixedRange",
                            "fixedRatio",
                            "fixedRatioRange",
                            "fixedReference",
                            "fixedSampledData",
                            "fixedSignature",
                            "fixedTiming",
                            "fixedContactDetail",
                            "fixedContributor",
                            "fixedDataRequirement",
                            "fixedExpression",
                            "fixedParameterDefinition",
                            "fixedRelatedArtifact",
                            "fixedTriggerDefinition",
                            "fixedUsageContext",
                            "fixedDosage",
                            "patternBase64Binary",
                            "patternBoolean",
                            "patternCanonical",
                            "patternCode",
                            "patternDate",
                            "patternDateTime",
                            "patternDecimal",
                            "patternId",
                            "patternInstant",
                            "patternInteger",
                            "patternMarkdown",
                            "patternOid",
                            "patternPositiveInt",
                            "patternString",
                            "patternTime",
                            "patternUnsignedInt",
                            "patternUri",
                            "patternUrl",
                            "patternUuid",
                            "patternAddress",
                            "patternAge",
                            "patternAnnotation",
                            "patternAttachment",
                            "patternCodeableConcept",
                            "patternCodeableReference",
                            "patternCoding",
                            "patternContactPoint",
                            "patternCount",
                            "patternDistance",
                            "patternDuration",
                            "patternHumanName",
                            "patternIdentifier",
                            "patternMoney",
                            "patternPeriod",
                            "patternQuantity",
                            "patternRange",
                            "patternRatio",
                            "patternRatioRange",
                            "patternReference",
                            "patternSampledData",
                            "patternSignature",
                            "patternTiming",
                            "patternContactDetail",
                            "patternContributor",
                            "patternDataRequirement",
                            "patternExpression",
                            "patternParameterDefinition",
                            "patternRelatedArtifact",
                            "patternTriggerDefinition",
                            "patternUsageContext",
                            "patternDosage",
                            "example",
                            "minValueDate",
                            "minValueDateTime",
                            "minValueInstant",
                            "minValueTime",
                            "minValueDecimal",
                            "minValueInteger",
                            "minValuePositiveInt",
                            "minValueUnsignedInt",
                            "minValueQuantity",
                            "maxValueDate",
                            "maxValueDateTime",
                            "maxValueInstant",
                            "maxValueTime",
                            "maxValueDecimal",
                            "maxValueInteger",
                            "maxValuePositiveInt",
                            "maxValueUnsignedInt",
                            "maxValueQuantity",
                            "maxLength",
                            "condition",
                            "constraint",
                            "mustSupport",
                            "isModifier",
                            "isModifierReason",
                            "isSummary",
                            "binding",
                            "mapping",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#representation: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#slice_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#slice_is_constraining: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#label: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#code: Option<Vec<Box<fhirbolt_model::r4b::types::Coding>>> = None;
                let mut r#slicing: Option<fhirbolt_model::r4b::types::ElementDefinitionSlicing> =
                    None;
                let mut r#short: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#definition: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#comment: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#requirements: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#alias: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
                let mut r#min: Option<fhirbolt_model::r4b::types::UnsignedInt> = None;
                let mut r#max: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#base: Option<fhirbolt_model::r4b::types::ElementDefinitionBase> = None;
                let mut r#content_reference: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r4b::types::ElementDefinitionType>> =
                    None;
                let mut r#default_value: Option<
                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue,
                > = None;
                let mut r#meaning_when_missing: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#order_meaning: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#fixed: Option<fhirbolt_model::r4b::types::ElementDefinitionFixed> = None;
                let mut r#pattern: Option<fhirbolt_model::r4b::types::ElementDefinitionPattern> =
                    None;
                let mut r#example: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
                > = None;
                let mut r#min_value: Option<fhirbolt_model::r4b::types::ElementDefinitionMinValue> =
                    None;
                let mut r#max_value: Option<fhirbolt_model::r4b::types::ElementDefinitionMaxValue> =
                    None;
                let mut r#max_length: Option<fhirbolt_model::r4b::types::Integer> = None;
                let mut r#condition: Option<Vec<fhirbolt_model::r4b::types::Id>> = None;
                let mut r#constraint: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
                > = None;
                let mut r#must_support: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#is_modifier: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#is_modifier_reason: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#is_summary: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#binding: Option<fhirbolt_model::r4b::types::ElementDefinitionBinding> =
                    None;
                let mut r#mapping: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
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
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                r#path = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::Representation => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#representation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "representation",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#representation.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::RepresentationPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#representation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_representation",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("representation");
                            }
                        }
                        Field::SliceName => {
                            if self.0.from_json {
                                let some = r#slice_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sliceName"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#slice_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sliceName"));
                                }
                                r#slice_name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::SliceNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#slice_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sliceName"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sliceName");
                            }
                        }
                        Field::SliceIsConstraining => {
                            if self.0.from_json {
                                let some =
                                    r#slice_is_constraining.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sliceIsConstraining",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#slice_is_constraining.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sliceIsConstraining",
                                    ));
                                }
                                r#slice_is_constraining = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::SliceIsConstrainingPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#slice_is_constraining.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_sliceIsConstraining",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sliceIsConstraining");
                            }
                        }
                        Field::Label => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                r#label = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Coding > >> ()) ?) ;
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                );
                            }
                        }
                        Field::Slicing => {
                            if r#slicing.is_some() {
                                return Err(serde::de::Error::duplicate_field("slicing"));
                            }
                            r#slicing = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicing > ()) ?) ;
                        }
                        Field::Short => {
                            if self.0.from_json {
                                let some = r#short.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("short"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#short.is_some() {
                                    return Err(serde::de::Error::duplicate_field("short"));
                                }
                                r#short = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::ShortPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#short.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_short"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("short");
                            }
                        }
                        Field::Definition => {
                            if self.0.from_json {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                r#definition = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::DefinitionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_definition"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("definition");
                            }
                        }
                        Field::Comment => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#comment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                r#comment = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comment"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comment");
                            }
                        }
                        Field::Requirements => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#requirements.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                r#requirements = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::RequirementsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requirements"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requirements");
                            }
                        }
                        Field::Alias => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#alias.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::AliasPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_alias"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("alias");
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
                                r#min = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::UnsignedInt>(),
                                    )?,
                                );
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
                        Field::Base => {
                            if r#base.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            r#base = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionBase > ()) ?) ;
                        }
                        Field::ContentReference => {
                            if self.0.from_json {
                                let some = r#content_reference.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#content_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                r#content_reference = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ContentReferencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#content_reference.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_contentReference",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contentReference");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4b::types::ElementDefinitionType,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionType > ()) ?) ;
                            }
                        }
                        Field::DefaultValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBase64Binary",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBase64Binary");
                            }
                        }
                        Field::DefaultValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBoolean",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueBoolean");
                            }
                        }
                        Field::DefaultValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCanonical",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCanonical");
                            }
                        }
                        Field::DefaultValueCode => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Code (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueCode")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCode",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Code (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueCode")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueCode");
                            }
                        }
                        Field::DefaultValueDate => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDate",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDate");
                            }
                        }
                        Field::DefaultValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDateTime",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDateTime");
                            }
                        }
                        Field::DefaultValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDecimal",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueDecimal");
                            }
                        }
                        Field::DefaultValueId => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Id (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueId")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueId",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Id(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Id (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueId")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueId");
                            }
                        }
                        Field::DefaultValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInstant",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Instant (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInstant");
                            }
                        }
                        Field::DefaultValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInteger",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueInteger");
                            }
                        }
                        Field::DefaultValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMarkdown",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueMarkdown");
                            }
                        }
                        Field::DefaultValueOid => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Oid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueOid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueOid",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Oid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueOid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueOid");
                            }
                        }
                        Field::DefaultValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePositiveInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValuePositiveInt");
                            }
                        }
                        Field::DefaultValueString => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueString",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueString");
                            }
                        }
                        Field::DefaultValueTime => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Time (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTime",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Time (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueTime");
                            }
                        }
                        Field::DefaultValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUnsignedInt",
                                    ));
                                }
                                r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::DefaultValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUnsignedInt");
                            }
                        }
                        Field::DefaultValueUri => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUri",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUri");
                            }
                        }
                        Field::DefaultValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Url (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUrl")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUrl",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Url (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUrl")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
                            } else {
                                return unknown_field_error("defaultValueUrl");
                            }
                        }
                        Field::DefaultValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Uuid (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("defaultValueUuid")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("defaultValue[x]")) ; }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUuid",
                                    ));
                                }
                                r#default_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::DefaultValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#default_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Uuid (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_defaultValueUuid")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_defaultValue[x]")) ; }
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
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueAge => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueAge"));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Age(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueAnnotation => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAnnotation",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::DefaultValueAttachment => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAttachment",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::DefaultValueCodeableConcept => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableConcept",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::DefaultValueCodeableReference => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableReference",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: CodeableReference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableReference > > ()) ?)) ;
                        }
                        Field::DefaultValueCoding => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCoding",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueContactPoint => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactPoint",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::DefaultValueCount => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueCount"));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Count(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueDistance => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDistance",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueDuration => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDuration",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueHumanName => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueHumanName",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::DefaultValueIdentifier => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueIdentifier",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::DefaultValueMoney => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMoney"));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Money(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValuePeriod => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValuePeriod",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueQuantity => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueQuantity",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueRange => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRange"));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueRatio => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRatio"));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Ratio(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueRatioRange => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRatioRange",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: RatioRange (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RatioRange > > ()) ?)) ;
                        }
                        Field::DefaultValueReference => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueReference",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::DefaultValueSampledData => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSampledData",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::DefaultValueSignature => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSignature",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::DefaultValueTiming => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTiming",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::DefaultValueContactDetail => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactDetail",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::DefaultValueContributor => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContributor",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::DefaultValueDataRequirement => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDataRequirement",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::DefaultValueExpression => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueExpression",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::DefaultValueParameterDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueParameterDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueRelatedArtifact => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRelatedArtifact",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::DefaultValueTriggerDefinition => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTriggerDefinition",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::DefaultValueUsageContext => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueUsageContext",
                                ));
                            }
                            r#default_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionDefaultValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::DefaultValueDosage => {
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDosage",
                                ));
                            }
                            r#default_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionDefaultValue::Dosage(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::MeaningWhenMissing => {
                            if self.0.from_json {
                                let some = r#meaning_when_missing.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "meaningWhenMissing",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#meaning_when_missing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "meaningWhenMissing",
                                    ));
                                }
                                r#meaning_when_missing = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::MeaningWhenMissingPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#meaning_when_missing.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_meaningWhenMissing",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("meaningWhenMissing");
                            }
                        }
                        Field::OrderMeaning => {
                            if self.0.from_json {
                                let some = r#order_meaning.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderMeaning"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#order_meaning.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderMeaning"));
                                }
                                r#order_meaning = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::OrderMeaningPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#order_meaning.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_orderMeaning"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("orderMeaning");
                            }
                        }
                        Field::FixedBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#fixed . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedBase64Binary",
                                    ));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::FixedBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedBase64Binary");
                            }
                        }
                        Field::FixedBoolean => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedBoolean"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::FixedBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedBoolean");
                            }
                        }
                        Field::FixedCanonical => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedCanonical",
                                    ));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::FixedCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedCanonical");
                            }
                        }
                        Field::FixedCode => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedCode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedCode"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedCode",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedCode");
                            }
                        }
                        Field::FixedDate => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDate"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedDate");
                            }
                        }
                        Field::FixedDateTime => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDateTime"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::FixedDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedDateTime");
                            }
                        }
                        Field::FixedDecimal => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDecimal"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::FixedDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedDecimal",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedDecimal");
                            }
                        }
                        Field::FixedId => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedId"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedId"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedId");
                            }
                        }
                        Field::FixedInstant => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedInstant",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInstant"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::FixedInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedInstant",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedInstant");
                            }
                        }
                        Field::FixedInteger => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInteger"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::FixedIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedInteger");
                            }
                        }
                        Field::FixedMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedMarkdown"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::FixedMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedMarkdown");
                            }
                        }
                        Field::FixedOid => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedOid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedOid"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedOid"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedOid");
                            }
                        }
                        Field::FixedPositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedPositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedPositiveInt",
                                    ));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::FixedPositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedPositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedPositiveInt");
                            }
                        }
                        Field::FixedString => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedString"));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::FixedStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedString");
                            }
                        }
                        Field::FixedTime => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedTime"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedTime"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedTime");
                            }
                        }
                        Field::FixedUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("fixedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("fixed[x]")) ; }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedUnsignedInt",
                                    ));
                                }
                                r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::FixedUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_fixedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_fixed[x]")) ; }
                            } else {
                                return unknown_field_error("fixedUnsignedInt");
                            }
                        }
                        Field::FixedUri => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUri"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedUri"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUri");
                            }
                        }
                        Field::FixedUrl => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUrl"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUrl"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedUrl"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUrl");
                            }
                        }
                        Field::FixedUuid => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUuid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUuid"));
                                }
                                r#fixed =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::FixedUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionFixed::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedUuid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUuid");
                            }
                        }
                        Field::FixedAddress => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAddress"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::FixedAge => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAge"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Age(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                    )?,
                                ));
                        }
                        Field::FixedAnnotation => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAnnotation"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::FixedAttachment => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAttachment"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::FixedCodeableConcept => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedCodeableConcept",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::FixedCodeableReference => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedCodeableReference",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: CodeableReference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableReference > > ()) ?)) ;
                        }
                        Field::FixedCoding => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCoding"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                ));
                        }
                        Field::FixedContactPoint => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContactPoint"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::FixedCount => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCount"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Count(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                    )?,
                                ));
                        }
                        Field::FixedDistance => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDistance"));
                            }
                            r#fixed = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionFixed::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::FixedDuration => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDuration"));
                            }
                            r#fixed = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionFixed::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::FixedHumanName => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedHumanName"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::FixedIdentifier => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedIdentifier"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::FixedMoney => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedMoney"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Money(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                    )?,
                                ));
                        }
                        Field::FixedPeriod => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedPeriod"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ));
                        }
                        Field::FixedQuantity => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedQuantity"));
                            }
                            r#fixed = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionFixed::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::FixedRange => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRange"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::FixedRatio => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRatio"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Ratio(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                    )?,
                                ));
                        }
                        Field::FixedRatioRange => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRatioRange"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: RatioRange (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RatioRange > > ()) ?)) ;
                        }
                        Field::FixedReference => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedReference"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::FixedSampledData => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSampledData"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::FixedSignature => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSignature"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::FixedTiming => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedTiming"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                    )?,
                                ));
                        }
                        Field::FixedContactDetail => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedContactDetail",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::FixedContributor => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContributor"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::FixedDataRequirement => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedDataRequirement",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::FixedExpression => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedExpression"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::FixedParameterDefinition => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedParameterDefinition",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::FixedRelatedArtifact => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedRelatedArtifact",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::FixedTriggerDefinition => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedTriggerDefinition",
                                ));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::FixedUsageContext => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedUsageContext"));
                            }
                            r#fixed = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionFixed :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::FixedDosage => {
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDosage"));
                            }
                            r#fixed =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionFixed::Dosage(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                ));
                        }
                        Field::PatternBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBase64Binary",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::PatternBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternBase64Binary");
                            }
                        }
                        Field::PatternBoolean => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBoolean",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::PatternBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternBoolean");
                            }
                        }
                        Field::PatternCanonical => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternCanonical",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::PatternCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternCanonical");
                            }
                        }
                        Field::PatternCode => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternCode",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternCode"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternCode",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternCode");
                            }
                        }
                        Field::PatternDate => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternDate"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternDate");
                            }
                        }
                        Field::PatternDateTime => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDateTime",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::PatternDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternDateTime");
                            }
                        }
                        Field::PatternDecimal => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDecimal",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::PatternDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternDecimal");
                            }
                        }
                        Field::PatternId => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("patternId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternId"));
                                }
                                r#pattern =
                                    Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                        )?,
                                    ));
                            }
                        }
                        Field::PatternIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternId",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternId");
                            }
                        }
                        Field::PatternInstant => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInstant",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::PatternInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternInstant");
                            }
                        }
                        Field::PatternInteger => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInteger",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::PatternIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternInteger");
                            }
                        }
                        Field::PatternMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternMarkdown",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::PatternMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternMarkdown");
                            }
                        }
                        Field::PatternOid => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternOid",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternOid"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternOid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternOid");
                            }
                        }
                        Field::PatternPositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternPositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternPositiveInt",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::PatternPositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternPositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternPositiveInt");
                            }
                        }
                        Field::PatternString => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternString"));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::PatternStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternString");
                            }
                        }
                        Field::PatternTime => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternTime"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternTime");
                            }
                        }
                        Field::PatternUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("patternUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("pattern[x]")) ; }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternUnsignedInt",
                                    ));
                                }
                                r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::PatternUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_patternUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_pattern[x]")) ; }
                            } else {
                                return unknown_field_error("patternUnsignedInt");
                            }
                        }
                        Field::PatternUri => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUri",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUri"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUri",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUri");
                            }
                        }
                        Field::PatternUrl => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUrl",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUrl"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUrl",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUrl");
                            }
                        }
                        Field::PatternUuid => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUuid",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUuid"));
                                }
                                r#pattern = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::PatternUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionPattern::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUuid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUuid");
                            }
                        }
                        Field::PatternAddress => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAddress"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternAge => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAge"));
                            }
                            r#pattern =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Age(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                    )?,
                                ));
                        }
                        Field::PatternAnnotation => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAnnotation"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::PatternAttachment => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAttachment"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::PatternCodeableConcept => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternCodeableConcept",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::PatternCodeableReference => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternCodeableReference",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: CodeableReference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableReference > > ()) ?)) ;
                        }
                        Field::PatternCoding => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCoding"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternContactPoint => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactPoint",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::PatternCount => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCount"));
                            }
                            r#pattern =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Count(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                    )?,
                                ));
                        }
                        Field::PatternDistance => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDistance"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternDuration => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDuration"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternHumanName => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternHumanName"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::PatternIdentifier => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternIdentifier"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::PatternMoney => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternMoney"));
                            }
                            r#pattern =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Money(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                    )?,
                                ));
                        }
                        Field::PatternPeriod => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternPeriod"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternQuantity => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternQuantity"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternRange => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRange"));
                            }
                            r#pattern =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::PatternRatio => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRatio"));
                            }
                            r#pattern =
                                Some(fhirbolt_model::r4b::types::ElementDefinitionPattern::Ratio(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                    )?,
                                ));
                        }
                        Field::PatternRatioRange => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRatioRange"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: RatioRange (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RatioRange > > ()) ?)) ;
                        }
                        Field::PatternReference => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternReference"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::PatternSampledData => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternSampledData",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::PatternSignature => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternSignature"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::PatternTiming => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternTiming"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::PatternContactDetail => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactDetail",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::PatternContributor => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContributor",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::PatternDataRequirement => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternDataRequirement",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::PatternExpression => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternExpression"));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::PatternParameterDefinition => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternParameterDefinition",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::PatternRelatedArtifact => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternRelatedArtifact",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::PatternTriggerDefinition => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternTriggerDefinition",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::PatternUsageContext => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternUsageContext",
                                ));
                            }
                            r#pattern = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionPattern :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::PatternDosage => {
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDosage"));
                            }
                            r#pattern = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionPattern::Dosage(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::Example => {
                            if self.0.from_json {
                                if r#example.is_some() {
                                    return Err(serde::de::Error::duplicate_field("example"));
                                }
                                r#example =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4b::types::ElementDefinitionExample,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#example.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionExample > ()) ?) ;
                            }
                        }
                        Field::MinValueDate => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueDate"));
                                }
                                r#min_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::MinValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMinValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueDate");
                            }
                        }
                        Field::MinValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDateTime",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::MinValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValueDateTime");
                            }
                        }
                        Field::MinValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInstant",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::MinValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValueInstant");
                            }
                        }
                        Field::MinValueTime => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueTime"));
                                }
                                r#min_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::MinValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMinValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueTime");
                            }
                        }
                        Field::MinValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDecimal",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::MinValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValueDecimal");
                            }
                        }
                        Field::MinValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInteger",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::MinValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMinValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValueInteger");
                            }
                        }
                        Field::MinValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#min_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValuePositiveInt",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::MinValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValuePositiveInt");
                            }
                        }
                        Field::MinValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#min_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minValueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minValue[x]")) ; }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueUnsignedInt",
                                    ));
                                }
                                r#min_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::MinValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#min_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMinValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minValueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minValue[x]")) ; }
                            } else {
                                return unknown_field_error("minValueUnsignedInt");
                            }
                        }
                        Field::MinValueQuantity => {
                            if r#min_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("minValueQuantity"));
                            }
                            r#min_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionMinValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::MaxValueDate => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueDate"));
                                }
                                r#max_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::MaxValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueDate");
                            }
                        }
                        Field::MaxValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDateTime",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::MaxValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValueDateTime");
                            }
                        }
                        Field::MaxValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Instant (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValueInstant")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInstant",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::MaxValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Instant (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValueInstant")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValueInstant");
                            }
                        }
                        Field::MaxValueTime => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueTime"));
                                }
                                r#max_value = Some(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::MaxValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueTime");
                            }
                        }
                        Field::MaxValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDecimal",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::MaxValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValueDecimal");
                            }
                        }
                        Field::MaxValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInteger",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::MaxValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value.get_or_insert(
                                    fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValueInteger");
                            }
                        }
                        Field::MaxValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#max_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValuePositiveInt",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::MaxValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: PositiveInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValuePositiveInt");
                            }
                        }
                        Field::MaxValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#max_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("maxValueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("maxValue[x]")) ; }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueUnsignedInt",
                                    ));
                                }
                                r#max_value = Some (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::MaxValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#max_value . get_or_insert (fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: types :: ElementDefinitionMaxValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_maxValueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_maxValue[x]")) ; }
                            } else {
                                return unknown_field_error("maxValueUnsignedInt");
                            }
                        }
                        Field::MaxValueQuantity => {
                            if r#max_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValueQuantity"));
                            }
                            r#max_value = Some(
                                fhirbolt_model::r4b::types::ElementDefinitionMaxValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::MaxLength => {
                            if self.0.from_json {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxLength"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#max_length.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxLength"));
                                }
                                r#max_length = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Integer>(),
                                )?);
                            }
                        }
                        Field::MaxLengthPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_maxLength"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("maxLength");
                            }
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#condition.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Id>(),
                                )?);
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#condition.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("condition");
                            }
                        }
                        Field::Constraint => {
                            if self.0.from_json {
                                if r#constraint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("constraint"));
                                }
                                r#constraint = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::types::ElementDefinitionConstraint,
                                    >>(),
                                )?);
                            } else {
                                let vec = r#constraint.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionConstraint > ()) ?) ;
                            }
                        }
                        Field::MustSupport => {
                            if self.0.from_json {
                                let some = r#must_support.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#must_support.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                r#must_support = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::MustSupportPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#must_support.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mustSupport"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("mustSupport");
                            }
                        }
                        Field::IsModifier => {
                            if self.0.from_json {
                                let some = r#is_modifier.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isModifier"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#is_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isModifier"));
                                }
                                r#is_modifier = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::IsModifierPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_modifier.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isModifier"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isModifier");
                            }
                        }
                        Field::IsModifierReason => {
                            if self.0.from_json {
                                let some = r#is_modifier_reason.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "isModifierReason",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#is_modifier_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "isModifierReason",
                                    ));
                                }
                                r#is_modifier_reason = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::IsModifierReasonPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_modifier_reason.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_isModifierReason",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isModifierReason");
                            }
                        }
                        Field::IsSummary => {
                            if self.0.from_json {
                                let some = r#is_summary.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSummary"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#is_summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSummary"));
                                }
                                r#is_summary = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::IsSummaryPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_summary.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isSummary"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isSummary");
                            }
                        }
                        Field::Binding => {
                            if r#binding.is_some() {
                                return Err(serde::de::Error::duplicate_field("binding"));
                            }
                            r#binding = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionBinding > ()) ?) ;
                        }
                        Field::Mapping => {
                            if self.0.from_json {
                                if r#mapping.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mapping"));
                                }
                                r#mapping =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4b::types::ElementDefinitionMapping,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#mapping.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: ElementDefinitionMapping > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::types::ElementDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        r#path.ok_or(serde::de::Error::missing_field("path"))?
                    },
                    r#representation: r#representation.unwrap_or(vec![]),
                    r#slice_name,
                    r#slice_is_constraining,
                    r#label,
                    r#code: r#code.unwrap_or(vec![]),
                    r#slicing,
                    r#short,
                    r#definition,
                    r#comment,
                    r#requirements,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#min,
                    r#max,
                    r#base,
                    r#content_reference,
                    r#type: r#type.unwrap_or(vec![]),
                    r#default_value,
                    r#meaning_when_missing,
                    r#order_meaning,
                    r#fixed,
                    r#pattern,
                    r#example: r#example.unwrap_or(vec![]),
                    r#min_value,
                    r#max_value,
                    r#max_length,
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#constraint: r#constraint.unwrap_or(vec![]),
                    r#must_support,
                    r#is_modifier,
                    r#is_modifier_reason,
                    r#is_summary,
                    r#binding,
                    r#mapping: r#mapping.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::ElementDefinition>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::ElementDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::ElementDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::ElementDefinition>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::ElementDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::ElementDefinition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::ElementDefinition>;
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
                        .transmute::<fhirbolt_model::r4b::types::ElementDefinition>(),
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
        Vec<Box<fhirbolt_model::r4b::types::ElementDefinition>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinition>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::ElementDefinition>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::ElementDefinition>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::types::ElementDefinition>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
