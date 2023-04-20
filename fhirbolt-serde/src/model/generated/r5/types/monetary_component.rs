// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::types::MonetaryComponent>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("factor", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#factor.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
            }
        }
        if let Some(some) = self.value.r#amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amount", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::types::MonetaryComponent>,
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
        &Vec<fhirbolt_model::r5::types::MonetaryComponent>,
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
        &Vec<Box<fhirbolt_model::r5::types::MonetaryComponent>>,
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
        fhirbolt_model::r5::types::MonetaryComponent,
    >
{
    type Value = fhirbolt_model::r5::types::MonetaryComponent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::types::MonetaryComponent,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::types::MonetaryComponent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MonetaryComponent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::types::MonetaryComponent, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "amount")]
                    Amount,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "type", "code", "factor", "amount"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Factor => {
                            if self.0.from_json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                r#factor = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Money>>(),
                            )?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::types::MonetaryComponent {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#code,
                    r#factor,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::types::MonetaryComponent>,
    >
{
    type Value = Box<fhirbolt_model::r5::types::MonetaryComponent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::types::MonetaryComponent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::types::MonetaryComponent>,
    >
{
    type Value = Vec<fhirbolt_model::r5::types::MonetaryComponent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::types::MonetaryComponent>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::types::MonetaryComponent>;
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
                        .transmute::<fhirbolt_model::r5::types::MonetaryComponent>(),
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
        Vec<Box<fhirbolt_model::r5::types::MonetaryComponent>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::types::MonetaryComponent>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::types::MonetaryComponent>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::types::MonetaryComponent>>;
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
                        .transmute::<Box<fhirbolt_model::r5::types::MonetaryComponent>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
