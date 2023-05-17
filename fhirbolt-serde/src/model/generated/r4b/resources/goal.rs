// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::resources::GoalTarget;
impl serde::ser::Serialize for SerializationContext<&GoalTarget> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Goal.target", field
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
        if let Some(some) = self.value.r#measure.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("measure", ctx))?;
        }
        {
            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
            if let Some(some) = self.value.r#detail.as_ref() {
                match some {
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("detailQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("detailRange", ctx))?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("detailCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("detailString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_detailString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("detailString", ctx)
                            })?;
                        }
                    }
                    _Enum::Boolean(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("detailBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Integer(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("detailInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("detailRatio", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("detail is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r4b::resources::GoalTargetDue as _Enum;
            if let Some(some) = self.value.r#due.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("dueDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Duration(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("dueDuration", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("due is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<GoalTarget>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<GoalTarget>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<GoalTarget> {
    type Value = GoalTarget;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<GoalTarget>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = GoalTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GoalTarget")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GoalTarget, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#measure: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#detail: Option<fhirbolt_model::r4b::resources::GoalTargetDetail> = None;
                let mut r#due: Option<fhirbolt_model::r4b::resources::GoalTargetDue> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Measure => {
                            if r#measure.is_some() {
                                return Err(serde::de::Error::duplicate_field("measure"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#measure = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::DetailQuantity => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#detail =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DetailRange => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#detail =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DetailCodeableConcept => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "detailCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#detail = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DetailString => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#detail = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DetailStringPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Boolean>,
                                > = self.0.transmute();
                                r#detail = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DetailBooleanPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("detail[x]"));
                                }
                            } else {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailInteger"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#detail = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DetailIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#detail.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_detailInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4b::resources::GoalTargetDetail as _Enum;
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#detail =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DueDate => {
                            use fhirbolt_model::r4b::resources::GoalTargetDue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#due.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("dueDate"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("due[x]"));
                                }
                            } else {
                                if r#due.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dueDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#due =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DueDatePrimitiveElement => {
                            use fhirbolt_model::r4b::resources::GoalTargetDue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#due.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_dueDate"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4b::resources::GoalTargetDue as _Enum;
                            if r#due.is_some() {
                                return Err(serde::de::Error::duplicate_field("dueDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#due =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(GoalTarget {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<GoalTarget>> {
    type Value = Box<GoalTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<GoalTarget>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<GoalTarget>> {
    type Value = Vec<GoalTarget>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<GoalTarget>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<GoalTarget>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<GoalTarget> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::Goal;
impl crate::Resource for Goal {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for SerializationContext<&Goal> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Goal", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Goal")?;
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
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
        if self.output == crate::context::Format::Json {
            if self.value.r#lifecycle_status.id.as_deref() == Some("$invalid") {
                return missing_field_error("lifecycleStatus");
            }
            if let Some(some) = self.value.r#lifecycle_status.value.as_ref().map(Ok) {
                state.serialize_entry("lifecycleStatus", &some?)?;
            }
            if self.value.r#lifecycle_status.id.is_some()
                || !self.value.r#lifecycle_status.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#lifecycle_status.id.as_ref(),
                    extension: &self.value.r#lifecycle_status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_lifecycleStatus", ctx)
                })?;
            }
        } else if self.value.r#lifecycle_status.id.as_deref() == Some("$invalid") {
            return missing_field_error("lifecycleStatus");
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
        if self.value.r#description.id.as_deref() == Some("$invalid") {
            return missing_field_error("description");
        } else {
            self.with_context(&self.value.r#description, |ctx| {
                state.serialize_entry("description", ctx)
            })?;
        }
        if self.value.r#subject.id.as_deref() == Some("$invalid") {
            return missing_field_error("subject");
        } else {
            self.with_context(&self.value.r#subject, |ctx| {
                state.serialize_entry("subject", ctx)
            })?;
        }
        {
            use fhirbolt_model::r4b::resources::GoalStart as _Enum;
            if let Some(some) = self.value.r#start.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("startDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_startDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("startDate", ctx)
                            })?;
                        }
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("startCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("start is invalid")),
                }
            }
        }
        if !self.value.r#target.is_empty() {
            self.with_context(&self.value.r#target, |ctx| {
                state.serialize_entry("target", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("statusDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("statusDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status_reason.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("statusReason", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusReason", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("statusReason", ctx))?;
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
impl serde::ser::Serialize for SerializationContext<&Box<Goal>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Goal>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Goal> {
    type Value = Goal;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Goal> {
    type Value = Goal;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Goal>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Goal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Goal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Goal, V::Error>
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
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                let mut r#id: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4b::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4b::types::Identifier>> = None;
                let mut r#lifecycle_status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#achievement_status: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#category: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#priority: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#description: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#start: Option<fhirbolt_model::r4b::resources::GoalStart> = None;
                let mut r#target: Option<Vec<fhirbolt_model::r4b::resources::GoalTarget>> = None;
                let mut r#status_date: Option<fhirbolt_model::r4b::types::Date> = None;
                let mut r#status_reason: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#expressed_by: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#addresses: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r4b::types::Annotation>> = None;
                let mut r#outcome_code: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#outcome_reference: Option<Vec<fhirbolt_model::r4b::types::Reference>> =
                    None;
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
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LifecycleStatus => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lifecycle_status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lifecycleStatus",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#lifecycle_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lifecycleStatus",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#lifecycle_status =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LifecycleStatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#lifecycle_status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lifecycleStatus",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#achievement_status =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Category => {
                            if self.0.from == crate::context::Format::Json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Priority => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#priority = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Description => {
                            if r#description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#description = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::StartDate => {
                            use fhirbolt_model::r4b::resources::GoalStart as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#start.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("startDate"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("start[x]"));
                                }
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("startDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#start =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::StartDatePrimitiveElement => {
                            use fhirbolt_model::r4b::resources::GoalStart as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#start.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_startDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4b::resources::GoalStart as _Enum;
                            if r#start.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "startCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#start = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Target => {
                            if self.0.from == crate::context::Format::Json {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::resources::GoalTarget>,
                                > = self.0.transmute();
                                r#target = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#target.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::resources::GoalTarget,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Date,
                                > = self.0.transmute();
                                r#status_date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusDate"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("statusDate");
                            }
                        }
                        Field::StatusReason => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_reason.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#status_reason = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusReasonPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_reason.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusReason"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#expressed_by = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Addresses => {
                            if self.0.from == crate::context::Format::Json {
                                if r#addresses.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addresses"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#addresses = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#addresses.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Annotation,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OutcomeCode => {
                            if self.0.from == crate::context::Format::Json {
                                if r#outcome_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#outcome_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#outcome_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OutcomeReference => {
                            if self.0.from == crate::context::Format::Json {
                                if r#outcome_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outcomeReference",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#outcome_reference =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#outcome_reference.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(Goal {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Goal>> {
    type Value = Box<Goal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Goal>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Goal>> {
    type Value = Vec<Goal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Goal>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Goal>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Goal> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
