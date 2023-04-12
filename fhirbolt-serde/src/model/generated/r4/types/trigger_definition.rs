// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::types::TriggerDefinition,
    >
{
    type Value = fhirbolt_model::r4::types::TriggerDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::types::TriggerDefinition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::types::TriggerDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TriggerDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::types::TriggerDefinition, V::Error>
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "timingTiming")]
                    TimingTiming,
                    #[serde(rename = "timingReference")]
                    TimingReference,
                    #[serde(rename = "timingDate")]
                    TimingDate,
                    #[serde(rename = "_timingDate")]
                    TimingDatePrimitiveElement,
                    #[serde(rename = "timingDateTime")]
                    TimingDateTime,
                    #[serde(rename = "_timingDateTime")]
                    TimingDateTimePrimitiveElement,
                    #[serde(rename = "data")]
                    Data,
                    #[serde(rename = "condition")]
                    Condition,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "type",
                            "name",
                            "timingTiming",
                            "timingReference",
                            "timingDate",
                            "timingDateTime",
                            "data",
                            "condition",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#timing: Option<fhirbolt_model::r4::types::TriggerDefinitionTiming> = None;
                let mut r#data: Option<Vec<Box<fhirbolt_model::r4::types::DataRequirement>>> = None;
                let mut r#condition: Option<Box<fhirbolt_model::r4::types::Expression>> = None;
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
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
                        Field::TimingTiming => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingTiming"));
                            }
                            r#timing =
                                Some(fhirbolt_model::r4::types::TriggerDefinitionTiming::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Timing>>(),
                                    )?,
                                ));
                        }
                        Field::TimingReference => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingReference"));
                            }
                            r#timing = Some(
                                fhirbolt_model::r4::types::TriggerDefinitionTiming::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ),
                            );
                        }
                        Field::TimingDate => {
                            if self.0.from_json {
                                let r#enum = r#timing.get_or_insert(
                                    fhirbolt_model::r4::types::TriggerDefinitionTiming::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::types::TriggerDefinitionTiming::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            } else {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingDate"));
                                }
                                r#timing =
                                    Some(fhirbolt_model::r4::types::TriggerDefinitionTiming::Date(
                                        map_access.next_value_seed(
                                            self.0
                                                .transmute::<Box<fhirbolt_model::r4::types::Date>>(
                                                ),
                                        )?,
                                    ));
                            }
                        }
                        Field::TimingDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#timing.get_or_insert(
                                    fhirbolt_model::r4::types::TriggerDefinitionTiming::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::types::TriggerDefinitionTiming::Date(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timingDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_timing[x]"));
                                }
                            } else {
                                return unknown_field_error("timingDate");
                            }
                        }
                        Field::TimingDateTime => {
                            if self.0.from_json {
                                let r#enum = r#timing.get_or_insert(
                                    fhirbolt_model::r4::types::TriggerDefinitionTiming::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4 :: types :: TriggerDefinitionTiming :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("timingDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("timing[x]")) ; }
                            } else {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDateTime",
                                    ));
                                }
                                r#timing = Some (fhirbolt_model :: r4 :: types :: TriggerDefinitionTiming :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::TimingDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#timing.get_or_insert(
                                    fhirbolt_model::r4::types::TriggerDefinitionTiming::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4 :: types :: TriggerDefinitionTiming :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_timingDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_timing[x]")) ; }
                            } else {
                                return unknown_field_error("timingDateTime");
                            }
                        }
                        Field::Data => {
                            if self.0.from_json {
                                if r#data.is_some() {
                                    return Err(serde::de::Error::duplicate_field("data"));
                                }
                                r#data =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::DataRequirement>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#data.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DataRequirement > > ()) ?) ;
                            }
                        }
                        Field::Condition => {
                            if r#condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            r#condition = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Expression>>(),
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
                Ok(fhirbolt_model::r4::types::TriggerDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#name,
                    r#timing,
                    r#data: r#data.unwrap_or(vec![]),
                    r#condition,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::types::TriggerDefinition>,
    >
{
    type Value = Box<fhirbolt_model::r4::types::TriggerDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::types::TriggerDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::types::TriggerDefinition>,
    >
{
    type Value = Vec<fhirbolt_model::r4::types::TriggerDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::types::TriggerDefinition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::types::TriggerDefinition>;
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
                        .transmute::<fhirbolt_model::r4::types::TriggerDefinition>(),
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
        Vec<Box<fhirbolt_model::r4::types::TriggerDefinition>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::types::TriggerDefinition>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::types::TriggerDefinition>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::types::TriggerDefinition>>;
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
                        .transmute::<Box<fhirbolt_model::r4::types::TriggerDefinition>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
