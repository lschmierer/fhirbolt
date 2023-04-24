// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionCondition,
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
                "RequestOrchestration.action.condition", field
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
            if self.value.r#kind.id.as_deref() == Some("$invalid") {
                return missing_field_error("kind");
            }
            if let Some(some) = self.value.r#kind.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("kind", &some)?;
            }
            if self.value.r#kind.id.is_some() || !self.value.r#kind.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#kind.id.as_ref(),
                    extension: &self.value.r#kind.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_kind", ctx)
                })?;
            }
        } else {
            if self.value.r#kind.id.as_deref() == Some("$invalid") {
                return missing_field_error("kind");
            }
            self.with_context(&self.value.r#kind, |ctx| state.serialize_entry("kind", ctx))?;
        }
        if let Some(some) = self.value.r#expression.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionCondition,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionCondition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionCondition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionCondition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionCondition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition, V::Error>
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
                    #[serde(rename = "kind")]
                    Kind,
                    #[serde(rename = "_kind")]
                    KindPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "kind", "expression"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#kind: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#expression: Option<Box<fhirbolt_model::r5::types::Expression>> = None;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Kind => {
                            if self.0.from_json {
                                let some = r#kind.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("kind"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#kind.is_some() {
                                    return Err(serde::de::Error::duplicate_field("kind"));
                                }
                                r#kind = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::KindPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#kind.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_kind"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("kind");
                            }
                        }
                        Field::Expression => {
                            if r#expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            r#expression = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Expression>>(),
                                )?,
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
                Ok(
                    fhirbolt_model::r5::resources::RequestOrchestrationActionCondition {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#kind: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#kind.unwrap_or(Default::default())
                        } else {
                            r#kind.ok_or(serde::de::Error::missing_field("kind"))?
                        },
                        r#expression,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionCondition > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionInput,
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
                "RequestOrchestration.action.input", field
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
        if let Some(some) = self.value.r#requirement.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requirement", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#related_data.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("relatedData", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_relatedData", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#related_data.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("relatedData", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionInput,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionInput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionInput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionInput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::RequestOrchestrationActionInput, V::Error>
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
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "requirement")]
                    Requirement,
                    #[serde(rename = "relatedData")]
                    RelatedData,
                    #[serde(rename = "_relatedData")]
                    RelatedDataPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "title",
                            "requirement",
                            "relatedData",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#requirement: Option<Box<fhirbolt_model::r5::types::DataRequirement>> =
                    None;
                let mut r#related_data: Option<fhirbolt_model::r5::types::Id> = None;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                        Field::Requirement => {
                            if r#requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirement"));
                            }
                            r#requirement = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: DataRequirement > > ()) ?) ;
                        }
                        Field::RelatedData => {
                            if self.0.from_json {
                                let some = r#related_data.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedData"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#related_data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedData"));
                                }
                                r#related_data = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Id>(),
                                )?);
                            }
                        }
                        Field::RelatedDataPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#related_data.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_relatedData"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("relatedData");
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
                    fhirbolt_model::r5::resources::RequestOrchestrationActionInput {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#title,
                        r#requirement,
                        r#related_data,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionInput > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionOutput,
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
                "RequestOrchestration.action.output", field
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
        if let Some(some) = self.value.r#requirement.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requirement", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#related_data.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("relatedData", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_relatedData", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#related_data.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("relatedData", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionOutput,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionOutput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionOutput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionOutput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput, V::Error>
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
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "requirement")]
                    Requirement,
                    #[serde(rename = "relatedData")]
                    RelatedData,
                    #[serde(rename = "_relatedData")]
                    RelatedDataPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "title",
                            "requirement",
                            "relatedData",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#requirement: Option<Box<fhirbolt_model::r5::types::DataRequirement>> =
                    None;
                let mut r#related_data: Option<fhirbolt_model::r5::types::String> = None;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                        Field::Requirement => {
                            if r#requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirement"));
                            }
                            r#requirement = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: DataRequirement > > ()) ?) ;
                        }
                        Field::RelatedData => {
                            if self.0.from_json {
                                let some = r#related_data.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedData"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#related_data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedData"));
                                }
                                r#related_data = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::RelatedDataPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#related_data.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_relatedData"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("relatedData");
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
                    fhirbolt_model::r5::resources::RequestOrchestrationActionOutput {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#title,
                        r#requirement,
                        r#related_data,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionOutput > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction,
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
                "RequestOrchestration.action.relatedAction", field
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
            if self.value.r#target_id.id.as_deref() == Some("$invalid") {
                return missing_field_error("targetId");
            }
            if let Some(some) = self.value.r#target_id.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("targetId", &some)?;
            }
            if self.value.r#target_id.id.is_some() || !self.value.r#target_id.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#target_id.id.as_ref(),
                    extension: &self.value.r#target_id.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_targetId", ctx)
                })?;
            }
        } else {
            if self.value.r#target_id.id.as_deref() == Some("$invalid") {
                return missing_field_error("targetId");
            }
            self.with_context(&self.value.r#target_id, |ctx| {
                state.serialize_entry("targetId", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#relationship.id.as_deref() == Some("$invalid") {
                return missing_field_error("relationship");
            }
            if let Some(some) = self.value.r#relationship.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("relationship", &some)?;
            }
            if self.value.r#relationship.id.is_some()
                || !self.value.r#relationship.extension.is_empty()
            {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#relationship.id.as_ref(),
                    extension: &self.value.r#relationship.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_relationship", ctx)
                })?;
            }
        } else {
            if self.value.r#relationship.id.as_deref() == Some("$invalid") {
                return missing_field_error("relationship");
            }
            self.with_context(&self.value.r#relationship, |ctx| {
                state.serialize_entry("relationship", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#end_relationship.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("endRelationship", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_endRelationship", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#end_relationship.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("endRelationship", ctx))?;
            }
        }
        if let Some(some) = self.value.r#offset.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedActionOffset :: Duration (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("offsetDuration" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedActionOffset :: Range (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("offsetRange" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedActionOffset :: Invalid => { return Err (serde :: ser :: Error :: custom ("offset is invalid")) } }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionRelatedAction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction,
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
                    #[serde(rename = "targetId")]
                    TargetId,
                    #[serde(rename = "_targetId")]
                    TargetIdPrimitiveElement,
                    #[serde(rename = "relationship")]
                    Relationship,
                    #[serde(rename = "_relationship")]
                    RelationshipPrimitiveElement,
                    #[serde(rename = "endRelationship")]
                    EndRelationship,
                    #[serde(rename = "_endRelationship")]
                    EndRelationshipPrimitiveElement,
                    #[serde(rename = "offsetDuration")]
                    OffsetDuration,
                    #[serde(rename = "offsetRange")]
                    OffsetRange,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "targetId",
                            "relationship",
                            "endRelationship",
                            "offsetDuration",
                            "offsetRange",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#target_id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#relationship: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#end_relationship: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#offset: Option<
                    fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedActionOffset,
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::TargetId => {
                            if self.0.from_json {
                                let some = r#target_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#target_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetId"));
                                }
                                r#target_id = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Id>(),
                                )?);
                            }
                        }
                        Field::TargetIdPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#target_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_targetId"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("targetId");
                            }
                        }
                        Field::Relationship => {
                            if self.0.from_json {
                                let some = r#relationship.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::RelationshipPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#relationship.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_relationship"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("relationship");
                            }
                        }
                        Field::EndRelationship => {
                            if self.0.from_json {
                                let some = r#end_relationship.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "endRelationship",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#end_relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "endRelationship",
                                    ));
                                }
                                r#end_relationship = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::EndRelationshipPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#end_relationship.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_endRelationship",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("endRelationship");
                            }
                        }
                        Field::OffsetDuration => {
                            if r#offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offsetDuration"));
                            }
                            r#offset = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedActionOffset :: Duration (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Duration > > ()) ?)) ;
                        }
                        Field::OffsetRange => {
                            if r#offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offsetRange"));
                            }
                            r#offset = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedActionOffset :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Range > > ()) ?)) ;
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
                    fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#target_id: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#target_id.unwrap_or(Default::default())
                        } else {
                            r#target_id.ok_or(serde::de::Error::missing_field("targetId"))?
                        },
                        r#relationship: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#relationship.unwrap_or(Default::default())
                        } else {
                            r#relationship.ok_or(serde::de::Error::missing_field("relationship"))?
                        },
                        r#end_relationship,
                        r#offset,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedAction > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant,
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
                "RequestOrchestration.action.participant", field
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
            if let Some(some) = self.value.r#type_canonical.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("typeCanonical", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_typeCanonical", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#type_canonical.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("typeCanonical", ctx))?;
            }
        }
        if let Some(some) = self.value.r#type_reference.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("typeReference", ctx))?;
        }
        if let Some(some) = self.value.r#role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
        }
        if let Some(some) = self.value.r#function.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("function", ctx))?;
        }
        if let Some(some) = self.value.r#actor.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("actorCanonical" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_actorCanonical" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("actorCanonical" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Reference (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("actorReference" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Invalid => { return Err (serde :: ser :: Error :: custom ("actor is invalid")) } }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionParticipant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant,
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "typeCanonical")]
                    TypeCanonical,
                    #[serde(rename = "_typeCanonical")]
                    TypeCanonicalPrimitiveElement,
                    #[serde(rename = "typeReference")]
                    TypeReference,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "function")]
                    Function,
                    #[serde(rename = "actorCanonical")]
                    ActorCanonical,
                    #[serde(rename = "_actorCanonical")]
                    ActorCanonicalPrimitiveElement,
                    #[serde(rename = "actorReference")]
                    ActorReference,
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
                            "typeCanonical",
                            "typeReference",
                            "role",
                            "function",
                            "actorCanonical",
                            "actorReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#type_canonical: Option<fhirbolt_model::r5::types::Canonical> = None;
                let mut r#type_reference: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#function: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#actor: Option<
                    fhirbolt_model::r5::resources::RequestOrchestrationActionParticipantActor,
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
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
                        Field::TypeCanonical => {
                            if self.0.from_json {
                                let some = r#type_canonical.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeCanonical"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#type_canonical.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeCanonical"));
                                }
                                r#type_canonical = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::TypeCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type_canonical.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_typeCanonical",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("typeCanonical");
                            }
                        }
                        Field::TypeReference => {
                            if r#type_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeReference"));
                            }
                            r#type_reference = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ActorCanonical => {
                            if self.0.from_json {
                                let r#enum = r#actor . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("actorCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("actor[x]")) ; }
                            } else {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "actorCanonical",
                                    ));
                                }
                                r#actor = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::ActorCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#actor . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_actorCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_actor[x]")) ; }
                            } else {
                                return unknown_field_error("actorCanonical");
                            }
                        }
                        Field::ActorReference => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actorReference"));
                            }
                            r#actor = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipantActor :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Reference > > ()) ?)) ;
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
                    fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#type_canonical,
                        r#type_reference,
                        r#role,
                        r#function,
                        r#actor,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipant > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue,
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
                "RequestOrchestration.action.dynamicValue", field
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
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("path", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_path", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#path.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("path", ctx))?;
            }
        }
        if let Some(some) = self.value.r#expression.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationActionDynamicValue")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue,
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
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "path", "expression"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#expression: Option<Box<fhirbolt_model::r5::types::Expression>> = None;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                        Field::Expression => {
                            if r#expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            r#expression = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Expression>>(),
                                )?,
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
                Ok(
                    fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#path,
                        r#expression,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDynamicValue > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestrationAction,
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
                "RequestOrchestration.action", field
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
            if let Some(some) = self.value.r#link_id.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("linkId", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_linkId", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#link_id.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("linkId", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#prefix.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("prefix", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_prefix", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#prefix.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("prefix", ctx))?;
            }
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
            if let Some(some) = self.value.r#text_equivalent.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("textEquivalent", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_textEquivalent", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text_equivalent.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("textEquivalent", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#priority.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("priority", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_priority", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#priority.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
            }
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if !self.value.r#documentation.is_empty() {
            self.with_context(&self.value.r#documentation, |ctx| {
                state.serialize_entry("documentation", ctx)
            })?;
        }
        if !self.value.r#goal.is_empty() {
            self.with_context(&self.value.r#goal, |ctx| state.serialize_entry("goal", ctx))?;
        }
        if !self.value.r#condition.is_empty() {
            self.with_context(&self.value.r#condition, |ctx| {
                state.serialize_entry("condition", ctx)
            })?;
        }
        if !self.value.r#input.is_empty() {
            self.with_context(&self.value.r#input, |ctx| {
                state.serialize_entry("input", ctx)
            })?;
        }
        if !self.value.r#output.is_empty() {
            self.with_context(&self.value.r#output, |ctx| {
                state.serialize_entry("output", ctx)
            })?;
        }
        if !self.value.r#related_action.is_empty() {
            self.with_context(&self.value.r#related_action, |ctx| {
                state.serialize_entry("relatedAction", ctx)
            })?;
        }
        if let Some(some) = self.value.r#timing.as_ref() {
            match some {
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::DateTime(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("timingDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_timingDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("timingDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("timingAge", ctx))?;
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Period(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("timingPeriod", ctx))?;
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Duration(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("timingDuration", ctx))?;
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Range(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("timingRange", ctx))?;
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Timing(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("timingTiming", ctx))?;
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionTiming::Invalid => {
                    return Err(serde::ser::Error::custom("timing is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#location.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("location", ctx))?;
        }
        if !self.value.r#participant.is_empty() {
            self.with_context(&self.value.r#participant, |ctx| {
                state.serialize_entry("participant", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#grouping_behavior.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("groupingBehavior", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_groupingBehavior", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#grouping_behavior.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("groupingBehavior", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#selection_behavior.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("selectionBehavior", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_selectionBehavior", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#selection_behavior.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("selectionBehavior", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#required_behavior.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("requiredBehavior", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requiredBehavior", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#required_behavior.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("requiredBehavior", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#precheck_behavior.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("precheckBehavior", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_precheckBehavior", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#precheck_behavior.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("precheckBehavior", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#cardinality_behavior.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("cardinalityBehavior", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_cardinalityBehavior", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#cardinality_behavior.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("cardinalityBehavior", ctx)
                })?;
            }
        }
        if let Some(some) = self.value.r#resource.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("resource", ctx))?;
        }
        if let Some(some) = self.value.r#definition.as_ref() {
            match some {
                fhirbolt_model::r5::resources::RequestOrchestrationActionDefinition::Canonical(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("definitionCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_definitionCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("definitionCanonical", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionDefinition::Uri(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("definitionUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_definitionUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("definitionUri", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r5::resources::RequestOrchestrationActionDefinition::Invalid => {
                    return Err(serde::ser::Error::custom("definition is invalid"))
                }
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
        if !self.value.r#dynamic_value.is_empty() {
            self.with_context(&self.value.r#dynamic_value, |ctx| {
                state.serialize_entry("dynamicValue", ctx)
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
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
        fhirbolt_model::r5::resources::RequestOrchestrationAction,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestrationAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestrationAction,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestrationAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestrationAction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::RequestOrchestrationAction, V::Error>
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
                    #[serde(rename = "linkId")]
                    LinkId,
                    #[serde(rename = "_linkId")]
                    LinkIdPrimitiveElement,
                    #[serde(rename = "prefix")]
                    Prefix,
                    #[serde(rename = "_prefix")]
                    PrefixPrimitiveElement,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "textEquivalent")]
                    TextEquivalent,
                    #[serde(rename = "_textEquivalent")]
                    TextEquivalentPrimitiveElement,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "_priority")]
                    PriorityPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "documentation")]
                    Documentation,
                    #[serde(rename = "goal")]
                    Goal,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "input")]
                    Input,
                    #[serde(rename = "output")]
                    Output,
                    #[serde(rename = "relatedAction")]
                    RelatedAction,
                    #[serde(rename = "timingDateTime")]
                    TimingDateTime,
                    #[serde(rename = "_timingDateTime")]
                    TimingDateTimePrimitiveElement,
                    #[serde(rename = "timingAge")]
                    TimingAge,
                    #[serde(rename = "timingPeriod")]
                    TimingPeriod,
                    #[serde(rename = "timingDuration")]
                    TimingDuration,
                    #[serde(rename = "timingRange")]
                    TimingRange,
                    #[serde(rename = "timingTiming")]
                    TimingTiming,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "participant")]
                    Participant,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "groupingBehavior")]
                    GroupingBehavior,
                    #[serde(rename = "_groupingBehavior")]
                    GroupingBehaviorPrimitiveElement,
                    #[serde(rename = "selectionBehavior")]
                    SelectionBehavior,
                    #[serde(rename = "_selectionBehavior")]
                    SelectionBehaviorPrimitiveElement,
                    #[serde(rename = "requiredBehavior")]
                    RequiredBehavior,
                    #[serde(rename = "_requiredBehavior")]
                    RequiredBehaviorPrimitiveElement,
                    #[serde(rename = "precheckBehavior")]
                    PrecheckBehavior,
                    #[serde(rename = "_precheckBehavior")]
                    PrecheckBehaviorPrimitiveElement,
                    #[serde(rename = "cardinalityBehavior")]
                    CardinalityBehavior,
                    #[serde(rename = "_cardinalityBehavior")]
                    CardinalityBehaviorPrimitiveElement,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "definitionCanonical")]
                    DefinitionCanonical,
                    #[serde(rename = "_definitionCanonical")]
                    DefinitionCanonicalPrimitiveElement,
                    #[serde(rename = "definitionUri")]
                    DefinitionUri,
                    #[serde(rename = "_definitionUri")]
                    DefinitionUriPrimitiveElement,
                    #[serde(rename = "transform")]
                    Transform,
                    #[serde(rename = "_transform")]
                    TransformPrimitiveElement,
                    #[serde(rename = "dynamicValue")]
                    DynamicValue,
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
                            "linkId",
                            "prefix",
                            "title",
                            "description",
                            "textEquivalent",
                            "priority",
                            "code",
                            "documentation",
                            "goal",
                            "condition",
                            "input",
                            "output",
                            "relatedAction",
                            "timingDateTime",
                            "timingAge",
                            "timingPeriod",
                            "timingDuration",
                            "timingRange",
                            "timingTiming",
                            "location",
                            "participant",
                            "type",
                            "groupingBehavior",
                            "selectionBehavior",
                            "requiredBehavior",
                            "precheckBehavior",
                            "cardinalityBehavior",
                            "resource",
                            "definitionCanonical",
                            "definitionUri",
                            "transform",
                            "dynamicValue",
                            "action",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#link_id: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#prefix: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#text_equivalent: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#priority: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#code: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#documentation: Option<Vec<fhirbolt_model::r5::types::RelatedArtifact>> =
                    None;
                let mut r#goal: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#condition: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionCondition>,
                > = None;
                let mut r#input: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionInput>,
                > = None;
                let mut r#output: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionOutput>,
                > = None;
                let mut r#related_action: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionRelatedAction>,
                > = None;
                let mut r#timing: Option<
                    fhirbolt_model::r5::resources::RequestOrchestrationActionTiming,
                > = None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#participant: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionParticipant>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#grouping_behavior: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#selection_behavior: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#required_behavior: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#precheck_behavior: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#cardinality_behavior: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#resource: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#definition: Option<
                    fhirbolt_model::r5::resources::RequestOrchestrationActionDefinition,
                > = None;
                let mut r#transform: Option<fhirbolt_model::r5::types::Canonical> = None;
                let mut r#dynamic_value: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationActionDynamicValue>,
                > = None;
                let mut r#action: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::LinkId => {
                            if self.0.from_json {
                                let some = r#link_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#link_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                r#link_id = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::LinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#link_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("linkId");
                            }
                        }
                        Field::Prefix => {
                            if self.0.from_json {
                                let some = r#prefix.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prefix"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#prefix.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prefix"));
                                }
                                r#prefix = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::PrefixPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#prefix.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_prefix"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("prefix");
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
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
                        Field::TextEquivalent => {
                            if self.0.from_json {
                                let some = r#text_equivalent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textEquivalent",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text_equivalent.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textEquivalent",
                                    ));
                                }
                                r#text_equivalent = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::TextEquivalentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text_equivalent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_textEquivalent",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("textEquivalent");
                            }
                        }
                        Field::Priority => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::PriorityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_priority"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("priority");
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Documentation => {
                            if self.0.from_json {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: RelatedArtifact >> ()) ?) ;
                            } else {
                                let vec = r#documentation.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: RelatedArtifact > ()) ?) ;
                            }
                        }
                        Field::Goal => {
                            if self.0.from_json {
                                if r#goal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("goal"));
                                }
                                r#goal = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#goal.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionCondition >> ()) ?) ;
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionCondition > ()) ?) ;
                            }
                        }
                        Field::Input => {
                            if self.0.from_json {
                                if r#input.is_some() {
                                    return Err(serde::de::Error::duplicate_field("input"));
                                }
                                r#input = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionInput >> ()) ?) ;
                            } else {
                                let vec = r#input.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionInput > ()) ?) ;
                            }
                        }
                        Field::Output => {
                            if self.0.from_json {
                                if r#output.is_some() {
                                    return Err(serde::de::Error::duplicate_field("output"));
                                }
                                r#output = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionOutput >> ()) ?) ;
                            } else {
                                let vec = r#output.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionOutput > ()) ?) ;
                            }
                        }
                        Field::RelatedAction => {
                            if self.0.from_json {
                                if r#related_action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedAction"));
                                }
                                r#related_action = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedAction >> ()) ?) ;
                            } else {
                                let vec = r#related_action.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionRelatedAction > ()) ?) ;
                            }
                        }
                        Field::TimingDateTime => {
                            if self.0.from_json {
                                let r#enum = r#timing . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("timingDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("timing[x]")) ; }
                            } else {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDateTime",
                                    ));
                                }
                                r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::TimingDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#timing . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_timingDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_timing[x]")) ; }
                            } else {
                                return unknown_field_error("timingDateTime");
                            }
                        }
                        Field::TimingAge => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingAge"));
                            }
                            r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: Age (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Age > > ()) ?)) ;
                        }
                        Field::TimingPeriod => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingPeriod"));
                            }
                            r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: Period (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Period > > ()) ?)) ;
                        }
                        Field::TimingDuration => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingDuration"));
                            }
                            r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: Duration (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Duration > > ()) ?)) ;
                        }
                        Field::TimingRange => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingRange"));
                            }
                            r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Range > > ()) ?)) ;
                        }
                        Field::TimingTiming => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingTiming"));
                            }
                            r#timing = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionTiming :: Timing (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Timing > > ()) ?)) ;
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::Participant => {
                            if self.0.from_json {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                r#participant = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipant >> ()) ?) ;
                            } else {
                                let vec = r#participant.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionParticipant > ()) ?) ;
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::GroupingBehavior => {
                            if self.0.from_json {
                                let some = r#grouping_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupingBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#grouping_behavior.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupingBehavior",
                                    ));
                                }
                                r#grouping_behavior = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::GroupingBehaviorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#grouping_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_groupingBehavior",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("groupingBehavior");
                            }
                        }
                        Field::SelectionBehavior => {
                            if self.0.from_json {
                                let some = r#selection_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "selectionBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#selection_behavior.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "selectionBehavior",
                                    ));
                                }
                                r#selection_behavior = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::SelectionBehaviorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#selection_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_selectionBehavior",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("selectionBehavior");
                            }
                        }
                        Field::RequiredBehavior => {
                            if self.0.from_json {
                                let some = r#required_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "requiredBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#required_behavior.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "requiredBehavior",
                                    ));
                                }
                                r#required_behavior = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::RequiredBehaviorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#required_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_requiredBehavior",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requiredBehavior");
                            }
                        }
                        Field::PrecheckBehavior => {
                            if self.0.from_json {
                                let some = r#precheck_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "precheckBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#precheck_behavior.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "precheckBehavior",
                                    ));
                                }
                                r#precheck_behavior = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::PrecheckBehaviorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#precheck_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_precheckBehavior",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("precheckBehavior");
                            }
                        }
                        Field::CardinalityBehavior => {
                            if self.0.from_json {
                                let some = r#cardinality_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cardinalityBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#cardinality_behavior.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cardinalityBehavior",
                                    ));
                                }
                                r#cardinality_behavior = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::CardinalityBehaviorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#cardinality_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_cardinalityBehavior",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("cardinalityBehavior");
                            }
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::DefinitionCanonical => {
                            if self.0.from_json {
                                let r#enum = r#definition . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Canonical (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionCanonical")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("definition[x]")) ; }
                            } else {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "definitionCanonical",
                                    ));
                                }
                                r#definition = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::DefinitionCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#definition . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Canonical (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Canonical (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_definitionCanonical")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_definition[x]")) ; }
                            } else {
                                return unknown_field_error("definitionCanonical");
                            }
                        }
                        Field::DefinitionUri => {
                            if self.0.from_json {
                                let r#enum = r#definition . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("definitionUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("definition[x]")) ; }
                            } else {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definitionUri"));
                                }
                                r#definition = Some (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::DefinitionUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#definition . get_or_insert (fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDefinition :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_definitionUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_definition[x]")) ; }
                            } else {
                                return unknown_field_error("definitionUri");
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Canonical>(),
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
                        Field::DynamicValue => {
                            if self.0.from_json {
                                if r#dynamic_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dynamicValue"));
                                }
                                r#dynamic_value = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDynamicValue >> ()) ?) ;
                            } else {
                                let vec = r#dynamic_value.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationActionDynamicValue > ()) ?) ;
                            }
                        }
                        Field::Action => {
                            if self.0.from_json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r5::resources::RequestOrchestrationAction,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationAction > ()) ?) ;
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
                Ok(fhirbolt_model::r5::resources::RequestOrchestrationAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#link_id,
                    r#prefix,
                    r#title,
                    r#description,
                    r#text_equivalent,
                    r#priority,
                    r#code: r#code.unwrap_or(vec![]),
                    r#documentation: r#documentation.unwrap_or(vec![]),
                    r#goal: r#goal.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#input: r#input.unwrap_or(vec![]),
                    r#output: r#output.unwrap_or(vec![]),
                    r#related_action: r#related_action.unwrap_or(vec![]),
                    r#timing,
                    r#location,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#type,
                    r#grouping_behavior,
                    r#selection_behavior,
                    r#required_behavior,
                    r#precheck_behavior,
                    r#cardinality_behavior,
                    r#resource,
                    r#definition,
                    r#transform,
                    r#dynamic_value: r#dynamic_value.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestrationAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestrationAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>;
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
                        .transmute::<fhirbolt_model::r5::resources::RequestOrchestrationAction>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::RequestOrchestration {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::RequestOrchestration,
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
                "RequestOrchestration", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "RequestOrchestration")?;
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
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#instantiates_canonical.is_empty() {
                let values = self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("instantiatesCanonical", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_canonical
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
                        state.serialize_entry("_instantiatesCanonical", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#instantiates_canonical.is_empty() {
                self.with_context(&self.value.r#instantiates_canonical, |ctx| {
                    state.serialize_entry("instantiatesCanonical", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#instantiates_uri.is_empty() {
                let values = self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("instantiatesUri", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_uri
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
                        state.serialize_entry("_instantiatesUri", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#instantiates_uri.is_empty() {
                self.with_context(&self.value.r#instantiates_uri, |ctx| {
                    state.serialize_entry("instantiatesUri", ctx)
                })?;
            }
        }
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if !self.value.r#replaces.is_empty() {
            self.with_context(&self.value.r#replaces, |ctx| {
                state.serialize_entry("replaces", ctx)
            })?;
        }
        if let Some(some) = self.value.r#group_identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("groupIdentifier", ctx))?;
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
            if self.value.r#intent.id.as_deref() == Some("$invalid") {
                return missing_field_error("intent");
            }
            if let Some(some) = self.value.r#intent.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("intent", &some)?;
            }
            if self.value.r#intent.id.is_some() || !self.value.r#intent.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#intent.id.as_ref(),
                    extension: &self.value.r#intent.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_intent", ctx)
                })?;
            }
        } else {
            if self.value.r#intent.id.as_deref() == Some("$invalid") {
                return missing_field_error("intent");
            }
            self.with_context(&self.value.r#intent, |ctx| {
                state.serialize_entry("intent", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#priority.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("priority", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_priority", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#priority.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
            }
        }
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        if let Some(some) = self.value.r#subject.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subject", ctx))?;
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#authored_on.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("authoredOn", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_authoredOn", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#authored_on.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("authoredOn", ctx))?;
            }
        }
        if let Some(some) = self.value.r#author.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("author", ctx))?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if !self.value.r#goal.is_empty() {
            self.with_context(&self.value.r#goal, |ctx| state.serialize_entry("goal", ctx))?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#action.is_empty() {
            self.with_context(&self.value.r#action, |ctx| {
                state.serialize_entry("action", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::RequestOrchestration>,
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
        &Vec<fhirbolt_model::r5::resources::RequestOrchestration>,
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
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::RequestOrchestration,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestration;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::RequestOrchestration,
    >
{
    type Value = fhirbolt_model::r5::resources::RequestOrchestration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::RequestOrchestration,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::RequestOrchestration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RequestOrchestration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::RequestOrchestration, V::Error>
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "instantiatesCanonical")]
                    InstantiatesCanonical,
                    #[serde(rename = "_instantiatesCanonical")]
                    InstantiatesCanonicalPrimitiveElement,
                    #[serde(rename = "instantiatesUri")]
                    InstantiatesUri,
                    #[serde(rename = "_instantiatesUri")]
                    InstantiatesUriPrimitiveElement,
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "replaces")]
                    Replaces,
                    #[serde(rename = "groupIdentifier")]
                    GroupIdentifier,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "intent")]
                    Intent,
                    #[serde(rename = "_intent")]
                    IntentPrimitiveElement,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "_priority")]
                    PriorityPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "authoredOn")]
                    AuthoredOn,
                    #[serde(rename = "_authoredOn")]
                    AuthoredOnPrimitiveElement,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "goal")]
                    Goal,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "action")]
                    Action,
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
                            "identifier",
                            "instantiatesCanonical",
                            "instantiatesUri",
                            "basedOn",
                            "replaces",
                            "groupIdentifier",
                            "status",
                            "intent",
                            "priority",
                            "code",
                            "subject",
                            "encounter",
                            "authoredOn",
                            "author",
                            "reason",
                            "goal",
                            "note",
                            "action",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#instantiates_canonical: Option<
                    Vec<fhirbolt_model::r5::types::Canonical>,
                > = None;
                let mut r#instantiates_uri: Option<Vec<fhirbolt_model::r5::types::Uri>> = None;
                let mut r#based_on: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#replaces: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#group_identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> =
                    None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#intent: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#priority: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#authored_on: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#author: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#goal: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#action: Option<
                    Vec<fhirbolt_model::r5::resources::RequestOrchestrationAction>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "RequestOrchestration" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"RequestOrchestration",
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
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r5::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r5::Resource>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::Resource>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Identifier >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Identifier>(),
                                )?);
                            }
                        }
                        Field::InstantiatesCanonical => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "instantiatesCanonical",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "_instantiatesCanonical",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("instantiatesCanonical");
                            }
                        }
                        Field::InstantiatesUri => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#instantiates_uri.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("instantiatesUri");
                            }
                        }
                        Field::BasedOn => {
                            if self.0.from_json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Replaces => {
                            if self.0.from_json {
                                if r#replaces.is_some() {
                                    return Err(serde::de::Error::duplicate_field("replaces"));
                                }
                                r#replaces = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#replaces.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::GroupIdentifier => {
                            if r#group_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupIdentifier"));
                            }
                            r#group_identifier = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Identifier>>(),
                                )?,
                            );
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
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
                        Field::Intent => {
                            if self.0.from_json {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#intent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                r#intent = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::IntentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_intent"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("intent");
                            }
                        }
                        Field::Priority => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::PriorityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_priority"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("priority");
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::AuthoredOn => {
                            if self.0.from_json {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authoredOn"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#authored_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authoredOn"));
                                }
                                r#authored_on = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::AuthoredOnPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_authoredOn"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("authoredOn");
                            }
                        }
                        Field::Author => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Reason => {
                            if self.0.from_json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableReference >> ()) ?) ;
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableReference > ()) ?) ;
                            }
                        }
                        Field::Goal => {
                            if self.0.from_json {
                                if r#goal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("goal"));
                                }
                                r#goal = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#goal.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Annotation >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Annotation>(),
                                )?);
                            }
                        }
                        Field::Action => {
                            if self.0.from_json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r5::resources::RequestOrchestrationAction,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: RequestOrchestrationAction > ()) ?) ;
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
                Ok(fhirbolt_model::r5::resources::RequestOrchestration {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#replaces: r#replaces.unwrap_or(vec![]),
                    r#group_identifier,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#intent: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#intent.unwrap_or(Default::default())
                    } else {
                        r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                    },
                    r#priority,
                    r#code,
                    r#subject,
                    r#encounter,
                    r#authored_on,
                    r#author,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#goal: r#goal.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::RequestOrchestration>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::RequestOrchestration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::RequestOrchestration>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::RequestOrchestration>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::RequestOrchestration>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::RequestOrchestration>;
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
                        .transmute::<fhirbolt_model::r5::resources::RequestOrchestration>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
