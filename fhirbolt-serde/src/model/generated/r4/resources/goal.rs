// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::GoalTarget>
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
        if let Some(some) = self.value.r#measure.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("measure", ctx))?;
        }
        if let Some(some) = self.value.r#detail.as_ref() {
            match some {
                fhirbolt_model::r4::resources::GoalTargetDetail::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("detailQuantity", ctx))?;
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("detailRange", ctx))?;
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("detailCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("detailString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_detailString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("detailString", ctx))?;
                    }
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("detailBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_detailBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("detailBoolean", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("detailInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_detailInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("detailInteger", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("detailRatio", ctx))?;
                }
                fhirbolt_model::r4::resources::GoalTargetDetail::Invalid => {
                    return Err(serde::ser::Error::custom("detail is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#due.as_ref() {
            match some {
                fhirbolt_model::r4::resources::GoalTargetDue::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("dueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_dueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("dueDate", ctx))?;
                    }
                }
                fhirbolt_model::r4::resources::GoalTargetDue::Duration(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("dueDuration", ctx))?;
                }
                fhirbolt_model::r4::resources::GoalTargetDue::Invalid => {
                    return Err(serde::ser::Error::custom("due is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::GoalTarget>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::GoalTarget>>
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
        &Vec<Box<fhirbolt_model::r4::resources::GoalTarget>>,
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::GoalTarget>
{
    type Value = fhirbolt_model::r4::resources::GoalTarget;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::GoalTarget,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::GoalTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GoalTarget")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::GoalTarget, V::Error>
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
                    #[serde(rename = "measure")]
                    Measure,
                    #[serde(rename = "detailQuantity")]
                    DetailQuantity,
                    #[serde(rename = "detailRange")]
                    DetailRange,
                    #[serde(rename = "detailCodeableConcept")]
                    DetailCodeableConcept,
                    #[serde(rename = "detailString")]
                    DetailString,
                    #[serde(rename = "_detailString")]
                    DetailStringPrimitiveElement,
                    #[serde(rename = "detailBoolean")]
                    DetailBoolean,
                    #[serde(rename = "_detailBoolean")]
                    DetailBooleanPrimitiveElement,
                    #[serde(rename = "detailInteger")]
                    DetailInteger,
                    #[serde(rename = "_detailInteger")]
                    DetailIntegerPrimitiveElement,
                    #[serde(rename = "detailRatio")]
                    DetailRatio,
                    #[serde(rename = "dueDate")]
                    DueDate,
                    #[serde(rename = "_dueDate")]
                    DueDatePrimitiveElement,
                    #[serde(rename = "dueDuration")]
                    DueDuration,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "measure",
                            "detailQuantity",
                            "detailRange",
                            "detailCodeableConcept",
                            "detailString",
                            "detailBoolean",
                            "detailInteger",
                            "detailRatio",
                            "dueDate",
                            "dueDuration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#measure: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#detail: Option<fhirbolt_model::r4::resources::GoalTargetDetail> = None;
                let mut r#due: Option<fhirbolt_model::r4::resources::GoalTargetDue> = None;
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
                        Field::Measure => {
                            if r#measure.is_some() {
                                return Err(serde::de::Error::duplicate_field("measure"));
                            }
                            r#measure = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::DetailQuantity => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailQuantity"));
                            }
                            r#detail =
                                Some(fhirbolt_model::r4::resources::GoalTargetDetail::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::DetailRange => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRange"));
                            }
                            r#detail =
                                Some(fhirbolt_model::r4::resources::GoalTargetDetail::Range(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::DetailCodeableConcept => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "detailCodeableConcept",
                                ));
                            }
                            r#detail = Some (fhirbolt_model :: r4 :: resources :: GoalTargetDetail :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::DetailString => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailString"));
                                }
                                r#detail = Some (fhirbolt_model :: r4 :: resources :: GoalTargetDetail :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::DetailStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::String(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_detail[x]"));
                                }
                            } else {
                                return unknown_field_error("detailString");
                            }
                        }
                        Field::DetailBoolean => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailBoolean"));
                                }
                                r#detail = Some (fhirbolt_model :: r4 :: resources :: GoalTargetDetail :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::DetailBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_detail[x]"));
                                }
                            } else {
                                return unknown_field_error("detailBoolean");
                            }
                        }
                        Field::DetailInteger => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailInteger"));
                                }
                                r#detail = Some (fhirbolt_model :: r4 :: resources :: GoalTargetDetail :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::DetailIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#detail.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDetail::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDetail::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_detail[x]"));
                                }
                            } else {
                                return unknown_field_error("detailInteger");
                            }
                        }
                        Field::DetailRatio => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRatio"));
                            }
                            r#detail =
                                Some(fhirbolt_model::r4::resources::GoalTargetDetail::Ratio(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
                                    )?,
                                ));
                        }
                        Field::DueDate => {
                            if self.0.from_json {
                                let r#enum = r#due.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDue::Date(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("dueDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("due[x]"));
                                }
                            } else {
                                if r#due.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dueDate"));
                                }
                                r#due = Some(fhirbolt_model::r4::resources::GoalTargetDue::Date(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4::types::Date>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::DueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#due.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalTargetDue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalTargetDue::Date(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_dueDate"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_due[x]"));
                                }
                            } else {
                                return unknown_field_error("dueDate");
                            }
                        }
                        Field::DueDuration => {
                            if r#due.is_some() {
                                return Err(serde::de::Error::duplicate_field("dueDuration"));
                            }
                            r#due = Some(fhirbolt_model::r4::resources::GoalTargetDue::Duration(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Duration>>(),
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
                Ok(fhirbolt_model::r4::resources::GoalTarget {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#measure,
                    r#detail,
                    r#due,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::GoalTarget>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::GoalTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::GoalTarget>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::GoalTarget>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::GoalTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::GoalTarget>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::GoalTarget>;
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
                        .transmute::<fhirbolt_model::r4::resources::GoalTarget>(),
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
        Vec<Box<fhirbolt_model::r4::resources::GoalTarget>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::GoalTarget>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::GoalTarget>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::GoalTarget>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::GoalTarget>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::Goal {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::Goal>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Goal")?;
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
            if let Some(some) = self.value.r#lifecycle_status.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("lifecycleStatus", &some)?;
            }
            if self.value.r#lifecycle_status.id.is_some()
                || !self.value.r#lifecycle_status.extension.is_empty()
            {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#lifecycle_status.id.as_ref(),
                    extension: &self.value.r#lifecycle_status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_lifecycleStatus", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#lifecycle_status, |ctx| {
                state.serialize_entry("lifecycleStatus", ctx)
            })?;
        }
        if let Some(some) = self.value.r#achievement_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("achievementStatus", ctx))?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if let Some(some) = self.value.r#priority.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
        }
        self.with_context(&self.value.r#description, |ctx| {
            state.serialize_entry("description", ctx)
        })?;
        self.with_context(&self.value.r#subject, |ctx| {
            state.serialize_entry("subject", ctx)
        })?;
        if let Some(some) = self.value.r#start.as_ref() {
            match some {
                fhirbolt_model::r4::resources::GoalStart::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("startDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_startDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("startDate", ctx))?;
                    }
                }
                fhirbolt_model::r4::resources::GoalStart::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("startCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::GoalStart::Invalid => {
                    return Err(serde::ser::Error::custom("start is invalid"))
                }
            }
        }
        if !self.value.r#target.is_empty() {
            self.with_context(&self.value.r#target, |ctx| {
                state.serialize_entry("target", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("statusDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("statusDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#status_reason.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("statusReason", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusReason", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status_reason.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("statusReason", ctx))?;
            }
        }
        if let Some(some) = self.value.r#expressed_by.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expressedBy", ctx))?;
        }
        if !self.value.r#addresses.is_empty() {
            self.with_context(&self.value.r#addresses, |ctx| {
                state.serialize_entry("addresses", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#outcome_code.is_empty() {
            self.with_context(&self.value.r#outcome_code, |ctx| {
                state.serialize_entry("outcomeCode", ctx)
            })?;
        }
        if !self.value.r#outcome_reference.is_empty() {
            self.with_context(&self.value.r#outcome_reference, |ctx| {
                state.serialize_entry("outcomeReference", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::Goal>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::Goal>>
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
    for crate::context::ser::SerializationContext<&Vec<Box<fhirbolt_model::r4::resources::Goal>>>
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Goal>
{
    type Value = fhirbolt_model::r4::resources::Goal;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Goal>
{
    type Value = fhirbolt_model::r4::resources::Goal;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Goal>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::Goal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Goal")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::Goal, V::Error>
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
                    #[serde(rename = "lifecycleStatus")]
                    LifecycleStatus,
                    #[serde(rename = "_lifecycleStatus")]
                    LifecycleStatusPrimitiveElement,
                    #[serde(rename = "achievementStatus")]
                    AchievementStatus,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "startDate")]
                    StartDate,
                    #[serde(rename = "_startDate")]
                    StartDatePrimitiveElement,
                    #[serde(rename = "startCodeableConcept")]
                    StartCodeableConcept,
                    #[serde(rename = "target")]
                    Target,
                    #[serde(rename = "statusDate")]
                    StatusDate,
                    #[serde(rename = "_statusDate")]
                    StatusDatePrimitiveElement,
                    #[serde(rename = "statusReason")]
                    StatusReason,
                    #[serde(rename = "_statusReason")]
                    StatusReasonPrimitiveElement,
                    #[serde(rename = "expressedBy")]
                    ExpressedBy,
                    #[serde(rename = "addresses")]
                    Addresses,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "outcomeCode")]
                    OutcomeCode,
                    #[serde(rename = "outcomeReference")]
                    OutcomeReference,
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
                            "lifecycleStatus",
                            "achievementStatus",
                            "category",
                            "priority",
                            "description",
                            "subject",
                            "startDate",
                            "startCodeableConcept",
                            "target",
                            "statusDate",
                            "statusReason",
                            "expressedBy",
                            "addresses",
                            "note",
                            "outcomeCode",
                            "outcomeReference",
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
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#lifecycle_status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#achievement_status: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#category: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> =
                    None;
                let mut r#priority: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#description: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#start: Option<fhirbolt_model::r4::resources::GoalStart> = None;
                let mut r#target: Option<Vec<fhirbolt_model::r4::resources::GoalTarget>> = None;
                let mut r#status_date: Option<fhirbolt_model::r4::types::Date> = None;
                let mut r#status_reason: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#expressed_by: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#addresses: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4::types::Annotation>>> = None;
                let mut r#outcome_code: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#outcome_reference: Option<
                    Vec<Box<fhirbolt_model::r4::types::Reference>>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Goal" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Goal",
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
                        Field::LifecycleStatus => {
                            if self.0.from_json {
                                let some = r#lifecycle_status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lifecycleStatus",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#lifecycle_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lifecycleStatus",
                                    ));
                                }
                                r#lifecycle_status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::LifecycleStatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#lifecycle_status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lifecycleStatus",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lifecycleStatus");
                            }
                        }
                        Field::AchievementStatus => {
                            if r#achievement_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("achievementStatus"));
                            }
                            r#achievement_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Priority => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            r#priority = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Description => {
                            if r#description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            r#description = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::StartDate => {
                            if self.0.from_json {
                                let r#enum = r#start.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalStart::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalStart::Date(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("startDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("start[x]"));
                                }
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("startDate"));
                                }
                                r#start = Some(fhirbolt_model::r4::resources::GoalStart::Date(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4::types::Date>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::StartDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#start.get_or_insert(
                                    fhirbolt_model::r4::resources::GoalStart::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::GoalStart::Date(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_startDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_start[x]"));
                                }
                            } else {
                                return unknown_field_error("startDate");
                            }
                        }
                        Field::StartCodeableConcept => {
                            if r#start.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "startCodeableConcept",
                                ));
                            }
                            r#start = Some (fhirbolt_model :: r4 :: resources :: GoalStart :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::Target => {
                            if self.0.from_json {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: GoalTarget >> ()) ?) ;
                            } else {
                                let vec = r#target.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<fhirbolt_model::r4::resources::GoalTarget>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::StatusDate => {
                            if self.0.from_json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Date>(),
                                )?);
                            }
                        }
                        Field::StatusDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("statusDate");
                            }
                        }
                        Field::StatusReason => {
                            if self.0.from_json {
                                let some = r#status_reason.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::StatusReasonPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status_reason.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusReason"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("statusReason");
                            }
                        }
                        Field::ExpressedBy => {
                            if r#expressed_by.is_some() {
                                return Err(serde::de::Error::duplicate_field("expressedBy"));
                            }
                            r#expressed_by = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Addresses => {
                            if self.0.from_json {
                                if r#addresses.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addresses"));
                                }
                                r#addresses = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#addresses.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::OutcomeCode => {
                            if self.0.from_json {
                                if r#outcome_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeCode"));
                                }
                                r#outcome_code =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#outcome_code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::OutcomeReference => {
                            if self.0.from_json {
                                if r#outcome_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outcomeReference",
                                    ));
                                }
                                r#outcome_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#outcome_reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
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
                Ok(fhirbolt_model::r4::resources::Goal {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#lifecycle_status: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#lifecycle_status.unwrap_or(Default::default())
                    } else {
                        r#lifecycle_status
                            .ok_or(serde::de::Error::missing_field("lifecycleStatus"))?
                    },
                    r#achievement_status,
                    r#category: r#category.unwrap_or(vec![]),
                    r#priority,
                    r#description: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#description.unwrap_or(Default::default())
                    } else {
                        r#description.ok_or(serde::de::Error::missing_field("description"))?
                    },
                    r#subject: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#subject.unwrap_or(Default::default())
                    } else {
                        r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                    },
                    r#start,
                    r#target: r#target.unwrap_or(vec![]),
                    r#status_date,
                    r#status_reason,
                    r#expressed_by,
                    r#addresses: r#addresses.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#outcome_code: r#outcome_code.unwrap_or(vec![]),
                    r#outcome_reference: r#outcome_reference.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4::resources::Goal>>
{
    type Value = Box<fhirbolt_model::r4::resources::Goal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::Goal>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4::resources::Goal>>
{
    type Value = Vec<fhirbolt_model::r4::resources::Goal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::Goal>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::Goal>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq
                    .next_element_seed(self.0.transmute::<fhirbolt_model::r4::resources::Goal>())?
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
        Vec<Box<fhirbolt_model::r4::resources::Goal>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::Goal>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::Goal>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::Goal>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::Goal>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
