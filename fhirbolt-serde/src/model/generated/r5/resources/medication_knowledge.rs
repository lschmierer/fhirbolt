// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge,
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
                "MedicationKnowledge.relatedMedicationKnowledge", field
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
        if !self.value.r#reference.is_empty() {
            self.with_context(&self.value.r#reference, |ctx| {
                state.serialize_entry("reference", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRelatedMedicationKnowledge")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge,
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
                    #[serde(rename = "reference")]
                    Reference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "reference"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#reference: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Reference => {
                            if self.0.from_json {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#reference.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
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
                Ok(
                    fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#reference: r#reference.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeMonograph,
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
                "MedicationKnowledge.monograph", field
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
        if let Some(some) = self.value.r#source.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("source", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeMonograph,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMonograph;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeMonograph,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMonograph;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonograph")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph, V::Error>
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
                    #[serde(rename = "source")]
                    Source,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "source"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#source: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeMonograph {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#source,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>(),
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
        &fhirbolt_model::r5::resources::MedicationKnowledgeCost,
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
                "MedicationKnowledge.cost", field
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
        if !self.value.r#effective_date.is_empty() {
            self.with_context(&self.value.r#effective_date, |ctx| {
                state.serialize_entry("effectiveDate", ctx)
            })?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
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
        match self.value.r#cost {
            fhirbolt_model::r5::resources::MedicationKnowledgeCostCost::Money(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("costMoney", ctx))?;
            }
            fhirbolt_model::r5::resources::MedicationKnowledgeCostCost::CodeableConcept(
                ref value,
            ) => {
                self.with_context(value, |ctx| {
                    state.serialize_entry("costCodeableConcept", ctx)
                })?;
            }
            fhirbolt_model::r5::resources::MedicationKnowledgeCostCost::Invalid => {
                return Err(serde::ser::Error::custom("cost is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeCost,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeCost;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeCost,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeCost;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeCost")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeCost, V::Error>
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
                    #[serde(rename = "effectiveDate")]
                    EffectiveDate,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "_source")]
                    SourcePrimitiveElement,
                    #[serde(rename = "costMoney")]
                    CostMoney,
                    #[serde(rename = "costCodeableConcept")]
                    CostCodeableConcept,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "effectiveDate",
                            "type",
                            "source",
                            "costMoney",
                            "costCodeableConcept",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#effective_date: Option<Vec<fhirbolt_model::r5::types::Period>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#source: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#cost: Option<fhirbolt_model::r5::resources::MedicationKnowledgeCostCost> =
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
                        Field::EffectiveDate => {
                            if self.0.from_json {
                                if r#effective_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("effectiveDate"));
                                }
                                r#effective_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r5::types::Period>>(),
                                )?);
                            } else {
                                let vec = r#effective_date.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Period>(),
                                )?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                        Field::CostMoney => {
                            if r#cost.is_some() {
                                return Err(serde::de::Error::duplicate_field("costMoney"));
                            }
                            r#cost = Some(
                                fhirbolt_model::r5::resources::MedicationKnowledgeCostCost::Money(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r5::types::Money>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::CostCodeableConcept => {
                            if r#cost.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "costCodeableConcept",
                                ));
                            }
                            r#cost = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeCostCost :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::MedicationKnowledgeCost {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#effective_date: r#effective_date.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#source,
                    r#cost: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#cost.unwrap_or(Default::default())
                    } else {
                        r#cost.ok_or(serde::de::Error::missing_field("cost[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeCost>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeCost>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeCost>(),
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
        &fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram,
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
                "MedicationKnowledge.monitoringProgram", field
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
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonitoringProgram")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram, V::Error>
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "name"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#name,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMonitoringProgram > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage,
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
                "MedicationKnowledge.indicationGuideline.dosingGuideline.dosage", field
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
        if !self.value.r#dosage.is_empty() {
            self.with_context(&self.value.r#dosage, |ctx| {
                state.serialize_entry("dosage", ctx)
            })?;
        }
        state.end()
    }
}
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage,
    >
{
    type Value =
        fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage")
            }            fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage , V :: Error > where V : serde :: de :: MapAccess < 'de > ,{
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
                    #[serde(rename = "dosage")]
                    Dosage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "dosage"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#dosage: Option<Vec<fhirbolt_model::r5::types::Dosage>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Dosage => {
                            if self.0.from_json {
                                if r#dosage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dosage"));
                                }
                                r#dosage = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r5::types::Dosage>>(),
                                )?);
                            } else {
                                let vec = r#dosage.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Dosage>(),
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
                Ok (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#dosage : r#dosage . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >> { type Value = Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >> { type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; # [allow (dead_code)] fn missing_field_error < T , E : serde :: ser :: Error > (field : & str) -> Result < T , E > { Err (E :: custom (format ! ("missing required field `{}.{}`" , "MedicationKnowledge.indicationGuideline.dosingGuideline.patientCharacteristic" , field))) } let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } if self . value . r#type . id . as_deref () == Some ("$invalid") { return missing_field_error ("type") } self . with_context (& self . value . r#type , | ctx | state . serialize_entry ("type" , ctx)) ? ; if let Some (some) = self . value . r#value . as_ref () { match some { fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: Range (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueRange" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("value is invalid")) } } } state . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > { type Value = fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "type")] Type , # [serde (rename = "valueCodeableConcept")] ValueCodeableConcept , # [serde (rename = "valueQuantity")] ValueQuantity , # [serde (rename = "valueRange")] ValueRange , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "type" , "valueCodeableConcept" , "valueQuantity" , "valueRange" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#modifier_extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#type : Option < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > = None ; let mut r#value : Option < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ; } , Field :: ValueCodeableConcept => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueCodeableConcept")) ; } r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ; } , Field :: ValueQuantity => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueQuantity")) ; } r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ; } , Field :: ValueRange => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueRange")) ; } r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicValue :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Range > > ()) ?)) ; } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#value , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >> { type Value = Box < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >> { type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline,
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
                "MedicationKnowledge.indicationGuideline.dosingGuideline", field
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
        if let Some(some) = self.value.r#treatment_intent.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("treatmentIntent", ctx))?;
        }
        if !self.value.r#dosage.is_empty() {
            self.with_context(&self.value.r#dosage, |ctx| {
                state.serialize_entry("dosage", ctx)
            })?;
        }
        if let Some(some) = self.value.r#administration_treatment.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("administrationTreatment", ctx)
            })?;
        }
        if !self.value.r#patient_characteristic.is_empty() {
            self.with_context(&self.value.r#patient_characteristic, |ctx| {
                state.serialize_entry("patientCharacteristic", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline,
    >
{
    type Value =
        fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeIndicationGuidelineDosingGuideline")
            }            fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline , V :: Error > where V : serde :: de :: MapAccess < 'de > ,{
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "treatmentIntent")]
                    TreatmentIntent,
                    #[serde(rename = "dosage")]
                    Dosage,
                    #[serde(rename = "administrationTreatment")]
                    AdministrationTreatment,
                    #[serde(rename = "patientCharacteristic")]
                    PatientCharacteristic,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "treatmentIntent",
                            "dosage",
                            "administrationTreatment",
                            "patientCharacteristic",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#treatment_intent: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#dosage : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > > = None ;
                let mut r#administration_treatment: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#patient_characteristic : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > > = None ;
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
                        Field::TreatmentIntent => {
                            if r#treatment_intent.is_some() {
                                return Err(serde::de::Error::duplicate_field("treatmentIntent"));
                            }
                            r#treatment_intent = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Dosage => {
                            if self.0.from_json {
                                if r#dosage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dosage"));
                                }
                                r#dosage = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage >> ()) ?) ;
                            } else {
                                let vec = r#dosage.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelineDosage > ()) ?) ;
                            }
                        }
                        Field::AdministrationTreatment => {
                            if r#administration_treatment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrationTreatment",
                                ));
                            }
                            r#administration_treatment = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::PatientCharacteristic => {
                            if self.0.from_json {
                                if r#patient_characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientCharacteristic",
                                    ));
                                }
                                r#patient_characteristic = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic >> ()) ?) ;
                            } else {
                                let vec =
                                    r#patient_characteristic.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristic > ()) ?) ;
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
                Ok (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#treatment_intent , r#dosage : r#dosage . unwrap_or (vec ! []) , r#administration_treatment , r#patient_characteristic : r#patient_characteristic . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>,
    >
{
    type Value =
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>,
    >
{
    type Value =
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuidelineDosingGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline > ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline,
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
                "MedicationKnowledge.indicationGuideline", field
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
        if !self.value.r#indication.is_empty() {
            self.with_context(&self.value.r#indication, |ctx| {
                state.serialize_entry("indication", ctx)
            })?;
        }
        if !self.value.r#dosing_guideline.is_empty() {
            self.with_context(&self.value.r#dosing_guideline, |ctx| {
                state.serialize_entry("dosingGuideline", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeIndicationGuideline")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline,
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
                    #[serde(rename = "indication")]
                    Indication,
                    #[serde(rename = "dosingGuideline")]
                    DosingGuideline,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "indication",
                            "dosingGuideline",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#indication: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#dosing_guideline : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline > > = None ;
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
                        Field::Indication => {
                            if self.0.from_json {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                r#indication = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableReference >> ()) ?) ;
                            } else {
                                let vec = r#indication.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableReference > ()) ?) ;
                            }
                        }
                        Field::DosingGuideline => {
                            if self.0.from_json {
                                if r#dosing_guideline.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dosingGuideline",
                                    ));
                                }
                                r#dosing_guideline = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline >> ()) ?) ;
                            } else {
                                let vec = r#dosing_guideline.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuidelineDosingGuideline > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#indication: r#indication.unwrap_or(vec![]),
                        r#dosing_guideline: r#dosing_guideline.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuideline > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification,
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
                "MedicationKnowledge.medicineClassification", field
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
        if let Some(some) = self.value.r#source.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("sourceString" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_sourceString" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("sourceString" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("sourceUri" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_sourceUri" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("sourceUri" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Invalid => { return Err (serde :: ser :: Error :: custom ("source is invalid")) } }
        }
        if !self.value.r#classification.is_empty() {
            self.with_context(&self.value.r#classification, |ctx| {
                state.serialize_entry("classification", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMedicineClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification,
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
                    #[serde(rename = "sourceString")]
                    SourceString,
                    #[serde(rename = "_sourceString")]
                    SourceStringPrimitiveElement,
                    #[serde(rename = "sourceUri")]
                    SourceUri,
                    #[serde(rename = "_sourceUri")]
                    SourceUriPrimitiveElement,
                    #[serde(rename = "classification")]
                    Classification,
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
                            "sourceString",
                            "sourceUri",
                            "classification",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#source: Option<
                    fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassificationSource,
                > = None;
                let mut r#classification: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::SourceString => {
                            if self.0.from_json {
                                let r#enum = r#source . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("sourceString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("source[x]")) ; }
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceString"));
                                }
                                r#source = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::SourceStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#source . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_sourceString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_source[x]")) ; }
                            } else {
                                return unknown_field_error("sourceString");
                            }
                        }
                        Field::SourceUri => {
                            if self.0.from_json {
                                let r#enum = r#source . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("sourceUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("source[x]")) ; }
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sourceUri"));
                                }
                                r#source = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::SourceUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#source . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassificationSource :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_sourceUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_source[x]")) ; }
                            } else {
                                return unknown_field_error("sourceUri");
                            }
                        }
                        Field::Classification => {
                            if self.0.from_json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#source,
                        r#classification: r#classification.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassification > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgePackaging,
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
                "MedicationKnowledge.packaging", field
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
        if !self.value.r#cost.is_empty() {
            self.with_context(&self.value.r#cost, |ctx| state.serialize_entry("cost", ctx))?;
        }
        if let Some(some) = self.value.r#packaged_product.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("packagedProduct", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgePackaging,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgePackaging;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgePackaging,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgePackaging;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgePackaging")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgePackaging, V::Error>
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
                    #[serde(rename = "cost")]
                    Cost,
                    #[serde(rename = "packagedProduct")]
                    PackagedProduct,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "cost",
                            "packagedProduct",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#cost: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
                > = None;
                let mut r#packaged_product: Option<Box<fhirbolt_model::r5::types::Reference>> =
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
                        Field::Cost => {
                            if self.0.from_json {
                                if r#cost.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cost"));
                                }
                                r#cost =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r5::resources::MedicationKnowledgeCost,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#cost.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeCost > ()) ?) ;
                            }
                        }
                        Field::PackagedProduct => {
                            if r#packaged_product.is_some() {
                                return Err(serde::de::Error::duplicate_field("packagedProduct"));
                            }
                            r#packaged_product = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
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
                    fhirbolt_model::r5::resources::MedicationKnowledgePackaging {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#cost: r#cost.unwrap_or(vec![]),
                        r#packaged_product,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>(),
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
        &fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting,
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
                "MedicationKnowledge.storageGuideline.environmentalSetting", field
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
        match self . value . r#value { fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: Range (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueRange" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("value is a required field")) } }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<
            fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting,
        >,
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
        &Vec<
            fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting,
    >
{
    type Value =
        fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeStorageGuidelineEnvironmentalSetting")
            }            fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting , V :: Error > where V : serde :: de :: MapAccess < 'de > ,{
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
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
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
                            "valueQuantity",
                            "valueRange",
                            "valueCodeableConcept",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value : Option < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue > = None ;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Range > > ()) ?)) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSettingValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#value : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#value . unwrap_or (Default :: default ()) } else { r#value . ok_or (serde :: de :: Error :: missing_field ("value[x]")) ? } , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting>,
    >
{
    type Value =
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting>,
    >
{
    type Value =
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuidelineEnvironmentalSetting>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting > ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline,
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
                "MedicationKnowledge.storageGuideline", field
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
            if let Some(some) = self.value.r#reference.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("reference", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_reference", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#reference.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("reference", ctx))?;
            }
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if let Some(some) = self.value.r#stability_duration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("stabilityDuration", ctx))?;
        }
        if !self.value.r#environmental_setting.is_empty() {
            self.with_context(&self.value.r#environmental_setting, |ctx| {
                state.serialize_entry("environmentalSetting", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeStorageGuideline")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline, V::Error>
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
                    #[serde(rename = "reference")]
                    Reference,
                    #[serde(rename = "_reference")]
                    ReferencePrimitiveElement,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "stabilityDuration")]
                    StabilityDuration,
                    #[serde(rename = "environmentalSetting")]
                    EnvironmentalSetting,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "reference",
                            "note",
                            "stabilityDuration",
                            "environmentalSetting",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#reference: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#stability_duration: Option<Box<fhirbolt_model::r5::types::Duration>> =
                    None;
                let mut r#environmental_setting : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting > > = None ;
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
                        Field::Reference => {
                            if self.0.from_json {
                                let some = r#reference.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ReferencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#reference.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_reference"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("reference");
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
                        Field::StabilityDuration => {
                            if r#stability_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("stabilityDuration"));
                            }
                            r#stability_duration = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Duration>>(),
                                )?,
                            );
                        }
                        Field::EnvironmentalSetting => {
                            if self.0.from_json {
                                if r#environmental_setting.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "environmentalSetting",
                                    ));
                                }
                                r#environmental_setting = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting >> ()) ?) ;
                            } else {
                                let vec = r#environmental_setting.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuidelineEnvironmentalSetting > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#reference,
                        r#note: r#note.unwrap_or(vec![]),
                        r#stability_duration,
                        r#environmental_setting: r#environmental_setting.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuideline > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution,
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
                "MedicationKnowledge.regulatory.substitution", field
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
        if self.output_json {
            if self.value.r#allowed.id.as_deref() == Some("$invalid") {
                return missing_field_error("allowed");
            }
            if let Some(some) = self.value.r#allowed.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("allowed", &some)?;
            }
            if self.value.r#allowed.id.is_some() || !self.value.r#allowed.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#allowed.id.as_ref(),
                    extension: &self.value.r#allowed.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_allowed", ctx)
                })?;
            }
        } else {
            if self.value.r#allowed.id.as_deref() == Some("$invalid") {
                return missing_field_error("allowed");
            }
            self.with_context(&self.value.r#allowed, |ctx| {
                state.serialize_entry("allowed", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatorySubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution,
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
                    #[serde(rename = "allowed")]
                    Allowed,
                    #[serde(rename = "_allowed")]
                    AllowedPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "allowed"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#allowed: Option<fhirbolt_model::r5::types::Boolean> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Allowed => {
                            if self.0.from_json {
                                let some = r#allowed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowed"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowed"));
                                }
                                r#allowed = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::AllowedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#allowed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_allowed"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("allowed");
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#allowed: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#allowed.unwrap_or(Default::default())
                        } else {
                            r#allowed.ok_or(serde::de::Error::missing_field("allowed"))?
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
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatorySubstitution > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense,
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
                "MedicationKnowledge.regulatory.maxDispense", field
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
        if self.value.r#quantity.id.as_deref() == Some("$invalid") {
            return missing_field_error("quantity");
        }
        self.with_context(&self.value.r#quantity, |ctx| {
            state.serialize_entry("quantity", ctx)
        })?;
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatoryMaxDispense")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense,
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
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "quantity", "period"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r5::types::Duration>> = None;
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Duration>>(),
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#quantity: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#quantity.unwrap_or(Default::default())
                        } else {
                            r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?
                        },
                        r#period,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatoryMaxDispense > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory,
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
                "MedicationKnowledge.regulatory", field
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
        if self.value.r#regulatory_authority.id.as_deref() == Some("$invalid") {
            return missing_field_error("regulatoryAuthority");
        }
        self.with_context(&self.value.r#regulatory_authority, |ctx| {
            state.serialize_entry("regulatoryAuthority", ctx)
        })?;
        if !self.value.r#substitution.is_empty() {
            self.with_context(&self.value.r#substitution, |ctx| {
                state.serialize_entry("substitution", ctx)
            })?;
        }
        if !self.value.r#schedule.is_empty() {
            self.with_context(&self.value.r#schedule, |ctx| {
                state.serialize_entry("schedule", ctx)
            })?;
        }
        if let Some(some) = self.value.r#max_dispense.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxDispense", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatory")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory, V::Error>
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
                    #[serde(rename = "regulatoryAuthority")]
                    RegulatoryAuthority,
                    #[serde(rename = "substitution")]
                    Substitution,
                    #[serde(rename = "schedule")]
                    Schedule,
                    #[serde(rename = "maxDispense")]
                    MaxDispense,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "regulatoryAuthority",
                            "substitution",
                            "schedule",
                            "maxDispense",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#regulatory_authority: Option<Box<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#substitution: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatorySubstitution>,
                > = None;
                let mut r#schedule: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#max_dispense: Option<
                    fhirbolt_model::r5::resources::MedicationKnowledgeRegulatoryMaxDispense,
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
                        Field::RegulatoryAuthority => {
                            if r#regulatory_authority.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "regulatoryAuthority",
                                ));
                            }
                            r#regulatory_authority = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Substitution => {
                            if self.0.from_json {
                                if r#substitution.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substitution"));
                                }
                                r#substitution = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatorySubstitution >> ()) ?) ;
                            } else {
                                let vec = r#substitution.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatorySubstitution > ()) ?) ;
                            }
                        }
                        Field::Schedule => {
                            if self.0.from_json {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#schedule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::MaxDispense => {
                            if r#max_dispense.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDispense"));
                            }
                            r#max_dispense = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatoryMaxDispense > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#regulatory_authority: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#regulatory_authority.unwrap_or(Default::default())
                        } else {
                            r#regulatory_authority
                                .ok_or(serde::de::Error::missing_field("regulatoryAuthority"))?
                        },
                        r#substitution: r#substitution.unwrap_or(vec![]),
                        r#schedule: r#schedule.unwrap_or(vec![]),
                        r#max_dispense,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>(
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
        &fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient,
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
                "MedicationKnowledge.definitional.ingredient", field
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
        if self.value.r#item.id.as_deref() == Some("$invalid") {
            return missing_field_error("item");
        }
        self.with_context(&self.value.r#item, |ctx| state.serialize_entry("item", ctx))?;
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#strength.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: Ratio (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("strengthRatio" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("strengthCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("strengthQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: Invalid => { return Err (serde :: ser :: Error :: custom ("strength is invalid")) } }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeDefinitionalIngredient")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient,
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
                    #[serde(rename = "item")]
                    Item,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "strengthRatio")]
                    StrengthRatio,
                    #[serde(rename = "strengthCodeableConcept")]
                    StrengthCodeableConcept,
                    #[serde(rename = "strengthQuantity")]
                    StrengthQuantity,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "item",
                            "type",
                            "strengthRatio",
                            "strengthCodeableConcept",
                            "strengthQuantity",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#item: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#strength : Option < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength > = None ;
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
                        Field::Item => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            r#item = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::StrengthRatio => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthRatio"));
                            }
                            r#strength = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: Ratio (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Ratio > > ()) ?)) ;
                        }
                        Field::StrengthCodeableConcept => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "strengthCodeableConcept",
                                ));
                            }
                            r#strength = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::StrengthQuantity => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthQuantity"));
                            }
                            r#strength = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredientStrength :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#item: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#item.unwrap_or(Default::default())
                        } else {
                            r#item.ok_or(serde::de::Error::missing_field("item"))?
                        },
                        r#type,
                        r#strength,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredient > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic,
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
                "MedicationKnowledge.definitional.drugCharacteristic", field
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
        if let Some(some) = self.value.r#value.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueString" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueString" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueString" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueBase64Binary" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueBase64Binary" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueBase64Binary" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Attachment (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueAttachment" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("value is invalid")) } }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeDefinitionalDrugCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic,
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
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueBase64Binary")]
                    ValueBase64Binary,
                    #[serde(rename = "_valueBase64Binary")]
                    ValueBase64BinaryPrimitiveElement,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
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
                            "valueCodeableConcept",
                            "valueString",
                            "valueQuantity",
                            "valueBase64Binary",
                            "valueAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value : Option < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue > = None ;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristicValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok (fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type , r#value , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>,
    >
{
    type Value =
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>,
    >
{
    type Value =
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalDrugCharacteristic,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional,
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
                "MedicationKnowledge.definitional", field
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
        if !self.value.r#definition.is_empty() {
            self.with_context(&self.value.r#definition, |ctx| {
                state.serialize_entry("definition", ctx)
            })?;
        }
        if let Some(some) = self.value.r#dose_form.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("doseForm", ctx))?;
        }
        if !self.value.r#intended_route.is_empty() {
            self.with_context(&self.value.r#intended_route, |ctx| {
                state.serialize_entry("intendedRoute", ctx)
            })?;
        }
        if !self.value.r#ingredient.is_empty() {
            self.with_context(&self.value.r#ingredient, |ctx| {
                state.serialize_entry("ingredient", ctx)
            })?;
        }
        if !self.value.r#drug_characteristic.is_empty() {
            self.with_context(&self.value.r#drug_characteristic, |ctx| {
                state.serialize_entry("drugCharacteristic", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>,
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
        fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeDefinitional")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional, V::Error>
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
                    #[serde(rename = "definition")]
                    Definition,
                    #[serde(rename = "doseForm")]
                    DoseForm,
                    #[serde(rename = "intendedRoute")]
                    IntendedRoute,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "drugCharacteristic")]
                    DrugCharacteristic,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "definition",
                            "doseForm",
                            "intendedRoute",
                            "ingredient",
                            "drugCharacteristic",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#definition: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#dose_form: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#intended_route: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#ingredient: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitionalIngredient>,
                > = None;
                let mut r#drug_characteristic : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic > > = None ;
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
                        Field::Definition => {
                            if self.0.from_json {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                r#definition = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#definition.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::DoseForm => {
                            if r#dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseForm"));
                            }
                            r#dose_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::IntendedRoute => {
                            if self.0.from_json {
                                if r#intended_route.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intendedRoute"));
                                }
                                r#intended_route = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#intended_route.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Ingredient => {
                            if self.0.from_json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredient >> ()) ?) ;
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalIngredient > ()) ?) ;
                            }
                        }
                        Field::DrugCharacteristic => {
                            if self.0.from_json {
                                if r#drug_characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "drugCharacteristic",
                                    ));
                                }
                                r#drug_characteristic = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic >> ()) ?) ;
                            } else {
                                let vec = r#drug_characteristic.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitionalDrugCharacteristic > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#definition: r#definition.unwrap_or(vec![]),
                        r#dose_form,
                        r#intended_route: r#intended_route.unwrap_or(vec![]),
                        r#ingredient: r#ingredient.unwrap_or(vec![]),
                        r#drug_characteristic: r#drug_characteristic.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitional > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::MedicationKnowledge {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationKnowledge,
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
                "MedicationKnowledge", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationKnowledge")?;
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
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
            }
        }
        if let Some(some) = self.value.r#author.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("author", ctx))?;
        }
        if !self.value.r#intended_jurisdiction.is_empty() {
            self.with_context(&self.value.r#intended_jurisdiction, |ctx| {
                state.serialize_entry("intendedJurisdiction", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#name.is_empty() {
                let values = self
                    .value
                    .r#name
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("name", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#name
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#name
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
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#name.is_empty() {
                self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
            }
        }
        if !self.value.r#related_medication_knowledge.is_empty() {
            self.with_context(&self.value.r#related_medication_knowledge, |ctx| {
                state.serialize_entry("relatedMedicationKnowledge", ctx)
            })?;
        }
        if !self.value.r#associated_medication.is_empty() {
            self.with_context(&self.value.r#associated_medication, |ctx| {
                state.serialize_entry("associatedMedication", ctx)
            })?;
        }
        if !self.value.r#product_type.is_empty() {
            self.with_context(&self.value.r#product_type, |ctx| {
                state.serialize_entry("productType", ctx)
            })?;
        }
        if !self.value.r#monograph.is_empty() {
            self.with_context(&self.value.r#monograph, |ctx| {
                state.serialize_entry("monograph", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#preparation_instruction.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("preparationInstruction", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_preparationInstruction", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#preparation_instruction.as_ref() {
                self.with_context(some, |ctx| {
                    state.serialize_entry("preparationInstruction", ctx)
                })?;
            }
        }
        if !self.value.r#cost.is_empty() {
            self.with_context(&self.value.r#cost, |ctx| state.serialize_entry("cost", ctx))?;
        }
        if !self.value.r#monitoring_program.is_empty() {
            self.with_context(&self.value.r#monitoring_program, |ctx| {
                state.serialize_entry("monitoringProgram", ctx)
            })?;
        }
        if !self.value.r#indication_guideline.is_empty() {
            self.with_context(&self.value.r#indication_guideline, |ctx| {
                state.serialize_entry("indicationGuideline", ctx)
            })?;
        }
        if !self.value.r#medicine_classification.is_empty() {
            self.with_context(&self.value.r#medicine_classification, |ctx| {
                state.serialize_entry("medicineClassification", ctx)
            })?;
        }
        if !self.value.r#packaging.is_empty() {
            self.with_context(&self.value.r#packaging, |ctx| {
                state.serialize_entry("packaging", ctx)
            })?;
        }
        if !self.value.r#clinical_use_issue.is_empty() {
            self.with_context(&self.value.r#clinical_use_issue, |ctx| {
                state.serialize_entry("clinicalUseIssue", ctx)
            })?;
        }
        if !self.value.r#storage_guideline.is_empty() {
            self.with_context(&self.value.r#storage_guideline, |ctx| {
                state.serialize_entry("storageGuideline", ctx)
            })?;
        }
        if !self.value.r#regulatory.is_empty() {
            self.with_context(&self.value.r#regulatory, |ctx| {
                state.serialize_entry("regulatory", ctx)
            })?;
        }
        if let Some(some) = self.value.r#definitional.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("definitional", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationKnowledge>,
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
        &Vec<fhirbolt_model::r5::resources::MedicationKnowledge>,
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
        fhirbolt_model::r5::resources::MedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledge;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationKnowledge;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationKnowledge,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledge")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationKnowledge, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "intendedJurisdiction")]
                    IntendedJurisdiction,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "relatedMedicationKnowledge")]
                    RelatedMedicationKnowledge,
                    #[serde(rename = "associatedMedication")]
                    AssociatedMedication,
                    #[serde(rename = "productType")]
                    ProductType,
                    #[serde(rename = "monograph")]
                    Monograph,
                    #[serde(rename = "preparationInstruction")]
                    PreparationInstruction,
                    #[serde(rename = "_preparationInstruction")]
                    PreparationInstructionPrimitiveElement,
                    #[serde(rename = "cost")]
                    Cost,
                    #[serde(rename = "monitoringProgram")]
                    MonitoringProgram,
                    #[serde(rename = "indicationGuideline")]
                    IndicationGuideline,
                    #[serde(rename = "medicineClassification")]
                    MedicineClassification,
                    #[serde(rename = "packaging")]
                    Packaging,
                    #[serde(rename = "clinicalUseIssue")]
                    ClinicalUseIssue,
                    #[serde(rename = "storageGuideline")]
                    StorageGuideline,
                    #[serde(rename = "regulatory")]
                    Regulatory,
                    #[serde(rename = "definitional")]
                    Definitional,
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
                            "code",
                            "status",
                            "author",
                            "intendedJurisdiction",
                            "name",
                            "relatedMedicationKnowledge",
                            "associatedMedication",
                            "productType",
                            "monograph",
                            "preparationInstruction",
                            "cost",
                            "monitoringProgram",
                            "indicationGuideline",
                            "medicineClassification",
                            "packaging",
                            "clinicalUseIssue",
                            "storageGuideline",
                            "regulatory",
                            "definitional",
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
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#author: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#intended_jurisdiction: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#name: Option<Vec<fhirbolt_model::r5::types::String>> = None;
                let mut r#related_medication_knowledge : Option < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > > = None ;
                let mut r#associated_medication: Option<Vec<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#product_type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#monograph: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonograph>,
                > = None;
                let mut r#preparation_instruction: Option<fhirbolt_model::r5::types::Markdown> =
                    None;
                let mut r#cost: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeCost>,
                > = None;
                let mut r#monitoring_program: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMonitoringProgram>,
                > = None;
                let mut r#indication_guideline: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeIndicationGuideline>,
                > = None;
                let mut r#medicine_classification: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeMedicineClassification>,
                > = None;
                let mut r#packaging: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgePackaging>,
                > = None;
                let mut r#clinical_use_issue: Option<Vec<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#storage_guideline: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeStorageGuideline>,
                > = None;
                let mut r#regulatory: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationKnowledgeRegulatory>,
                > = None;
                let mut r#definitional: Option<
                    fhirbolt_model::r5::resources::MedicationKnowledgeDefinitional,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationKnowledge" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationKnowledge",
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
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
                        Field::IntendedJurisdiction => {
                            if self.0.from_json {
                                if r#intended_jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "intendedJurisdiction",
                                    ));
                                }
                                r#intended_jurisdiction = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#intended_jurisdiction.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#name.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#name.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#name.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::RelatedMedicationKnowledge => {
                            if self.0.from_json {
                                if r#related_medication_knowledge.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedMedicationKnowledge",
                                    ));
                                }
                                r#related_medication_knowledge = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRelatedMedicationKnowledge >> ()) ?) ;
                            } else {
                                let vec = r#related_medication_knowledge
                                    .get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > ()) ?) ;
                            }
                        }
                        Field::AssociatedMedication => {
                            if self.0.from_json {
                                if r#associated_medication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "associatedMedication",
                                    ));
                                }
                                r#associated_medication = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#associated_medication.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::ProductType => {
                            if self.0.from_json {
                                if r#product_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productType"));
                                }
                                r#product_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#product_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Monograph => {
                            if self.0.from_json {
                                if r#monograph.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monograph"));
                                }
                                r#monograph =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r5::resources::MedicationKnowledgeMonograph,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#monograph.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMonograph > ()) ?) ;
                            }
                        }
                        Field::PreparationInstruction => {
                            if self.0.from_json {
                                let some =
                                    r#preparation_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preparationInstruction",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#preparation_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preparationInstruction",
                                    ));
                                }
                                r#preparation_instruction = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::PreparationInstructionPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#preparation_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_preparationInstruction",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preparationInstruction");
                            }
                        }
                        Field::Cost => {
                            if self.0.from_json {
                                if r#cost.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cost"));
                                }
                                r#cost =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r5::resources::MedicationKnowledgeCost,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#cost.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeCost > ()) ?) ;
                            }
                        }
                        Field::MonitoringProgram => {
                            if self.0.from_json {
                                if r#monitoring_program.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "monitoringProgram",
                                    ));
                                }
                                r#monitoring_program = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMonitoringProgram >> ()) ?) ;
                            } else {
                                let vec = r#monitoring_program.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMonitoringProgram > ()) ?) ;
                            }
                        }
                        Field::IndicationGuideline => {
                            if self.0.from_json {
                                if r#indication_guideline.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "indicationGuideline",
                                    ));
                                }
                                r#indication_guideline = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuideline >> ()) ?) ;
                            } else {
                                let vec = r#indication_guideline.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeIndicationGuideline > ()) ?) ;
                            }
                        }
                        Field::MedicineClassification => {
                            if self.0.from_json {
                                if r#medicine_classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "medicineClassification",
                                    ));
                                }
                                r#medicine_classification = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassification >> ()) ?) ;
                            } else {
                                let vec =
                                    r#medicine_classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeMedicineClassification > ()) ?) ;
                            }
                        }
                        Field::Packaging => {
                            if self.0.from_json {
                                if r#packaging.is_some() {
                                    return Err(serde::de::Error::duplicate_field("packaging"));
                                }
                                r#packaging =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r5::resources::MedicationKnowledgePackaging,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#packaging.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgePackaging > ()) ?) ;
                            }
                        }
                        Field::ClinicalUseIssue => {
                            if self.0.from_json {
                                if r#clinical_use_issue.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "clinicalUseIssue",
                                    ));
                                }
                                r#clinical_use_issue = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#clinical_use_issue.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::StorageGuideline => {
                            if self.0.from_json {
                                if r#storage_guideline.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "storageGuideline",
                                    ));
                                }
                                r#storage_guideline = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuideline >> ()) ?) ;
                            } else {
                                let vec = r#storage_guideline.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeStorageGuideline > ()) ?) ;
                            }
                        }
                        Field::Regulatory => {
                            if self.0.from_json {
                                if r#regulatory.is_some() {
                                    return Err(serde::de::Error::duplicate_field("regulatory"));
                                }
                                r#regulatory = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatory >> ()) ?) ;
                            } else {
                                let vec = r#regulatory.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeRegulatory > ()) ?) ;
                            }
                        }
                        Field::Definitional => {
                            if r#definitional.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitional"));
                            }
                            r#definitional = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationKnowledgeDefinitional > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::MedicationKnowledge {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#code,
                    r#status,
                    r#author,
                    r#intended_jurisdiction: r#intended_jurisdiction.unwrap_or(vec![]),
                    r#name: r#name.unwrap_or(vec![]),
                    r#related_medication_knowledge: r#related_medication_knowledge
                        .unwrap_or(vec![]),
                    r#associated_medication: r#associated_medication.unwrap_or(vec![]),
                    r#product_type: r#product_type.unwrap_or(vec![]),
                    r#monograph: r#monograph.unwrap_or(vec![]),
                    r#preparation_instruction,
                    r#cost: r#cost.unwrap_or(vec![]),
                    r#monitoring_program: r#monitoring_program.unwrap_or(vec![]),
                    r#indication_guideline: r#indication_guideline.unwrap_or(vec![]),
                    r#medicine_classification: r#medicine_classification.unwrap_or(vec![]),
                    r#packaging: r#packaging.unwrap_or(vec![]),
                    r#clinical_use_issue: r#clinical_use_issue.unwrap_or(vec![]),
                    r#storage_guideline: r#storage_guideline.unwrap_or(vec![]),
                    r#regulatory: r#regulatory.unwrap_or(vec![]),
                    r#definitional,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationKnowledge>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationKnowledge>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationKnowledge>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationKnowledge>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationKnowledge>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationKnowledge>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
