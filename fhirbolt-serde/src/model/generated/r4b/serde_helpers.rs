// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
pub struct PrimitiveElement<'a> {
    pub id: Option<&'a std::string::String>,
    pub extension: &'a Vec<fhirbolt_model::r4b::types::Extension>,
}
impl<'a> serde::ser::Serialize
    for crate::context::ser::SerializationContext<&PrimitiveElement<'a>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(id) = self.value.id {
            state.serialize_entry("id", id)?;
        }
        if !self.value.extension.is_empty() {
            self.with_context(self.value.extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        state.end()
    }
}
impl<'a> serde::ser::Serialize
    for crate::context::ser::SerializationContext<Option<&PrimitiveElement<'a>>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        if let Some(value) = self.value {
            self.with_context(value, |ctx| serializer.serialize_some(ctx))
        } else {
            serializer.serialize_none()
        }
    }
}
impl<'a> serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<Option<PrimitiveElement<'a>>>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value.as_ref(), |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
pub struct PrimitiveElementOwned {
    pub id: Option<std::string::String>,
    pub extension: Vec<fhirbolt_model::r4b::types::Extension>,
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<PrimitiveElementOwned>
{
    type Value = PrimitiveElementOwned;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<PrimitiveElementOwned>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = PrimitiveElementOwned;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive element extension")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PrimitiveElementOwned, V::Error>
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
                    Unknown(std::string::String),
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
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
                            r#extension = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Vec<fhirbolt_model::r4b::types::Extension>>(),
                                )?,
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension"],
                                ));
                            }
                        }
                    }
                }
                Ok(PrimitiveElementOwned {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Option<PrimitiveElementOwned>>
{
    type Value = Option<PrimitiveElementOwned>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<Option<PrimitiveElementOwned>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Option<PrimitiveElementOwned>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("maybe primitive element extension")
            }
            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::DeserializeSeed;
                self.0
                    .transmute::<PrimitiveElementOwned>()
                    .deserialize(deserializer)
                    .map(Some)
            }
        }
        deserializer.deserialize_option(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<Option<PrimitiveElementOwned>>>
{
    type Value = Vec<Option<PrimitiveElementOwned>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<Vec<Option<PrimitiveElementOwned>>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Option<PrimitiveElementOwned>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a primitive element extension sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<Option<PrimitiveElementOwned>>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
