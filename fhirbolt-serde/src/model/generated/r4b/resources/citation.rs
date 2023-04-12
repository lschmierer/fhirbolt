// Generated on 2023-04-13 by fhirbolt-codegen v0.1.0
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationSummary>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#style.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("style", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
            }
            if self.value.r#text.id.is_some() || !self.value.r#text.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#text.id.as_ref(),
                    extension: &self.value.r#text.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_text", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#text, |ctx| state.serialize_entry("text", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationSummary>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationSummary>>
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
    for crate::SerializationContext<&Vec<Box<fhirbolt_model::r4b::resources::CitationSummary>>>
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
        fhirbolt_model::r4b::resources::CitationSummary,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationSummary;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationSummary,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationSummary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationSummary")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationSummary, V::Error>
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
                    #[serde(rename = "style")]
                    Style,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "style", "text"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#style: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#text: Option<fhirbolt_model::r4b::types::Markdown> = None;
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
                        Field::Style => {
                            if r#style.is_some() {
                                return Err(serde::de::Error::duplicate_field("style"));
                            }
                            r#style = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
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
                Ok(fhirbolt_model::r4b::resources::CitationSummary {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#style,
                    r#text: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#text.unwrap_or(Default::default())
                    } else {
                        r#text.ok_or(serde::de::Error::missing_field("text"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationSummary>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationSummary>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationSummary>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationSummary>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationSummary>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationSummary>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationSummary>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationSummary>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationSummary>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationSummary>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationSummary>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationSummary>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::CitationSummary>>(),
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
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationClassification>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#classifier.is_empty() {
            self.with_context(&self.value.r#classifier, |ctx| {
                state.serialize_entry("classifier", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationClassification>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationClassification>>
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationClassification>>,
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
        fhirbolt_model::r4b::resources::CitationClassification,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationClassification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationClassification,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationClassification, V::Error>
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
                    #[serde(rename = "classifier")]
                    Classifier,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "classifier"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#classifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Classifier => {
                            if self.0.from_json {
                                if r#classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("classifier"));
                                }
                                r#classifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#classifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::CitationClassification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#classifier: r#classifier.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationClassification>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationClassification>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationClassification>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationClassification>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationClassification>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationClassification>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationClassification>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationClassification>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationClassification>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationClassification>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::CitationClassification>>(),
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
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationStatusDate>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        self.with_context(&self.value.r#activity, |ctx| {
            state.serialize_entry("activity", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#actual.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("actual", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_actual", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#actual.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("actual", ctx))?;
            }
        }
        self.with_context(&self.value.r#period, |ctx| {
            state.serialize_entry("period", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationStatusDate>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationStatusDate>>
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
    for crate::SerializationContext<&Vec<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>>
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
        fhirbolt_model::r4b::resources::CitationStatusDate,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationStatusDate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationStatusDate,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationStatusDate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationStatusDate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationStatusDate, V::Error>
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
                    #[serde(rename = "activity")]
                    Activity,
                    #[serde(rename = "actual")]
                    Actual,
                    #[serde(rename = "_actual")]
                    ActualPrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "activity",
                            "actual",
                            "period",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#activity: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#actual: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
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
                        Field::Activity => {
                            if r#activity.is_some() {
                                return Err(serde::de::Error::duplicate_field("activity"));
                            }
                            r#activity = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Actual => {
                            if self.0.from_json {
                                let some = r#actual.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actual"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#actual.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actual"));
                                }
                                r#actual = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ActualPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#actual.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_actual"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("actual");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::CitationStatusDate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#activity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#activity.unwrap_or(Default::default())
                    } else {
                        r#activity.ok_or(serde::de::Error::missing_field("activity"))?
                    },
                    r#actual,
                    r#period: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#period.unwrap_or(Default::default())
                    } else {
                        r#period.ok_or(serde::de::Error::missing_field("period"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationStatusDate>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationStatusDate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationStatusDate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationStatusDate>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationStatusDate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationStatusDate>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationStatusDate>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationStatusDate>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::CitationStatusDate>>(),
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
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationRelatesTo>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        self.with_context(&self.value.r#relationship_type, |ctx| {
            state.serialize_entry("relationshipType", ctx)
        })?;
        if !self.value.r#target_classifier.is_empty() {
            self.with_context(&self.value.r#target_classifier, |ctx| {
                state.serialize_entry("targetClassifier", ctx)
            })?;
        }
        match self.value.r#target {
            fhirbolt_model::r4b::resources::CitationRelatesToTarget::Uri(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("targetUri", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_targetUri", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("targetUri", ctx))?;
                }
            }
            fhirbolt_model::r4b::resources::CitationRelatesToTarget::Identifier(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("targetIdentifier", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationRelatesToTarget::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("targetReference", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationRelatesToTarget::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("targetAttachment", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationRelatesToTarget::Invalid => {
                return Err(serde::ser::Error::custom("target is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>>
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
    for crate::SerializationContext<&Vec<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>>
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
        fhirbolt_model::r4b::resources::CitationRelatesTo,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationRelatesTo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationRelatesTo,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationRelatesTo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationRelatesTo, V::Error>
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
                    #[serde(rename = "relationshipType")]
                    RelationshipType,
                    #[serde(rename = "targetClassifier")]
                    TargetClassifier,
                    #[serde(rename = "targetUri")]
                    TargetUri,
                    #[serde(rename = "_targetUri")]
                    TargetUriPrimitiveElement,
                    #[serde(rename = "targetIdentifier")]
                    TargetIdentifier,
                    #[serde(rename = "targetReference")]
                    TargetReference,
                    #[serde(rename = "targetAttachment")]
                    TargetAttachment,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "relationshipType",
                            "targetClassifier",
                            "targetUri",
                            "targetIdentifier",
                            "targetReference",
                            "targetAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#relationship_type: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#target_classifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#target: Option<fhirbolt_model::r4b::resources::CitationRelatesToTarget> =
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
                        Field::RelationshipType => {
                            if r#relationship_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipType"));
                            }
                            r#relationship_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::TargetClassifier => {
                            if self.0.from_json {
                                if r#target_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetClassifier",
                                    ));
                                }
                                r#target_classifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#target_classifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::TargetUri => {
                            if self.0.from_json {
                                let r#enum = r#target.get_or_insert(
                                    fhirbolt_model::r4b::resources::CitationRelatesToTarget::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: CitationRelatesToTarget :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("targetUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("target[x]")) ; }
                            } else {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetUri"));
                                }
                                r#target = Some(
                                    fhirbolt_model::r4b::resources::CitationRelatesToTarget::Uri(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4b::types::Uri>>(
                                                ),
                                        )?,
                                    ),
                                );
                            }
                        }
                        Field::TargetUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#target.get_or_insert(
                                    fhirbolt_model::r4b::resources::CitationRelatesToTarget::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: CitationRelatesToTarget :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_targetUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_target[x]")) ; }
                            } else {
                                return unknown_field_error("targetUri");
                            }
                        }
                        Field::TargetIdentifier => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIdentifier"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationRelatesToTarget :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::TargetReference => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetReference"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationRelatesToTarget :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::TargetAttachment => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetAttachment"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationRelatesToTarget :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::CitationRelatesTo {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#relationship_type: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#relationship_type.unwrap_or(Default::default())
                    } else {
                        r#relationship_type
                            .ok_or(serde::de::Error::missing_field("relationshipType"))?
                    },
                    r#target_classifier: r#target_classifier.unwrap_or(vec![]),
                    r#target: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#target.unwrap_or(Default::default())
                    } else {
                        r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationRelatesTo>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationRelatesTo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationRelatesTo>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::CitationRelatesTo>>(),
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
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#value.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("value", &some)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        if let Some(some) = self.value.r#base_citation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("baseCitation", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactVersion,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactVersion;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactVersion,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactVersion;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactVersion")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion, V::Error>
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
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    #[serde(rename = "baseCitation")]
                    BaseCitation,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "value",
                            "baseCitation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#base_citation: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
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
                        Field::Value => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                r#value = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::BaseCitation => {
                            if r#base_citation.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseCitation"));
                            }
                            r#base_citation = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
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
                    fhirbolt_model::r4b::resources::CitationCitedArtifactVersion {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
                        },
                        r#base_citation,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>(
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactVersion>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactVersion >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        self.with_context(&self.value.r#activity, |ctx| {
            state.serialize_entry("activity", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#actual.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("actual", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_actual", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#actual.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("actual", ctx))?;
            }
        }
        self.with_context(&self.value.r#period, |ctx| {
            state.serialize_entry("period", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactStatusDate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate, V::Error>
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
                    #[serde(rename = "activity")]
                    Activity,
                    #[serde(rename = "actual")]
                    Actual,
                    #[serde(rename = "_actual")]
                    ActualPrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "activity",
                            "actual",
                            "period",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#activity: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#actual: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
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
                        Field::Activity => {
                            if r#activity.is_some() {
                                return Err(serde::de::Error::duplicate_field("activity"));
                            }
                            r#activity = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Actual => {
                            if self.0.from_json {
                                let some = r#actual.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actual"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#actual.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actual"));
                                }
                                r#actual = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ActualPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#actual.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_actual"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("actual");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#activity: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#activity.unwrap_or(Default::default())
                        } else {
                            r#activity.ok_or(serde::de::Error::missing_field("activity"))?
                        },
                        r#actual,
                        r#period: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#period.unwrap_or(Default::default())
                        } else {
                            r#period.ok_or(serde::de::Error::missing_field("period"))?
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
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactStatusDate > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactStatusDate >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
            }
            if self.value.r#text.id.is_some() || !self.value.r#text.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#text.id.as_ref(),
                    extension: &self.value.r#text.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_text", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#text, |ctx| state.serialize_entry("text", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactTitle,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactTitle;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactTitle,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactTitle;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactTitle")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle, V::Error>
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
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
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
                            "language",
                            "text",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>> =
                    None;
                let mut r#language: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#text: Option<fhirbolt_model::r4b::types::Markdown> = None;
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
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
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
                Ok(fhirbolt_model::r4b::resources::CitationCitedArtifactTitle {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#language,
                    r#text: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#text.unwrap_or(Default::default())
                    } else {
                        r#text.ok_or(serde::de::Error::missing_field("text"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactTitle >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
            }
            if self.value.r#text.id.is_some() || !self.value.r#text.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#text.id.as_ref(),
                    extension: &self.value.r#text.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_text", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#text, |ctx| state.serialize_entry("text", ctx))?;
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
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactAbstract")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract, V::Error>
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
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "copyright")]
                    Copyright,
                    #[serde(rename = "_copyright")]
                    CopyrightPrimitiveElement,
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
                            "language",
                            "text",
                            "copyright",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#language: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#text: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#copyright: Option<fhirbolt_model::r4b::types::Markdown> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#language,
                        r#text: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#text.unwrap_or(Default::default())
                        } else {
                            r#text.ok_or(serde::de::Error::missing_field("text"))?
                        },
                        r#copyright,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>(
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactAbstract >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifactPart>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#value.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("value", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_value", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#value.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("value", ctx))?;
            }
        }
        if let Some(some) = self.value.r#base_citation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("baseCitation", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactPart,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactPart;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPart,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactPart;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPart")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactPart, V::Error>
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
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
                    #[serde(rename = "baseCitation")]
                    BaseCitation,
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
                            "value",
                            "baseCitation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#base_citation: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Value => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                r#value = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("value");
                            }
                        }
                        Field::BaseCitation => {
                            if r#base_citation.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseCitation"));
                            }
                            r#base_citation = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
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
                Ok(fhirbolt_model::r4b::resources::CitationCitedArtifactPart {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#value,
                    r#base_citation,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPart>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPart >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        self.with_context(&self.value.r#relationship_type, |ctx| {
            state.serialize_entry("relationshipType", ctx)
        })?;
        if !self.value.r#target_classifier.is_empty() {
            self.with_context(&self.value.r#target_classifier, |ctx| {
                state.serialize_entry("targetClassifier", ctx)
            })?;
        }
        match self.value.r#target {
            fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget::Uri(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("targetUri", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_targetUri", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("targetUri", ctx))?;
                }
            }
            fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget::Identifier(
                ref value,
            ) => {
                self.with_context(value, |ctx| state.serialize_entry("targetIdentifier", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget::Reference(
                ref value,
            ) => {
                self.with_context(value, |ctx| state.serialize_entry("targetReference", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget::Attachment(
                ref value,
            ) => {
                self.with_context(value, |ctx| state.serialize_entry("targetAttachment", ctx))?;
            }
            fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget::Invalid => {
                return Err(serde::ser::Error::custom("target is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactRelatesTo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo, V::Error>
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
                    #[serde(rename = "relationshipType")]
                    RelationshipType,
                    #[serde(rename = "targetClassifier")]
                    TargetClassifier,
                    #[serde(rename = "targetUri")]
                    TargetUri,
                    #[serde(rename = "_targetUri")]
                    TargetUriPrimitiveElement,
                    #[serde(rename = "targetIdentifier")]
                    TargetIdentifier,
                    #[serde(rename = "targetReference")]
                    TargetReference,
                    #[serde(rename = "targetAttachment")]
                    TargetAttachment,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "relationshipType",
                            "targetClassifier",
                            "targetUri",
                            "targetIdentifier",
                            "targetReference",
                            "targetAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#relationship_type: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#target_classifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#target: Option<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesToTarget,
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
                        Field::RelationshipType => {
                            if r#relationship_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationshipType"));
                            }
                            r#relationship_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::TargetClassifier => {
                            if self.0.from_json {
                                if r#target_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetClassifier",
                                    ));
                                }
                                r#target_classifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#target_classifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::TargetUri => {
                            if self.0.from_json {
                                let r#enum = r#target . get_or_insert (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("targetUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("target[x]")) ; }
                            } else {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetUri"));
                                }
                                r#target = Some (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::TargetUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#target . get_or_insert (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_targetUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_target[x]")) ; }
                            } else {
                                return unknown_field_error("targetUri");
                            }
                        }
                        Field::TargetIdentifier => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIdentifier"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::TargetReference => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetReference"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::TargetAttachment => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetAttachment"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesToTarget :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Attachment > > ()) ?)) ;
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
                    fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationship_type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#relationship_type.unwrap_or(Default::default())
                        } else {
                            r#relationship_type
                                .ok_or(serde::de::Error::missing_field("relationshipType"))?
                        },
                        r#target_classifier: r#target_classifier.unwrap_or(vec![]),
                        r#target: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#target.unwrap_or(Default::default())
                        } else {
                            r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
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
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesTo > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesTo >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
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
        if let Some(some) = self.value.r#publisher.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#publisher_location.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("publisherLocation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_publisherLocation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#publisher_location.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("publisherLocation", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationFormPublishedIn")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "publisher")]
                    Publisher,
                    #[serde(rename = "publisherLocation")]
                    PublisherLocation,
                    #[serde(rename = "_publisherLocation")]
                    PublisherLocationPrimitiveElement,
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
                            "identifier",
                            "title",
                            "publisher",
                            "publisherLocation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#title: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#publisher: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#publisher_location: Option<fhirbolt_model::r4b::types::String> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                        Field::Publisher => {
                            if r#publisher.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            r#publisher = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::PublisherLocation => {
                            if self.0.from_json {
                                let some = r#publisher_location.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publisherLocation",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#publisher_location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publisherLocation",
                                    ));
                                }
                                r#publisher_location = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PublisherLocationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#publisher_location.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_publisherLocation",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publisherLocation");
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
                Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPublishedIn { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type , r#identifier : r#identifier . unwrap_or (vec ! []) , r#title , r#publisher , r#publisher_location , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>,
    >
{
    type Value =
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPublishedIn > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
                >,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPublishedIn > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPublishedIn >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
                >,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
                >>())? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde :: ser :: Serialize for crate :: SerializationContext < & fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } if self . output_json { if let Some (some) = self . value . r#date . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("date" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_date" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#date . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("date" , ctx)) ? ; } } if self . output_json { if let Some (some) = self . value . r#year . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("year" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_year" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#year . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("year" , ctx)) ? ; } } if self . output_json { if let Some (some) = self . value . r#month . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("month" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_month" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#month . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("month" , ctx)) ? ; } } if self . output_json { if let Some (some) = self . value . r#day . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("day" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_day" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#day . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("day" , ctx)) ? ; } } if self . output_json { if let Some (some) = self . value . r#season . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("season" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_season" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#season . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("season" , ctx)) ? ; } } if self . output_json { if let Some (some) = self . value . r#text . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("text" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_text" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#text . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("text" , ctx)) ? ; } } state . end () } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > { type Value = fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "date")] Date , # [serde (rename = "_date")] DatePrimitiveElement , # [serde (rename = "year")] Year , # [serde (rename = "_year")] YearPrimitiveElement , # [serde (rename = "month")] Month , # [serde (rename = "_month")] MonthPrimitiveElement , # [serde (rename = "day")] Day , # [serde (rename = "_day")] DayPrimitiveElement , # [serde (rename = "season")] Season , # [serde (rename = "_season")] SeasonPrimitiveElement , # [serde (rename = "text")] Text , # [serde (rename = "_text")] TextPrimitiveElement , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "date" , "year" , "month" , "day" , "season" , "text" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#date : Option < fhirbolt_model :: r4b :: types :: Date > = None ; let mut r#year : Option < fhirbolt_model :: r4b :: types :: String > = None ; let mut r#month : Option < fhirbolt_model :: r4b :: types :: String > = None ; let mut r#day : Option < fhirbolt_model :: r4b :: types :: String > = None ; let mut r#season : Option < fhirbolt_model :: r4b :: types :: String > = None ; let mut r#text : Option < fhirbolt_model :: r4b :: types :: String > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: Date => { if self . 0 . from_json { let some = r#date . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("date")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#date . is_some () { return Err (serde :: de :: Error :: duplicate_field ("date")) ; } r#date = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: Date > ()) ?) ; } } , Field :: DatePrimitiveElement => { if self . 0 . from_json { let some = r#date . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_date")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("date") ; } } , Field :: Year => { if self . 0 . from_json { let some = r#year . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("year")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#year . is_some () { return Err (serde :: de :: Error :: duplicate_field ("year")) ; } r#year = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: YearPrimitiveElement => { if self . 0 . from_json { let some = r#year . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_year")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("year") ; } } , Field :: Month => { if self . 0 . from_json { let some = r#month . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("month")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#month . is_some () { return Err (serde :: de :: Error :: duplicate_field ("month")) ; } r#month = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: MonthPrimitiveElement => { if self . 0 . from_json { let some = r#month . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_month")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("month") ; } } , Field :: Day => { if self . 0 . from_json { let some = r#day . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("day")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#day . is_some () { return Err (serde :: de :: Error :: duplicate_field ("day")) ; } r#day = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: DayPrimitiveElement => { if self . 0 . from_json { let some = r#day . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_day")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("day") ; } } , Field :: Season => { if self . 0 . from_json { let some = r#season . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("season")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#season . is_some () { return Err (serde :: de :: Error :: duplicate_field ("season")) ; } r#season = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: SeasonPrimitiveElement => { if self . 0 . from_json { let some = r#season . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_season")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("season") ; } } , Field :: Text => { if self . 0 . from_json { let some = r#text . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("text")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#text . is_some () { return Err (serde :: de :: Error :: duplicate_field ("text")) ; } r#text = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: TextPrimitiveElement => { if self . 0 . from_json { let some = r#text . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_text")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("text") ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#date , r#year , r#month , r#day , r#season , r#text , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> { type Value = Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#cited_medium.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("citedMedium", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#volume.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("volume", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_volume", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#volume.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("volume", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#issue.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("issue", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_issue", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#issue.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("issue", ctx))?;
            }
        }
        if let Some(some) = self.value.r#date_of_publication.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("dateOfPublication", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>,
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
    for crate::SerializationContext<
        &Vec<
            Box<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
            >,
        >,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
    >
{
    type Value =
        fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationFormPeriodicRelease")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
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
                    #[serde(rename = "citedMedium")]
                    CitedMedium,
                    #[serde(rename = "volume")]
                    Volume,
                    #[serde(rename = "_volume")]
                    VolumePrimitiveElement,
                    #[serde(rename = "issue")]
                    Issue,
                    #[serde(rename = "_issue")]
                    IssuePrimitiveElement,
                    #[serde(rename = "dateOfPublication")]
                    DateOfPublication,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "citedMedium",
                            "volume",
                            "issue",
                            "dateOfPublication",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#cited_medium: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#volume: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#issue: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#date_of_publication : Option < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > = None ;
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
                        Field::CitedMedium => {
                            if r#cited_medium.is_some() {
                                return Err(serde::de::Error::duplicate_field("citedMedium"));
                            }
                            r#cited_medium = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Volume => {
                            if self.0.from_json {
                                let some = r#volume.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("volume"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#volume.is_some() {
                                    return Err(serde::de::Error::duplicate_field("volume"));
                                }
                                r#volume = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::VolumePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#volume.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_volume"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("volume");
                            }
                        }
                        Field::Issue => {
                            if self.0.from_json {
                                let some = r#issue.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issue"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#issue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issue"));
                                }
                                r#issue = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::IssuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#issue.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issue"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issue");
                            }
                        }
                        Field::DateOfPublication => {
                            if r#date_of_publication.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateOfPublication"));
                            }
                            r#date_of_publication = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#cited_medium , r#volume , r#issue , r#date_of_publication , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>,
    >
{
    type Value =
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<
            Box<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease,
            >,
        >,
    >
{
    type Value = Vec<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPeriodicRelease>,
    >;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease >> ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#published_in.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publishedIn", ctx))?;
        }
        if let Some(some) = self.value.r#periodic_release.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("periodicRelease", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#article_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("articleDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_articleDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#article_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("articleDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#last_revision_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lastRevisionDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastRevisionDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#last_revision_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lastRevisionDate", ctx))?;
            }
        }
        if !self.value.r#language.is_empty() {
            self.with_context(&self.value.r#language, |ctx| {
                state.serialize_entry("language", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#accession_number.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("accessionNumber", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_accessionNumber", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#accession_number.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("accessionNumber", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#page_string.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("pageString", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_pageString", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#page_string.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("pageString", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#first_page.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("firstPage", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_firstPage", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#first_page.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("firstPage", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#last_page.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lastPage", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastPage", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#last_page.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lastPage", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#page_count.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("pageCount", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_pageCount", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#page_count.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("pageCount", ctx))?;
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
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactPublicationForm")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm,
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
                    #[serde(rename = "publishedIn")]
                    PublishedIn,
                    #[serde(rename = "periodicRelease")]
                    PeriodicRelease,
                    #[serde(rename = "articleDate")]
                    ArticleDate,
                    #[serde(rename = "_articleDate")]
                    ArticleDatePrimitiveElement,
                    #[serde(rename = "lastRevisionDate")]
                    LastRevisionDate,
                    #[serde(rename = "_lastRevisionDate")]
                    LastRevisionDatePrimitiveElement,
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "accessionNumber")]
                    AccessionNumber,
                    #[serde(rename = "_accessionNumber")]
                    AccessionNumberPrimitiveElement,
                    #[serde(rename = "pageString")]
                    PageString,
                    #[serde(rename = "_pageString")]
                    PageStringPrimitiveElement,
                    #[serde(rename = "firstPage")]
                    FirstPage,
                    #[serde(rename = "_firstPage")]
                    FirstPagePrimitiveElement,
                    #[serde(rename = "lastPage")]
                    LastPage,
                    #[serde(rename = "_lastPage")]
                    LastPagePrimitiveElement,
                    #[serde(rename = "pageCount")]
                    PageCount,
                    #[serde(rename = "_pageCount")]
                    PageCountPrimitiveElement,
                    #[serde(rename = "copyright")]
                    Copyright,
                    #[serde(rename = "_copyright")]
                    CopyrightPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "publishedIn",
                            "periodicRelease",
                            "articleDate",
                            "lastRevisionDate",
                            "language",
                            "accessionNumber",
                            "pageString",
                            "firstPage",
                            "lastPage",
                            "pageCount",
                            "copyright",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#published_in: Option<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationFormPublishedIn,
                > = None;
                let mut r#periodic_release : Option < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease > = None ;
                let mut r#article_date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#last_revision_date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#language: Option<Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>> =
                    None;
                let mut r#accession_number: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#page_string: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#first_page: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#last_page: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#page_count: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#copyright: Option<fhirbolt_model::r4b::types::Markdown> = None;
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
                        Field::PublishedIn => {
                            if r#published_in.is_some() {
                                return Err(serde::de::Error::duplicate_field("publishedIn"));
                            }
                            r#published_in = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPublishedIn > ()) ?) ;
                        }
                        Field::PeriodicRelease => {
                            if r#periodic_release.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodicRelease"));
                            }
                            r#periodic_release = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationFormPeriodicRelease > ()) ?) ;
                        }
                        Field::ArticleDate => {
                            if self.0.from_json {
                                let some = r#article_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("articleDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#article_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("articleDate"));
                                }
                                r#article_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::ArticleDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#article_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_articleDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("articleDate");
                            }
                        }
                        Field::LastRevisionDate => {
                            if self.0.from_json {
                                let some = r#last_revision_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastRevisionDate",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#last_revision_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastRevisionDate",
                                    ));
                                }
                                r#last_revision_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::LastRevisionDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#last_revision_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastRevisionDate",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastRevisionDate");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#language.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::AccessionNumber => {
                            if self.0.from_json {
                                let some = r#accession_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "accessionNumber",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#accession_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "accessionNumber",
                                    ));
                                }
                                r#accession_number = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::AccessionNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#accession_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_accessionNumber",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("accessionNumber");
                            }
                        }
                        Field::PageString => {
                            if self.0.from_json {
                                let some = r#page_string.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pageString"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#page_string.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pageString"));
                                }
                                r#page_string = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PageStringPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#page_string.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_pageString"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("pageString");
                            }
                        }
                        Field::FirstPage => {
                            if self.0.from_json {
                                let some = r#first_page.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("firstPage"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#first_page.is_some() {
                                    return Err(serde::de::Error::duplicate_field("firstPage"));
                                }
                                r#first_page = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::FirstPagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#first_page.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_firstPage"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("firstPage");
                            }
                        }
                        Field::LastPage => {
                            if self.0.from_json {
                                let some = r#last_page.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastPage"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#last_page.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastPage"));
                                }
                                r#last_page = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::LastPagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#last_page.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lastPage"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastPage");
                            }
                        }
                        Field::PageCount => {
                            if self.0.from_json {
                                let some = r#page_count.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pageCount"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#page_count.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pageCount"));
                                }
                                r#page_count = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PageCountPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#page_count.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_pageCount"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("pageCount");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#published_in,
                        r#periodic_release,
                        r#article_date,
                        r#last_revision_date,
                        r#language: r#language.unwrap_or(vec![]),
                        r#accession_number,
                        r#page_string,
                        r#first_page,
                        r#last_page,
                        r#page_count,
                        r#copyright,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationForm > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>>;
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
                            fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm,
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
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#url.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactWebLocation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation, V::Error>
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
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "url"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#url: Option<fhirbolt_model::r4b::types::Uri> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#url,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactWebLocation > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactWebLocation >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#person.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("person", ctx))?;
        }
        if let Some(some) = self.value.r#organization.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("organization", ctx))?;
        }
        if let Some(some) = self.value.r#publisher.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#classifier_copyright.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("classifierCopyright", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_classifierCopyright", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#classifier_copyright.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("classifierCopyright", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#free_to_share.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("freeToShare", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_freeToShare", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#free_to_share.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("freeToShare", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactClassificationWhoClassified")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified,
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
                    #[serde(rename = "person")]
                    Person,
                    #[serde(rename = "organization")]
                    Organization,
                    #[serde(rename = "publisher")]
                    Publisher,
                    #[serde(rename = "classifierCopyright")]
                    ClassifierCopyright,
                    #[serde(rename = "_classifierCopyright")]
                    ClassifierCopyrightPrimitiveElement,
                    #[serde(rename = "freeToShare")]
                    FreeToShare,
                    #[serde(rename = "_freeToShare")]
                    FreeToSharePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "person",
                            "organization",
                            "publisher",
                            "classifierCopyright",
                            "freeToShare",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#person: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#organization: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#publisher: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#classifier_copyright: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#free_to_share: Option<fhirbolt_model::r4b::types::Boolean> = None;
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
                        Field::Person => {
                            if r#person.is_some() {
                                return Err(serde::de::Error::duplicate_field("person"));
                            }
                            r#person = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Organization => {
                            if r#organization.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            r#organization = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Publisher => {
                            if r#publisher.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            r#publisher = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ClassifierCopyright => {
                            if self.0.from_json {
                                let some = r#classifier_copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classifierCopyright",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#classifier_copyright.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classifierCopyright",
                                    ));
                                }
                                r#classifier_copyright = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::ClassifierCopyrightPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#classifier_copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_classifierCopyright",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("classifierCopyright");
                            }
                        }
                        Field::FreeToShare => {
                            if self.0.from_json {
                                let some = r#free_to_share.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("freeToShare"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#free_to_share.is_some() {
                                    return Err(serde::de::Error::duplicate_field("freeToShare"));
                                }
                                r#free_to_share = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::FreeToSharePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#free_to_share.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_freeToShare"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("freeToShare");
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
                Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#person , r#organization , r#publisher , r#classifier_copyright , r#free_to_share , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>,
    >
{
    type Value =
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassificationWhoClassified>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified >> ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactClassification,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#classifier.is_empty() {
            self.with_context(&self.value.r#classifier, |ctx| {
                state.serialize_entry("classifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#who_classified.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("whoClassified", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactClassification,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactClassification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactClassification,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification, V::Error>
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
                    #[serde(rename = "classifier")]
                    Classifier,
                    #[serde(rename = "whoClassified")]
                    WhoClassified,
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
                            "classifier",
                            "whoClassified",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#classifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#who_classified : Option < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified > = None ;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Classifier => {
                            if self.0.from_json {
                                if r#classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("classifier"));
                                }
                                r#classifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#classifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::WhoClassified => {
                            if r#who_classified.is_some() {
                                return Err(serde::de::Error::duplicate_field("whoClassified"));
                            }
                            r#who_classified = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassificationWhoClassified > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::CitationCitedArtifactClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#classifier: r#classifier.unwrap_or(vec![]),
                        r#who_classified,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassification > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>>;
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
                            fhirbolt_model::r4b::resources::CitationCitedArtifactClassification,
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
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntryAffiliationInfo,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#affiliation.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("affiliation", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_affiliation", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#affiliation.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("affiliation", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#role.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("role", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_role", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#role.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
            }
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        state.end()
    }
}
impl serde :: ser :: Serialize for crate :: SerializationContext < & Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntryAffiliationInfo,
    >
{
    type Value =
        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntryAffiliationInfo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipEntryAffiliationInfo")
            }            fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo , V :: Error > where V : serde :: de :: MapAccess < 'de > ,{
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "affiliation")]
                    Affiliation,
                    #[serde(rename = "_affiliation")]
                    AffiliationPrimitiveElement,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "_role")]
                    RolePrimitiveElement,
                    #[serde(rename = "identifier")]
                    Identifier,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "affiliation",
                            "role",
                            "identifier",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#affiliation: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#role: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
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
                        Field::Affiliation => {
                            if self.0.from_json {
                                let some = r#affiliation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("affiliation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#affiliation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("affiliation"));
                                }
                                r#affiliation = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::AffiliationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#affiliation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_affiliation"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("affiliation");
                            }
                        }
                        Field::Role => {
                            if self.0.from_json {
                                let some = r#role.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::RolePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#role.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_role"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("role");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#affiliation , r#role , r#identifier : r#identifier . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> { type Value = Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } self . with_context (& self . value . r#type , | ctx | state . serialize_entry ("type" , ctx)) ? ; if self . output_json { if let Some (some) = self . value . r#time . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("time" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_time" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#time . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("time" , ctx)) ? ; } } state . end () } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl serde :: ser :: Serialize for crate :: SerializationContext < & Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > { type Value = fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("CitationCitedArtifactContributorshipEntryContributionInstance") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "type")] Type , # [serde (rename = "time")] Time , # [serde (rename = "_time")] TimePrimitiveElement , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "type" , "time" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#type : Option < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > = None ; let mut r#time : Option < fhirbolt_model :: r4b :: types :: DateTime > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ; } , Field :: Time => { if self . 0 . from_json { let some = r#time . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("time")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#time . is_some () { return Err (serde :: de :: Error :: duplicate_field ("time")) ; } r#time = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: DateTime > ()) ?) ; } } , Field :: TimePrimitiveElement => { if self . 0 . from_json { let some = r#time . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_time")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("time") ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#time , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> { type Value = Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#initials.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("initials", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_initials", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#initials.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("initials", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#collective_name.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("collectiveName", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_collectiveName", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#collective_name.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("collectiveName", ctx))?;
            }
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if !self.value.r#affiliation_info.is_empty() {
            self.with_context(&self.value.r#affiliation_info, |ctx| {
                state.serialize_entry("affiliationInfo", ctx)
            })?;
        }
        if !self.value.r#address.is_empty() {
            self.with_context(&self.value.r#address, |ctx| {
                state.serialize_entry("address", ctx)
            })?;
        }
        if !self.value.r#telecom.is_empty() {
            self.with_context(&self.value.r#telecom, |ctx| {
                state.serialize_entry("telecom", ctx)
            })?;
        }
        if !self.value.r#contribution_type.is_empty() {
            self.with_context(&self.value.r#contribution_type, |ctx| {
                state.serialize_entry("contributionType", ctx)
            })?;
        }
        if let Some(some) = self.value.r#role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
        }
        if !self.value.r#contribution_instance.is_empty() {
            self.with_context(&self.value.r#contribution_instance, |ctx| {
                state.serialize_entry("contributionInstance", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#corresponding_contact.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("correspondingContact", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_correspondingContact", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#corresponding_contact.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("correspondingContact", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#list_order.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("listOrder", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_listOrder", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#list_order.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("listOrder", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipEntry")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry,
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "initials")]
                    Initials,
                    #[serde(rename = "_initials")]
                    InitialsPrimitiveElement,
                    #[serde(rename = "collectiveName")]
                    CollectiveName,
                    #[serde(rename = "_collectiveName")]
                    CollectiveNamePrimitiveElement,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "affiliationInfo")]
                    AffiliationInfo,
                    #[serde(rename = "address")]
                    Address,
                    #[serde(rename = "telecom")]
                    Telecom,
                    #[serde(rename = "contributionType")]
                    ContributionType,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "contributionInstance")]
                    ContributionInstance,
                    #[serde(rename = "correspondingContact")]
                    CorrespondingContact,
                    #[serde(rename = "_correspondingContact")]
                    CorrespondingContactPrimitiveElement,
                    #[serde(rename = "listOrder")]
                    ListOrder,
                    #[serde(rename = "_listOrder")]
                    ListOrderPrimitiveElement,
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
                            "initials",
                            "collectiveName",
                            "identifier",
                            "affiliationInfo",
                            "address",
                            "telecom",
                            "contributionType",
                            "role",
                            "contributionInstance",
                            "correspondingContact",
                            "listOrder",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#name: Option<Box<fhirbolt_model::r4b::types::HumanName>> = None;
                let mut r#initials: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#collective_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#affiliation_info : Option < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > > = None ;
                let mut r#address: Option<Vec<Box<fhirbolt_model::r4b::types::Address>>> = None;
                let mut r#telecom: Option<Vec<Box<fhirbolt_model::r4b::types::ContactPoint>>> =
                    None;
                let mut r#contribution_type: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#role: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#contribution_instance : Option < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > > = None ;
                let mut r#corresponding_contact: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#list_order: Option<fhirbolt_model::r4b::types::PositiveInt> = None;
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
                            if r#name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            r#name = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::HumanName>>(),
                                )?,
                            );
                        }
                        Field::Initials => {
                            if self.0.from_json {
                                let some = r#initials.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("initials"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#initials.is_some() {
                                    return Err(serde::de::Error::duplicate_field("initials"));
                                }
                                r#initials = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::InitialsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#initials.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_initials"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("initials");
                            }
                        }
                        Field::CollectiveName => {
                            if self.0.from_json {
                                let some = r#collective_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "collectiveName",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#collective_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "collectiveName",
                                    ));
                                }
                                r#collective_name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::CollectiveNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#collective_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_collectiveName",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("collectiveName");
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
                        Field::AffiliationInfo => {
                            if self.0.from_json {
                                if r#affiliation_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "affiliationInfo",
                                    ));
                                }
                                r#affiliation_info = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo >> ()) ?) ;
                            } else {
                                let vec = r#affiliation_info.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryAffiliationInfo > ()) ?) ;
                            }
                        }
                        Field::Address => {
                            if self.0.from_json {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                r#address = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Address > >> ()) ?) ;
                            } else {
                                let vec = r#address.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Address>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Telecom => {
                            if self.0.from_json {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactPoint > >> ()) ?) ;
                            } else {
                                let vec = r#telecom.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?) ;
                            }
                        }
                        Field::ContributionType => {
                            if self.0.from_json {
                                if r#contribution_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributionType",
                                    ));
                                }
                                r#contribution_type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#contribution_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ContributionInstance => {
                            if self.0.from_json {
                                if r#contribution_instance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributionInstance",
                                    ));
                                }
                                r#contribution_instance = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance >> ()) ?) ;
                            } else {
                                let vec = r#contribution_instance.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntryContributionInstance > ()) ?) ;
                            }
                        }
                        Field::CorrespondingContact => {
                            if self.0.from_json {
                                let some =
                                    r#corresponding_contact.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "correspondingContact",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#corresponding_contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "correspondingContact",
                                    ));
                                }
                                r#corresponding_contact = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::CorrespondingContactPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#corresponding_contact.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_correspondingContact",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("correspondingContact");
                            }
                        }
                        Field::ListOrder => {
                            if self.0.from_json {
                                let some = r#list_order.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listOrder"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#list_order.is_some() {
                                    return Err(serde::de::Error::duplicate_field("listOrder"));
                                }
                                r#list_order = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4b::types::PositiveInt>(),
                                    )?,
                                );
                            }
                        }
                        Field::ListOrderPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#list_order.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_listOrder"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("listOrder");
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
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name,
                        r#initials,
                        r#collective_name,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#affiliation_info: r#affiliation_info.unwrap_or(vec![]),
                        r#address: r#address.unwrap_or(vec![]),
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#contribution_type: r#contribution_type.unwrap_or(vec![]),
                        r#role,
                        r#contribution_instance: r#contribution_instance.unwrap_or(vec![]),
                        r#corresponding_contact,
                        r#list_order,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>(
        )
        .deserialize(deserializer)
        .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntry > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry,
                >>())? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#style.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("style", ctx))?;
        }
        if let Some(some) = self.value.r#source.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("source", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#value.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("value", &some)?;
            }
            if self.value.r#value.id.is_some() || !self.value.r#value.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#value.id.as_ref(),
                    extension: &self.value.r#value.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_value", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorshipSummary")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
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
                    #[serde(rename = "style")]
                    Style,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "value")]
                    Value,
                    #[serde(rename = "_value")]
                    ValuePrimitiveElement,
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
                            "style",
                            "source",
                            "value",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#style: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#source: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::Markdown> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Style => {
                            if r#style.is_some() {
                                return Err(serde::de::Error::duplicate_field("style"));
                            }
                            r#style = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Value => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                r#value = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                Ok(
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#style,
                        r#source,
                        r#value: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value"))?
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
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipSummary > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipSummary > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<
                    Box<
                        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
                    >,
                >,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary>,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
                >>())? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
    >
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
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#complete.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("complete", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_complete", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#complete.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("complete", ctx))?;
            }
        }
        if !self.value.r#entry.is_empty() {
            self.with_context(&self.value.r#entry, |ctx| {
                state.serialize_entry("entry", ctx)
            })?;
        }
        if !self.value.r#summary.is_empty() {
            self.with_context(&self.value.r#summary, |ctx| {
                state.serialize_entry("summary", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>,
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifactContributorship")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
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
                    #[serde(rename = "complete")]
                    Complete,
                    #[serde(rename = "_complete")]
                    CompletePrimitiveElement,
                    #[serde(rename = "entry")]
                    Entry,
                    #[serde(rename = "summary")]
                    Summary,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "complete",
                            "entry",
                            "summary",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#complete: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#entry: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipEntry>,
                > = None;
                let mut r#summary: Option<
                    Vec<
                        fhirbolt_model::r4b::resources::CitationCitedArtifactContributorshipSummary,
                    >,
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
                        Field::Complete => {
                            if self.0.from_json {
                                let some = r#complete.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("complete"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#complete.is_some() {
                                    return Err(serde::de::Error::duplicate_field("complete"));
                                }
                                r#complete = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::CompletePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#complete.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_complete"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("complete");
                            }
                        }
                        Field::Entry => {
                            if self.0.from_json {
                                if r#entry.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entry"));
                                }
                                r#entry = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntry >> ()) ?) ;
                            } else {
                                let vec = r#entry.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipEntry > ()) ?) ;
                            }
                        }
                        Field::Summary => {
                            if self.0.from_json {
                                if r#summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("summary"));
                                }
                                r#summary = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipSummary >> ()) ?) ;
                            } else {
                                let vec = r#summary.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorshipSummary > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#complete,
                        r#entry: r#entry.unwrap_or(vec![]),
                        r#summary: r#summary.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorship > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship>>;
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
                            fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
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
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::CitationCitedArtifact>
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
        if !self.value.r#related_identifier.is_empty() {
            self.with_context(&self.value.r#related_identifier, |ctx| {
                state.serialize_entry("relatedIdentifier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#date_accessed.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("dateAccessed", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_dateAccessed", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#date_accessed.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("dateAccessed", ctx))?;
            }
        }
        if let Some(some) = self.value.r#version.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
        }
        if !self.value.r#current_state.is_empty() {
            self.with_context(&self.value.r#current_state, |ctx| {
                state.serialize_entry("currentState", ctx)
            })?;
        }
        if !self.value.r#status_date.is_empty() {
            self.with_context(&self.value.r#status_date, |ctx| {
                state.serialize_entry("statusDate", ctx)
            })?;
        }
        if !self.value.r#title.is_empty() {
            self.with_context(&self.value.r#title, |ctx| {
                state.serialize_entry("title", ctx)
            })?;
        }
        if !self.value.r#abstract.is_empty() {
            self.with_context(&self.value.r#abstract, |ctx| {
                state.serialize_entry("abstract", ctx)
            })?;
        }
        if let Some(some) = self.value.r#part.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("part", ctx))?;
        }
        if !self.value.r#relates_to.is_empty() {
            self.with_context(&self.value.r#relates_to, |ctx| {
                state.serialize_entry("relatesTo", ctx)
            })?;
        }
        if !self.value.r#publication_form.is_empty() {
            self.with_context(&self.value.r#publication_form, |ctx| {
                state.serialize_entry("publicationForm", ctx)
            })?;
        }
        if !self.value.r#web_location.is_empty() {
            self.with_context(&self.value.r#web_location, |ctx| {
                state.serialize_entry("webLocation", ctx)
            })?;
        }
        if !self.value.r#classification.is_empty() {
            self.with_context(&self.value.r#classification, |ctx| {
                state.serialize_entry("classification", ctx)
            })?;
        }
        if let Some(some) = self.value.r#contributorship.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("contributorship", ctx))?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::CitationCitedArtifact>>
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
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>,
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
        fhirbolt_model::r4b::resources::CitationCitedArtifact,
    >
{
    type Value = fhirbolt_model::r4b::resources::CitationCitedArtifact;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CitationCitedArtifact,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CitationCitedArtifact;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CitationCitedArtifact")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CitationCitedArtifact, V::Error>
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "relatedIdentifier")]
                    RelatedIdentifier,
                    #[serde(rename = "dateAccessed")]
                    DateAccessed,
                    #[serde(rename = "_dateAccessed")]
                    DateAccessedPrimitiveElement,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "currentState")]
                    CurrentState,
                    #[serde(rename = "statusDate")]
                    StatusDate,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "abstract")]
                    Abstract,
                    #[serde(rename = "part")]
                    Part,
                    #[serde(rename = "relatesTo")]
                    RelatesTo,
                    #[serde(rename = "publicationForm")]
                    PublicationForm,
                    #[serde(rename = "webLocation")]
                    WebLocation,
                    #[serde(rename = "classification")]
                    Classification,
                    #[serde(rename = "contributorship")]
                    Contributorship,
                    #[serde(rename = "note")]
                    Note,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "identifier",
                            "relatedIdentifier",
                            "dateAccessed",
                            "version",
                            "currentState",
                            "statusDate",
                            "title",
                            "abstract",
                            "part",
                            "relatesTo",
                            "publicationForm",
                            "webLocation",
                            "classification",
                            "contributorship",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#related_identifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Identifier>>,
                > = None;
                let mut r#date_accessed: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#version: Option<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactVersion,
                > = None;
                let mut r#current_state: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#status_date: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactStatusDate>,
                > = None;
                let mut r#title: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactTitle>,
                > = None;
                let mut r#abstract: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactAbstract>,
                > = None;
                let mut r#part: Option<fhirbolt_model::r4b::resources::CitationCitedArtifactPart> =
                    None;
                let mut r#relates_to: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactRelatesTo>,
                > = None;
                let mut r#publication_form: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactPublicationForm>,
                > = None;
                let mut r#web_location: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactWebLocation>,
                > = None;
                let mut r#classification: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationCitedArtifactClassification>,
                > = None;
                let mut r#contributorship: Option<
                    fhirbolt_model::r4b::resources::CitationCitedArtifactContributorship,
                > = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
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
                        Field::RelatedIdentifier => {
                            if self.0.from_json {
                                if r#related_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedIdentifier",
                                    ));
                                }
                                r#related_identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#related_identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::DateAccessed => {
                            if self.0.from_json {
                                let some = r#date_accessed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateAccessed"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#date_accessed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateAccessed"));
                                }
                                r#date_accessed = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::DateAccessedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#date_accessed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dateAccessed"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("dateAccessed");
                            }
                        }
                        Field::Version => {
                            if r#version.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            r#version = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactVersion > ()) ?) ;
                        }
                        Field::CurrentState => {
                            if self.0.from_json {
                                if r#current_state.is_some() {
                                    return Err(serde::de::Error::duplicate_field("currentState"));
                                }
                                r#current_state =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#current_state.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::StatusDate => {
                            if self.0.from_json {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactStatusDate >> ()) ?) ;
                            } else {
                                let vec = r#status_date.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactStatusDate > ()) ?) ;
                            }
                        }
                        Field::Title => {
                            if self.0.from_json {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                r#title =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::CitationCitedArtifactTitle,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#title.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactTitle > ()) ?) ;
                            }
                        }
                        Field::Abstract => {
                            if self.0.from_json {
                                if r#abstract.is_some() {
                                    return Err(serde::de::Error::duplicate_field("abstract"));
                                }
                                r#abstract = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactAbstract >> ()) ?) ;
                            } else {
                                let vec = r#abstract.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactAbstract > ()) ?) ;
                            }
                        }
                        Field::Part => {
                            if r#part.is_some() {
                                return Err(serde::de::Error::duplicate_field("part"));
                            }
                            r#part = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPart > ()) ?) ;
                        }
                        Field::RelatesTo => {
                            if self.0.from_json {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesTo >> ()) ?) ;
                            } else {
                                let vec = r#relates_to.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactRelatesTo > ()) ?) ;
                            }
                        }
                        Field::PublicationForm => {
                            if self.0.from_json {
                                if r#publication_form.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationForm",
                                    ));
                                }
                                r#publication_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationForm >> ()) ?) ;
                            } else {
                                let vec = r#publication_form.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactPublicationForm > ()) ?) ;
                            }
                        }
                        Field::WebLocation => {
                            if self.0.from_json {
                                if r#web_location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("webLocation"));
                                }
                                r#web_location = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactWebLocation >> ()) ?) ;
                            } else {
                                let vec = r#web_location.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactWebLocation > ()) ?) ;
                            }
                        }
                        Field::Classification => {
                            if self.0.from_json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassification >> ()) ?) ;
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactClassification > ()) ?) ;
                            }
                        }
                        Field::Contributorship => {
                            if r#contributorship.is_some() {
                                return Err(serde::de::Error::duplicate_field("contributorship"));
                            }
                            r#contributorship = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifactContributorship > ()) ?) ;
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::CitationCitedArtifact {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#related_identifier: r#related_identifier.unwrap_or(vec![]),
                    r#date_accessed,
                    r#version,
                    r#current_state: r#current_state.unwrap_or(vec![]),
                    r#status_date: r#status_date.unwrap_or(vec![]),
                    r#title: r#title.unwrap_or(vec![]),
                    r#abstract: r#abstract.unwrap_or(vec![]),
                    r#part,
                    r#relates_to: r#relates_to.unwrap_or(vec![]),
                    r#publication_form: r#publication_form.unwrap_or(vec![]),
                    r#web_location: r#web_location.unwrap_or(vec![]),
                    r#classification: r#classification.unwrap_or(vec![]),
                    r#contributorship,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifact>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CitationCitedArtifact>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CitationCitedArtifact>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CitationCitedArtifact>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CitationCitedArtifact>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::CitationCitedArtifact>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::Citation {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::Citation>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Citation")?;
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
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#url.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
            }
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
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#name.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
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
        if self.output_json {
            if let Some(some) = self.value.r#approval_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("approvalDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_approvalDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#approval_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("approvalDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#last_review_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lastReviewDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastReviewDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#last_review_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lastReviewDate", ctx))?;
            }
        }
        if let Some(some) = self.value.r#effective_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("effectivePeriod", ctx))?;
        }
        if !self.value.r#author.is_empty() {
            self.with_context(&self.value.r#author, |ctx| {
                state.serialize_entry("author", ctx)
            })?;
        }
        if !self.value.r#editor.is_empty() {
            self.with_context(&self.value.r#editor, |ctx| {
                state.serialize_entry("editor", ctx)
            })?;
        }
        if !self.value.r#reviewer.is_empty() {
            self.with_context(&self.value.r#reviewer, |ctx| {
                state.serialize_entry("reviewer", ctx)
            })?;
        }
        if !self.value.r#endorser.is_empty() {
            self.with_context(&self.value.r#endorser, |ctx| {
                state.serialize_entry("endorser", ctx)
            })?;
        }
        if !self.value.r#summary.is_empty() {
            self.with_context(&self.value.r#summary, |ctx| {
                state.serialize_entry("summary", ctx)
            })?;
        }
        if !self.value.r#classification.is_empty() {
            self.with_context(&self.value.r#classification, |ctx| {
                state.serialize_entry("classification", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#current_state.is_empty() {
            self.with_context(&self.value.r#current_state, |ctx| {
                state.serialize_entry("currentState", ctx)
            })?;
        }
        if !self.value.r#status_date.is_empty() {
            self.with_context(&self.value.r#status_date, |ctx| {
                state.serialize_entry("statusDate", ctx)
            })?;
        }
        if !self.value.r#relates_to.is_empty() {
            self.with_context(&self.value.r#relates_to, |ctx| {
                state.serialize_entry("relatesTo", ctx)
            })?;
        }
        if let Some(some) = self.value.r#cited_artifact.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("citedArtifact", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::Citation>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::Citation>>
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
    for crate::SerializationContext<&Vec<Box<fhirbolt_model::r4b::resources::Citation>>>
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::Citation>
{
    type Value = fhirbolt_model::r4b::resources::Citation;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::Citation>
{
    type Value = fhirbolt_model::r4b::resources::Citation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::Citation,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::Citation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Citation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::Citation, V::Error>
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
                    #[serde(rename = "approvalDate")]
                    ApprovalDate,
                    #[serde(rename = "_approvalDate")]
                    ApprovalDatePrimitiveElement,
                    #[serde(rename = "lastReviewDate")]
                    LastReviewDate,
                    #[serde(rename = "_lastReviewDate")]
                    LastReviewDatePrimitiveElement,
                    #[serde(rename = "effectivePeriod")]
                    EffectivePeriod,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "editor")]
                    Editor,
                    #[serde(rename = "reviewer")]
                    Reviewer,
                    #[serde(rename = "endorser")]
                    Endorser,
                    #[serde(rename = "summary")]
                    Summary,
                    #[serde(rename = "classification")]
                    Classification,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "currentState")]
                    CurrentState,
                    #[serde(rename = "statusDate")]
                    StatusDate,
                    #[serde(rename = "relatesTo")]
                    RelatesTo,
                    #[serde(rename = "citedArtifact")]
                    CitedArtifact,
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
                            "approvalDate",
                            "lastReviewDate",
                            "effectivePeriod",
                            "author",
                            "editor",
                            "reviewer",
                            "endorser",
                            "summary",
                            "classification",
                            "note",
                            "currentState",
                            "statusDate",
                            "relatesTo",
                            "citedArtifact",
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
                let mut r#approval_date: Option<fhirbolt_model::r4b::types::Date> = None;
                let mut r#last_review_date: Option<fhirbolt_model::r4b::types::Date> = None;
                let mut r#effective_period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#author: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#editor: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#reviewer: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#endorser: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#summary: Option<Vec<fhirbolt_model::r4b::resources::CitationSummary>> =
                    None;
                let mut r#classification: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationClassification>,
                > = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
                let mut r#current_state: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#status_date: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationStatusDate>,
                > = None;
                let mut r#relates_to: Option<
                    Vec<fhirbolt_model::r4b::resources::CitationRelatesTo>,
                > = None;
                let mut r#cited_artifact: Option<
                    fhirbolt_model::r4b::resources::CitationCitedArtifact,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Citation" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Citation",
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
                        Field::ApprovalDate => {
                            if self.0.from_json {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#approval_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                r#approval_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Date>(),
                                )?);
                            }
                        }
                        Field::ApprovalDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_approvalDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("approvalDate");
                            }
                        }
                        Field::LastReviewDate => {
                            if self.0.from_json {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#last_review_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                r#last_review_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Date>(),
                                )?);
                            }
                        }
                        Field::LastReviewDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastReviewDate",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastReviewDate");
                            }
                        }
                        Field::EffectivePeriod => {
                            if r#effective_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            r#effective_period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                )?,
                            );
                        }
                        Field::Author => {
                            if self.0.from_json {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#author.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Editor => {
                            if self.0.from_json {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                r#editor = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#editor.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Reviewer => {
                            if self.0.from_json {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                r#reviewer = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#reviewer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Endorser => {
                            if self.0.from_json {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                r#endorser = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#endorser.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Summary => {
                            if self.0.from_json {
                                if r#summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("summary"));
                                }
                                r#summary =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4b::resources::CitationSummary,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#summary.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationSummary > ()) ?) ;
                            }
                        }
                        Field::Classification => {
                            if self.0.from_json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4b::resources::CitationClassification,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationClassification > ()) ?) ;
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::CurrentState => {
                            if self.0.from_json {
                                if r#current_state.is_some() {
                                    return Err(serde::de::Error::duplicate_field("currentState"));
                                }
                                r#current_state =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#current_state.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::StatusDate => {
                            if self.0.from_json {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4b::resources::CitationStatusDate,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#status_date.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationStatusDate > ()) ?) ;
                            }
                        }
                        Field::RelatesTo => {
                            if self.0.from_json {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4b::resources::CitationRelatesTo,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#relates_to.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationRelatesTo > ()) ?) ;
                            }
                        }
                        Field::CitedArtifact => {
                            if r#cited_artifact.is_some() {
                                return Err(serde::de::Error::duplicate_field("citedArtifact"));
                            }
                            r#cited_artifact = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CitationCitedArtifact > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::Citation {
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
                    r#name,
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
                    r#approval_date,
                    r#last_review_date,
                    r#effective_period,
                    r#author: r#author.unwrap_or(vec![]),
                    r#editor: r#editor.unwrap_or(vec![]),
                    r#reviewer: r#reviewer.unwrap_or(vec![]),
                    r#endorser: r#endorser.unwrap_or(vec![]),
                    r#summary: r#summary.unwrap_or(vec![]),
                    r#classification: r#classification.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#current_state: r#current_state.unwrap_or(vec![]),
                    r#status_date: r#status_date.unwrap_or(vec![]),
                    r#relates_to: r#relates_to.unwrap_or(vec![]),
                    r#cited_artifact,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::Citation>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::Citation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::Citation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::Citation>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::Citation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::Citation>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::Citation>;
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
                        .transmute::<fhirbolt_model::r4b::resources::Citation>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::Citation>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::Citation>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::Citation>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::Citation>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::Citation>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
