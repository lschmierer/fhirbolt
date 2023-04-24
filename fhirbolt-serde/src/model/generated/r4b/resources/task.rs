// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::resources::TaskRestriction>
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
                "Task.restriction", field
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
            if let Some(some) = self.value.r#repetitions.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("repetitions", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_repetitions", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#repetitions.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("repetitions", ctx))?;
            }
        }
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        if !self.value.r#recipient.is_empty() {
            self.with_context(&self.value.r#recipient, |ctx| {
                state.serialize_entry("recipient", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::TaskRestriction>,
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
        &Vec<fhirbolt_model::r4b::resources::TaskRestriction>,
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
        fhirbolt_model::r4b::resources::TaskRestriction,
    >
{
    type Value = fhirbolt_model::r4b::resources::TaskRestriction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::TaskRestriction,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::TaskRestriction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskRestriction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::TaskRestriction, V::Error>
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
                    #[serde(rename = "repetitions")]
                    Repetitions,
                    #[serde(rename = "_repetitions")]
                    RepetitionsPrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "recipient")]
                    Recipient,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "repetitions",
                            "period",
                            "recipient",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#repetitions: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#recipient: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Repetitions => {
                            if self.0.from_json {
                                let some = r#repetitions.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repetitions"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#repetitions.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repetitions"));
                                }
                                r#repetitions = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::RepetitionsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#repetitions.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_repetitions"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("repetitions");
                            }
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                )?,
                            );
                        }
                        Field::Recipient => {
                            if self.0.from_json {
                                if r#recipient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recipient"));
                                }
                                r#recipient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Reference >> ()) ?) ;
                            } else {
                                let vec = r#recipient.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Reference>(),
                                )?);
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
                Ok(fhirbolt_model::r4b::resources::TaskRestriction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#repetitions,
                    r#period,
                    r#recipient: r#recipient.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::TaskRestriction>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::TaskRestriction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::TaskRestriction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::TaskRestriction>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::TaskRestriction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::TaskRestriction>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::TaskRestriction>;
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
                        .transmute::<fhirbolt_model::r4b::resources::TaskRestriction>(),
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
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::resources::TaskInput>
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
                "Task.input", field
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        match self.value.r#value {
            fhirbolt_model::r4b::resources::TaskInputValue::Base64Binary(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Boolean(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Canonical(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Code(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Date(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::DateTime(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Decimal(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Id(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Instant(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Integer(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Markdown(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Oid(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::PositiveInt(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::String(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Time(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::UnsignedInt(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Uri(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Url(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Uuid(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskInputValue::Address(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Age(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Annotation(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAnnotation", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::CodeableConcept(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Coding(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::ContactPoint(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContactPoint", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Count(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Distance(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDistance", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Duration(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDuration", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::HumanName(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueHumanName", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Identifier(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueIdentifier", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Money(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Period(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Quantity(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Range(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Ratio(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::SampledData(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSampledData", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Signature(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSignature", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Timing(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::ContactDetail(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueContactDetail", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Contributor(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContributor", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::DataRequirement(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueDataRequirement", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Expression(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueExpression", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::ParameterDefinition(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueParameterDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::RelatedArtifact(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueRelatedArtifact", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::TriggerDefinition(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueTriggerDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::UsageContext(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueUsageContext", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Dosage(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Meta(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueMeta", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskInputValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4b::resources::TaskInput>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4b::resources::TaskInput>>
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::TaskInput>
{
    type Value = fhirbolt_model::r4b::resources::TaskInput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::TaskInput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::TaskInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskInput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::TaskInput, V::Error>
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
                    #[serde(rename = "valueMeta")]
                    ValueMeta,
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
                            "valueMeta",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<fhirbolt_model::r4b::resources::TaskInputValue> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskInputValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskInputValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCanonical",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCanonical");
                            }
                        }
                        Field::ValueCode => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCode");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Id(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInstant");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueMarkdown",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueOid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueOid"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueOid");
                            }
                        }
                        Field::ValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valuePositiveInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valuePositiveInt",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valuePositiveInt");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueUnsignedInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUnsignedInt",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUnsignedInt");
                            }
                        }
                        Field::ValueUri => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUri"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUrl");
                            }
                        }
                        Field::ValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskInputValue::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskInputValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskInputValue::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUuid");
                            }
                        }
                        Field::ValueAddress => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskInputValue::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueAge => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Age(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                )?,
                            ));
                        }
                        Field::ValueAnnotation => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Coding(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                )?,
                            ));
                        }
                        Field::ValueContactPoint => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::ValueCount => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Count(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                )?,
                            ));
                        }
                        Field::ValueDistance => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskInputValue::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueDuration => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskInputValue::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueHumanName => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::ValueIdentifier => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::ValueMoney => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Money(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                )?,
                            ));
                        }
                        Field::ValuePeriod => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Period(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                )?,
                            ));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskInputValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Range(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                )?,
                            ));
                        }
                        Field::ValueRatio => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Ratio(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                )?,
                            ));
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::ValueSampledData => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::ValueSignature => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::ValueTiming => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Timing(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                )?,
                            ));
                        }
                        Field::ValueContactDetail => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::ValueContributor => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::ValueDataRequirement => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::ValueExpression => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::ValueParameterDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::ValueRelatedArtifact => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::ValueTriggerDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::ValueUsageContext => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskInputValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::ValueDosage => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Dosage(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                )?,
                            ));
                        }
                        Field::ValueMeta => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskInputValue::Meta(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Meta>>(),
                                )?,
                            ));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::TaskInput {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
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
        Box<fhirbolt_model::r4b::resources::TaskInput>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::TaskInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::TaskInput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::TaskInput>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::TaskInput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::TaskInput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::TaskInput>;
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
                        .transmute::<fhirbolt_model::r4b::resources::TaskInput>(),
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
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::resources::TaskOutput>
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
                "Task.output", field
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        match self.value.r#value {
            fhirbolt_model::r4b::resources::TaskOutputValue::Base64Binary(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Boolean(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Canonical(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Code(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Date(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::DateTime(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Decimal(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Id(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Instant(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Integer(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Markdown(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Oid(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::PositiveInt(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::String(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Time(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::UnsignedInt(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Uri(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Url(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(ref value) => {
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
            fhirbolt_model::r4b::resources::TaskOutputValue::Address(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Age(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Annotation(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAnnotation", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::CodeableConcept(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Coding(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::ContactPoint(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContactPoint", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Count(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Distance(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDistance", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Duration(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDuration", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::HumanName(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueHumanName", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Identifier(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueIdentifier", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Money(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Period(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Quantity(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Range(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Ratio(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::SampledData(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSampledData", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Signature(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueSignature", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Timing(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::ContactDetail(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueContactDetail", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Contributor(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueContributor", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::DataRequirement(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueDataRequirement", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Expression(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueExpression", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::ParameterDefinition(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueParameterDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::RelatedArtifact(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueRelatedArtifact", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::TriggerDefinition(ref value) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("valueTriggerDefinition", ctx)
                })?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::UsageContext(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueUsageContext", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Dosage(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Meta(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueMeta", ctx))?;
            }
            fhirbolt_model::r4b::resources::TaskOutputValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4b::resources::TaskOutput>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4b::resources::TaskOutput>>
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::TaskOutput>
{
    type Value = fhirbolt_model::r4b::resources::TaskOutput;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::TaskOutput,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::TaskOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskOutput")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::TaskOutput, V::Error>
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
                    #[serde(rename = "valueMeta")]
                    ValueMeta,
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
                            "valueMeta",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<fhirbolt_model::r4b::resources::TaskOutputValue> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCanonical",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCanonical");
                            }
                        }
                        Field::ValueCode => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Code(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Code>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Code(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCode");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Date>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Id(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Id(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInstant");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueMarkdown",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueOid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Oid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Oid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Oid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueOid"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueOid");
                            }
                        }
                        Field::ValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: PositiveInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valuePositiveInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: PositiveInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valuePositiveInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valuePositiveInt");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Time(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Time>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Time(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: TaskOutputValue :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUnsignedInt");
                            }
                        }
                        Field::ValueUri => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Uri(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUri"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Url(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Url>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Url(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUrl");
                            }
                        }
                        Field::ValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                r#value =
                                    Some(fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::resources::TaskOutputValue::Uuid(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUuid");
                            }
                        }
                        Field::ValueAddress => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Address(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueAge => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Age(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                )?,
                            ));
                        }
                        Field::ValueAnnotation => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Annotation (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?)) ;
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                    )?,
                                ));
                        }
                        Field::ValueContactPoint => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::ValueCount => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Count(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                )?,
                            ));
                        }
                        Field::ValueDistance => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Distance(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueDuration => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Duration(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueHumanName => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: HumanName (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: HumanName > > ()) ?)) ;
                        }
                        Field::ValueIdentifier => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::ValueMoney => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Money(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                )?,
                            ));
                        }
                        Field::ValuePeriod => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Range(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                )?,
                            ));
                        }
                        Field::ValueRatio => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Ratio(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                )?,
                            ));
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::ValueSampledData => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::ValueSignature => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Signature (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Signature > > ()) ?)) ;
                        }
                        Field::ValueTiming => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                    )?,
                                ));
                        }
                        Field::ValueContactDetail => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::ValueContributor => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::ValueDataRequirement => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::ValueExpression => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: Expression (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Expression > > ()) ?)) ;
                        }
                        Field::ValueParameterDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: ParameterDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ParameterDefinition > > ()) ?)) ;
                        }
                        Field::ValueRelatedArtifact => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::ValueTriggerDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::ValueUsageContext => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: TaskOutputValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::ValueDosage => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value =
                                Some(fhirbolt_model::r4b::resources::TaskOutputValue::Dosage(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                ));
                        }
                        Field::ValueMeta => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            r#value = Some(fhirbolt_model::r4b::resources::TaskOutputValue::Meta(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Meta>>(),
                                )?,
                            ));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::TaskOutput {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
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
        Box<fhirbolt_model::r4b::resources::TaskOutput>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::TaskOutput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::TaskOutput>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::TaskOutput>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::TaskOutput>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::TaskOutput>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::TaskOutput>;
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
                        .transmute::<fhirbolt_model::r4b::resources::TaskOutput>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::Task {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::resources::Task>
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
                "Task", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Task")?;
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
            if let Some(some) = self.value.r#instantiates_canonical.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("instantiatesCanonical", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_instantiatesCanonical", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#instantiates_canonical.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("instantiatesCanonical", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#instantiates_uri.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("instantiatesUri", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_instantiatesUri", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#instantiates_uri.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("instantiatesUri", ctx))?;
            }
        }
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if let Some(some) = self.value.r#group_identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("groupIdentifier", ctx))?;
        }
        if !self.value.r#part_of.is_empty() {
            self.with_context(&self.value.r#part_of, |ctx| {
                state.serialize_entry("partOf", ctx)
            })?;
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
        if let Some(some) = self.value.r#status_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("statusReason", ctx))?;
        }
        if let Some(some) = self.value.r#business_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("businessStatus", ctx))?;
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
        if let Some(some) = self.value.r#focus.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("focus", ctx))?;
        }
        if let Some(some) = self.value.r#for.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("for", ctx))?;
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if let Some(some) = self.value.r#execution_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("executionPeriod", ctx))?;
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
        if self.output_json {
            if let Some(some) = self.value.r#last_modified.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lastModified", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastModified", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#last_modified.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lastModified", ctx))?;
            }
        }
        if let Some(some) = self.value.r#requester.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requester", ctx))?;
        }
        if !self.value.r#performer_type.is_empty() {
            self.with_context(&self.value.r#performer_type, |ctx| {
                state.serialize_entry("performerType", ctx)
            })?;
        }
        if let Some(some) = self.value.r#owner.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("owner", ctx))?;
        }
        if let Some(some) = self.value.r#location.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("location", ctx))?;
        }
        if let Some(some) = self.value.r#reason_code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reasonCode", ctx))?;
        }
        if let Some(some) = self.value.r#reason_reference.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reasonReference", ctx))?;
        }
        if !self.value.r#insurance.is_empty() {
            self.with_context(&self.value.r#insurance, |ctx| {
                state.serialize_entry("insurance", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#relevant_history.is_empty() {
            self.with_context(&self.value.r#relevant_history, |ctx| {
                state.serialize_entry("relevantHistory", ctx)
            })?;
        }
        if let Some(some) = self.value.r#restriction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("restriction", ctx))?;
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
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4b::resources::Task>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4b::resources::Task>>
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::Task>
{
    type Value = fhirbolt_model::r4b::resources::Task;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::Task>
{
    type Value = fhirbolt_model::r4b::resources::Task;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::Task,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::Task;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Task")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::Task, V::Error>
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
                    #[serde(rename = "groupIdentifier")]
                    GroupIdentifier,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "statusReason")]
                    StatusReason,
                    #[serde(rename = "businessStatus")]
                    BusinessStatus,
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
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "focus")]
                    Focus,
                    #[serde(rename = "for")]
                    For,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "executionPeriod")]
                    ExecutionPeriod,
                    #[serde(rename = "authoredOn")]
                    AuthoredOn,
                    #[serde(rename = "_authoredOn")]
                    AuthoredOnPrimitiveElement,
                    #[serde(rename = "lastModified")]
                    LastModified,
                    #[serde(rename = "_lastModified")]
                    LastModifiedPrimitiveElement,
                    #[serde(rename = "requester")]
                    Requester,
                    #[serde(rename = "performerType")]
                    PerformerType,
                    #[serde(rename = "owner")]
                    Owner,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "reasonCode")]
                    ReasonCode,
                    #[serde(rename = "reasonReference")]
                    ReasonReference,
                    #[serde(rename = "insurance")]
                    Insurance,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "relevantHistory")]
                    RelevantHistory,
                    #[serde(rename = "restriction")]
                    Restriction,
                    #[serde(rename = "input")]
                    Input,
                    #[serde(rename = "output")]
                    Output,
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
                            "groupIdentifier",
                            "partOf",
                            "status",
                            "statusReason",
                            "businessStatus",
                            "intent",
                            "priority",
                            "code",
                            "description",
                            "focus",
                            "for",
                            "encounter",
                            "executionPeriod",
                            "authoredOn",
                            "lastModified",
                            "requester",
                            "performerType",
                            "owner",
                            "location",
                            "reasonCode",
                            "reasonReference",
                            "insurance",
                            "note",
                            "relevantHistory",
                            "restriction",
                            "input",
                            "output",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4b::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4b::types::Identifier>> = None;
                let mut r#instantiates_canonical: Option<fhirbolt_model::r4b::types::Canonical> =
                    None;
                let mut r#instantiates_uri: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#based_on: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#group_identifier: Option<Box<fhirbolt_model::r4b::types::Identifier>> =
                    None;
                let mut r#part_of: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#status_reason: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#business_status: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#intent: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#priority: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#focus: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#for: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#execution_period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#authored_on: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#last_modified: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#requester: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#performer_type: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#owner: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#location: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#reason_code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#reason_reference: Option<Box<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#insurance: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r4b::types::Annotation>> = None;
                let mut r#relevant_history: Option<Vec<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#restriction: Option<fhirbolt_model::r4b::resources::TaskRestriction> =
                    None;
                let mut r#input: Option<Vec<fhirbolt_model::r4b::resources::TaskInput>> = None;
                let mut r#output: Option<Vec<fhirbolt_model::r4b::resources::TaskOutput>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Task" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Task",
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
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r4b::Resource>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::Resource>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Identifier >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Identifier>(),
                                )?);
                            }
                        }
                        Field::InstantiatesCanonical => {
                            if self.0.from_json {
                                let some =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#instantiates_canonical.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                r#instantiates_canonical = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Canonical>(),
                                )?);
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesCanonical",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instantiatesCanonical");
                            }
                        }
                        Field::InstantiatesUri => {
                            if self.0.from_json {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#instantiates_uri.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                r#instantiates_uri = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instantiatesUri");
                            }
                        }
                        Field::BasedOn => {
                            if self.0.from_json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Reference >> ()) ?) ;
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Reference>(),
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
                                        .transmute::<Box<fhirbolt_model::r4b::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::PartOf => {
                            if self.0.from_json {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Reference >> ()) ?) ;
                            } else {
                                let vec = r#part_of.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Reference>(),
                                )?);
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
                        Field::StatusReason => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusReason"));
                            }
                            r#status_reason = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::BusinessStatus => {
                            if r#business_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("businessStatus"));
                            }
                            r#business_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                        Field::Focus => {
                            if r#focus.is_some() {
                                return Err(serde::de::Error::duplicate_field("focus"));
                            }
                            r#focus = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::For => {
                            if r#for.is_some() {
                                return Err(serde::de::Error::duplicate_field("for"));
                            }
                            r#for = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
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
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ExecutionPeriod => {
                            if r#execution_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionPeriod"));
                            }
                            r#execution_period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
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
                        Field::LastModified => {
                            if self.0.from_json {
                                let some = r#last_modified.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastModified"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#last_modified.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastModified"));
                                }
                                r#last_modified = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::LastModifiedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#last_modified.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lastModified"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastModified");
                            }
                        }
                        Field::Requester => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::PerformerType => {
                            if self.0.from_json {
                                if r#performer_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerType"));
                                }
                                r#performer_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#performer_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Owner => {
                            if r#owner.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            r#owner = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ReasonReference => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Insurance => {
                            if self.0.from_json {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Reference >> ()) ?) ;
                            } else {
                                let vec = r#insurance.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Annotation >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Annotation>(),
                                )?);
                            }
                        }
                        Field::RelevantHistory => {
                            if self.0.from_json {
                                if r#relevant_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relevantHistory",
                                    ));
                                }
                                r#relevant_history = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Reference >> ()) ?) ;
                            } else {
                                let vec = r#relevant_history.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Restriction => {
                            if r#restriction.is_some() {
                                return Err(serde::de::Error::duplicate_field("restriction"));
                            }
                            r#restriction = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: TaskRestriction > ()) ?) ;
                        }
                        Field::Input => {
                            if self.0.from_json {
                                if r#input.is_some() {
                                    return Err(serde::de::Error::duplicate_field("input"));
                                }
                                r#input = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: TaskInput >> ()) ?) ;
                            } else {
                                let vec = r#input.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::resources::TaskInput>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Output => {
                            if self.0.from_json {
                                if r#output.is_some() {
                                    return Err(serde::de::Error::duplicate_field("output"));
                                }
                                r#output = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: TaskOutput >> ()) ?) ;
                            } else {
                                let vec = r#output.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: TaskOutput > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::Task {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical,
                    r#instantiates_uri,
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#group_identifier,
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#status_reason,
                    r#business_status,
                    r#intent: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#intent.unwrap_or(Default::default())
                    } else {
                        r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                    },
                    r#priority,
                    r#code,
                    r#description,
                    r#focus,
                    r#for,
                    r#encounter,
                    r#execution_period,
                    r#authored_on,
                    r#last_modified,
                    r#requester,
                    r#performer_type: r#performer_type.unwrap_or(vec![]),
                    r#owner,
                    r#location,
                    r#reason_code,
                    r#reason_reference,
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                    r#restriction,
                    r#input: r#input.unwrap_or(vec![]),
                    r#output: r#output.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4b::resources::Task>>
{
    type Value = Box<fhirbolt_model::r4b::resources::Task>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::Task>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4b::resources::Task>>
{
    type Value = Vec<fhirbolt_model::r4b::resources::Task>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::Task>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::Task>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq
                    .next_element_seed(self.0.transmute::<fhirbolt_model::r4b::resources::Task>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
