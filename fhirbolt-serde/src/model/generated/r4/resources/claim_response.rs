// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::ClaimResponseItemAdjudication;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseItemAdjudication> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.item.adjudication", field
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
        if self.value.r#category.id.as_deref() == Some("$invalid") {
            return missing_field_error("category");
        } else {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if let Some(some) = self.value.r#reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reason", ctx))?;
        }
        if let Some(some) = self.value.r#amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amount", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#value.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("value", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_value", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#value.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("value", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseItemAdjudication>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseItemAdjudication>> {
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
    for &mut DeserializationContext<ClaimResponseItemAdjudication>
{
    type Value = ClaimResponseItemAdjudication;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseItemAdjudication>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseItemAdjudication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemAdjudication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseItemAdjudication, V::Error>
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "amount")]
                    Amount,
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
                            "category",
                            "reason",
                            "amount",
                            "value",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#reason: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#value: Option<fhirbolt_model::r4::types::Decimal> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Reason => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#reason = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValuePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_value"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                Ok(ClaimResponseItemAdjudication {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#category.unwrap_or(Default::default())
                    } else {
                        r#category.ok_or(serde::de::Error::missing_field("category"))?
                    },
                    r#reason,
                    r#amount,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseItemAdjudication>>
{
    type Value = Box<ClaimResponseItemAdjudication>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseItemAdjudication>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseItemAdjudication>>
{
    type Value = Vec<ClaimResponseItemAdjudication>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseItemAdjudication>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseItemAdjudication>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseItemAdjudication> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseItemDetailSubDetail;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseItemDetailSubDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.item.detail.subDetail", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#sub_detail_sequence.id.as_deref() == Some("$invalid") {
                return missing_field_error("subDetailSequence");
            }
            if let Some(some) = self.value.r#sub_detail_sequence.value.as_ref().map(Ok) {
                state.serialize_entry("subDetailSequence", &some?)?;
            }
            if self.value.r#sub_detail_sequence.id.is_some()
                || !self.value.r#sub_detail_sequence.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#sub_detail_sequence.id.as_ref(),
                    extension: &self.value.r#sub_detail_sequence.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_subDetailSequence", ctx)
                })?;
            }
        } else if self.value.r#sub_detail_sequence.id.as_deref() == Some("$invalid") {
            return missing_field_error("subDetailSequence");
        } else {
            self.with_context(&self.value.r#sub_detail_sequence, |ctx| {
                state.serialize_entry("subDetailSequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseItemDetailSubDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseItemDetailSubDetail>> {
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
    for &mut DeserializationContext<ClaimResponseItemDetailSubDetail>
{
    type Value = ClaimResponseItemDetailSubDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseItemDetailSubDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseItemDetailSubDetail, V::Error>
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
                    #[serde(rename = "subDetailSequence")]
                    SubDetailSequence,
                    #[serde(rename = "_subDetailSequence")]
                    SubDetailSequencePrimitiveElement,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "subDetailSequence",
                            "noteNumber",
                            "adjudication",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#sub_detail_sequence: Option<fhirbolt_model::r4::types::PositiveInt> =
                    None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sub_detail_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subDetailSequence",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#sub_detail_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subDetailSequence",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#sub_detail_sequence =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sub_detail_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_subDetailSequence",
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
                                return unknown_field_error("subDetailSequence");
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
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
                Ok(ClaimResponseItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sub_detail_sequence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#sub_detail_sequence.unwrap_or(Default::default())
                    } else {
                        r#sub_detail_sequence
                            .ok_or(serde::de::Error::missing_field("subDetailSequence"))?
                    },
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseItemDetailSubDetail>>
{
    type Value = Box<ClaimResponseItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseItemDetailSubDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseItemDetailSubDetail>>
{
    type Value = Vec<ClaimResponseItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseItemDetailSubDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseItemDetailSubDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseItemDetailSubDetail> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseItemDetail;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseItemDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.item.detail", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#detail_sequence.id.as_deref() == Some("$invalid") {
                return missing_field_error("detailSequence");
            }
            if let Some(some) = self.value.r#detail_sequence.value.as_ref().map(Ok) {
                state.serialize_entry("detailSequence", &some?)?;
            }
            if self.value.r#detail_sequence.id.is_some()
                || !self.value.r#detail_sequence.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#detail_sequence.id.as_ref(),
                    extension: &self.value.r#detail_sequence.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_detailSequence", ctx)
                })?;
            }
        } else if self.value.r#detail_sequence.id.as_deref() == Some("$invalid") {
            return missing_field_error("detailSequence");
        } else {
            self.with_context(&self.value.r#detail_sequence, |ctx| {
                state.serialize_entry("detailSequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        if !self.value.r#sub_detail.is_empty() {
            self.with_context(&self.value.r#sub_detail, |ctx| {
                state.serialize_entry("subDetail", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseItemDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseItemDetail>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseItemDetail> {
    type Value = ClaimResponseItemDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseItemDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseItemDetail, V::Error>
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
                    #[serde(rename = "detailSequence")]
                    DetailSequence,
                    #[serde(rename = "_detailSequence")]
                    DetailSequencePrimitiveElement,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    #[serde(rename = "subDetail")]
                    SubDetail,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "detailSequence",
                            "noteNumber",
                            "adjudication",
                            "subDetail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#detail_sequence: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
                > = None;
                let mut r#sub_detail: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemDetailSubDetail>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#detail_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailSequence",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#detail_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailSequence",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#detail_sequence =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#detail_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_detailSequence",
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
                                return unknown_field_error("detailSequence");
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetail => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemDetailSubDetail >> = self . 0 . transmute () ;
                                r#sub_detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_detail.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemDetailSubDetail,
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
                Ok(ClaimResponseItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#detail_sequence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#detail_sequence.unwrap_or(Default::default())
                    } else {
                        r#detail_sequence
                            .ok_or(serde::de::Error::missing_field("detailSequence"))?
                    },
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseItemDetail>>
{
    type Value = Box<ClaimResponseItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseItemDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseItemDetail>>
{
    type Value = Vec<ClaimResponseItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseItemDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseItemDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseItemDetail> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseItem;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseItem> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.item", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#item_sequence.id.as_deref() == Some("$invalid") {
                return missing_field_error("itemSequence");
            }
            if let Some(some) = self.value.r#item_sequence.value.as_ref().map(Ok) {
                state.serialize_entry("itemSequence", &some?)?;
            }
            if self.value.r#item_sequence.id.is_some()
                || !self.value.r#item_sequence.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#item_sequence.id.as_ref(),
                    extension: &self.value.r#item_sequence.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_itemSequence", ctx)
                })?;
            }
        } else if self.value.r#item_sequence.id.as_deref() == Some("$invalid") {
            return missing_field_error("itemSequence");
        } else {
            self.with_context(&self.value.r#item_sequence, |ctx| {
                state.serialize_entry("itemSequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        if !self.value.r#detail.is_empty() {
            self.with_context(&self.value.r#detail, |ctx| {
                state.serialize_entry("detail", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseItem>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseItem>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseItem> {
    type Value = ClaimResponseItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseItem>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseItem, V::Error>
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
                    #[serde(rename = "itemSequence")]
                    ItemSequence,
                    #[serde(rename = "_itemSequence")]
                    ItemSequencePrimitiveElement,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    #[serde(rename = "detail")]
                    Detail,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "itemSequence",
                            "noteNumber",
                            "adjudication",
                            "detail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#item_sequence: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
                > = None;
                let mut r#detail: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemDetail>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#item_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemSequence"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#item_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemSequence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#item_sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#item_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_itemSequence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("itemSequence");
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Detail => {
                            if self.0.from == crate::context::Format::Json {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemDetail>,
                                > = self.0.transmute();
                                r#detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#detail.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemDetail,
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
                Ok(ClaimResponseItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#item_sequence.unwrap_or(Default::default())
                    } else {
                        r#item_sequence.ok_or(serde::de::Error::missing_field("itemSequence"))?
                    },
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ClaimResponseItem>> {
    type Value = Box<ClaimResponseItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ClaimResponseItem>> {
    type Value = Vec<ClaimResponseItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseItem>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseItem> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseAddItemDetailSubDetail;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseAddItemDetailSubDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.addItem.detail.subDetail", field
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
        if self.value.r#product_or_service.id.as_deref() == Some("$invalid") {
            return missing_field_error("productOrService");
        } else {
            self.with_context(&self.value.r#product_or_service, |ctx| {
                state.serialize_entry("productOrService", ctx)
            })?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("factor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#factor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseAddItemDetailSubDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseAddItemDetailSubDetail>> {
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
    for &mut DeserializationContext<ClaimResponseAddItemDetailSubDetail>
{
    type Value = ClaimResponseAddItemDetailSubDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseAddItemDetailSubDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseAddItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseAddItemDetailSubDetail, V::Error>
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
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "productOrService",
                            "modifier",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "net",
                            "noteNumber",
                            "adjudication",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#net: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
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
                Ok(ClaimResponseAddItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#product_or_service.unwrap_or(Default::default())
                    } else {
                        r#product_or_service
                            .ok_or(serde::de::Error::missing_field("productOrService"))?
                    },
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseAddItemDetailSubDetail>>
{
    type Value = Box<ClaimResponseAddItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseAddItemDetailSubDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseAddItemDetailSubDetail>>
{
    type Value = Vec<ClaimResponseAddItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ClaimResponseAddItemDetailSubDetail>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseAddItemDetailSubDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseAddItemDetailSubDetail> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseAddItemDetail;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseAddItemDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.addItem.detail", field
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
        if self.value.r#product_or_service.id.as_deref() == Some("$invalid") {
            return missing_field_error("productOrService");
        } else {
            self.with_context(&self.value.r#product_or_service, |ctx| {
                state.serialize_entry("productOrService", ctx)
            })?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("factor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#factor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        if !self.value.r#sub_detail.is_empty() {
            self.with_context(&self.value.r#sub_detail, |ctx| {
                state.serialize_entry("subDetail", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseAddItemDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseAddItemDetail>> {
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
    for &mut DeserializationContext<ClaimResponseAddItemDetail>
{
    type Value = ClaimResponseAddItemDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseAddItemDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseAddItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseAddItemDetail, V::Error>
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
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    #[serde(rename = "subDetail")]
                    SubDetail,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "productOrService",
                            "modifier",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "net",
                            "noteNumber",
                            "adjudication",
                            "subDetail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#net: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
                > = None;
                let mut r#sub_detail: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseAddItemDetailSubDetail>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetail => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseAddItemDetailSubDetail >> = self . 0 . transmute () ;
                                r#sub_detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_detail.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: ClaimResponseAddItemDetailSubDetail > = self . 0 . transmute () ;
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
                Ok(ClaimResponseAddItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#product_or_service.unwrap_or(Default::default())
                    } else {
                        r#product_or_service
                            .ok_or(serde::de::Error::missing_field("productOrService"))?
                    },
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseAddItemDetail>>
{
    type Value = Box<ClaimResponseAddItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseAddItemDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseAddItemDetail>>
{
    type Value = Vec<ClaimResponseAddItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseAddItemDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseAddItemDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseAddItemDetail> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseAddItem;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseAddItem> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.addItem", field
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
        if self.output == crate::context::Format::Json {
            if !self.value.r#item_sequence.is_empty() {
                let values = self
                    .value
                    .r#item_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("itemSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#item_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#item_sequence
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_itemSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#item_sequence.is_empty() {
            self.with_context(&self.value.r#item_sequence, |ctx| {
                state.serialize_entry("itemSequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#detail_sequence.is_empty() {
                let values = self
                    .value
                    .r#detail_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("detailSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#detail_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#detail_sequence
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_detailSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#detail_sequence.is_empty() {
            self.with_context(&self.value.r#detail_sequence, |ctx| {
                state.serialize_entry("detailSequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#subdetail_sequence.is_empty() {
                let values = self
                    .value
                    .r#subdetail_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("subdetailSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#subdetail_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#subdetail_sequence
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_subdetailSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#subdetail_sequence.is_empty() {
            self.with_context(&self.value.r#subdetail_sequence, |ctx| {
                state.serialize_entry("subdetailSequence", ctx)
            })?;
        }
        if !self.value.r#provider.is_empty() {
            self.with_context(&self.value.r#provider, |ctx| {
                state.serialize_entry("provider", ctx)
            })?;
        }
        if self.value.r#product_or_service.id.as_deref() == Some("$invalid") {
            return missing_field_error("productOrService");
        } else {
            self.with_context(&self.value.r#product_or_service, |ctx| {
                state.serialize_entry("productOrService", ctx)
            })?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if !self.value.r#program_code.is_empty() {
            self.with_context(&self.value.r#program_code, |ctx| {
                state.serialize_entry("programCode", ctx)
            })?;
        }
        {
            use fhirbolt_model::r4::resources::ClaimResponseAddItemServiced as _Enum;
            if let Some(some) = self.value.r#serviced.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("servicedDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_servicedDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("servicedDate", ctx)
                            })?;
                        }
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("servicedPeriod", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("serviced is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r4::resources::ClaimResponseAddItemLocation as _Enum;
            if let Some(some) = self.value.r#location.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("locationCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Address(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("locationAddress", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("locationReference", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("location is invalid")),
                }
            }
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("factor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#factor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if let Some(some) = self.value.r#body_site.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("bodySite", ctx))?;
        }
        if !self.value.r#sub_site.is_empty() {
            self.with_context(&self.value.r#sub_site, |ctx| {
                state.serialize_entry("subSite", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#note_number.is_empty() {
                let values = self
                    .value
                    .r#note_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("noteNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#note_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#note_number
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_noteNumber", ctx)
                    })?;
                }
            }
        } else if !self.value.r#note_number.is_empty() {
            self.with_context(&self.value.r#note_number, |ctx| {
                state.serialize_entry("noteNumber", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        if !self.value.r#detail.is_empty() {
            self.with_context(&self.value.r#detail, |ctx| {
                state.serialize_entry("detail", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseAddItem>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseAddItem>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseAddItem> {
    type Value = ClaimResponseAddItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseAddItem>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseAddItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseAddItem, V::Error>
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
                    #[serde(rename = "itemSequence")]
                    ItemSequence,
                    #[serde(rename = "_itemSequence")]
                    ItemSequencePrimitiveElement,
                    #[serde(rename = "detailSequence")]
                    DetailSequence,
                    #[serde(rename = "_detailSequence")]
                    DetailSequencePrimitiveElement,
                    #[serde(rename = "subdetailSequence")]
                    SubdetailSequence,
                    #[serde(rename = "_subdetailSequence")]
                    SubdetailSequencePrimitiveElement,
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "programCode")]
                    ProgramCode,
                    #[serde(rename = "servicedDate")]
                    ServicedDate,
                    #[serde(rename = "_servicedDate")]
                    ServicedDatePrimitiveElement,
                    #[serde(rename = "servicedPeriod")]
                    ServicedPeriod,
                    #[serde(rename = "locationCodeableConcept")]
                    LocationCodeableConcept,
                    #[serde(rename = "locationAddress")]
                    LocationAddress,
                    #[serde(rename = "locationReference")]
                    LocationReference,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "bodySite")]
                    BodySite,
                    #[serde(rename = "subSite")]
                    SubSite,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    #[serde(rename = "detail")]
                    Detail,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "itemSequence",
                            "detailSequence",
                            "subdetailSequence",
                            "provider",
                            "productOrService",
                            "modifier",
                            "programCode",
                            "servicedDate",
                            "servicedPeriod",
                            "locationCodeableConcept",
                            "locationAddress",
                            "locationReference",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "net",
                            "bodySite",
                            "subSite",
                            "noteNumber",
                            "adjudication",
                            "detail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#item_sequence: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#detail_sequence: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> =
                    None;
                let mut r#subdetail_sequence: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> =
                    None;
                let mut r#provider: Option<Vec<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#program_code: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#serviced: Option<
                    fhirbolt_model::r4::resources::ClaimResponseAddItemServiced,
                > = None;
                let mut r#location: Option<
                    fhirbolt_model::r4::resources::ClaimResponseAddItemLocation,
                > = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#net: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#body_site: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r4::types::PositiveInt>> = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
                > = None;
                let mut r#detail: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseAddItemDetail>,
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#item_sequence.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("itemSequence"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#item_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#item_sequence.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_itemSequence"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("itemSequence");
                            }
                        }
                        Field::DetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#detail_sequence.get_or_insert(
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
                                        "detailSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#detail_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#detail_sequence.get_or_insert(
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
                                        "_detailSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("detailSequence");
                            }
                        }
                        Field::SubdetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#subdetail_sequence.get_or_insert(
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
                                        "subdetailSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#subdetail_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubdetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#subdetail_sequence.get_or_insert(
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
                                        "_subdetailSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("subdetailSequence");
                            }
                        }
                        Field::Provider => {
                            if self.0.from == crate::context::Format::Json {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#provider = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#provider.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProgramCode => {
                            if self.0.from == crate::context::Format::Json {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ServicedDate => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemServiced as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#serviced.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "servicedDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("serviced[x]"));
                                }
                            } else {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Date,
                                > = self.0.transmute();
                                r#serviced =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemServiced as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#serviced.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_servicedDate",
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
                                    return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                                }
                            } else {
                                return unknown_field_error("servicedDate");
                            }
                        }
                        Field::ServicedPeriod => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemServiced as _Enum;
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#serviced =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationCodeableConcept => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::LocationAddress => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Address>,
                            > = self.0.transmute();
                            r#location =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationReference => {
                            use fhirbolt_model::r4::resources::ClaimResponseAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#body_site = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SubSite => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#sub_site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("noteNumber"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#note_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#note_number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_noteNumber"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("noteNumber");
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Detail => {
                            if self.0.from == crate::context::Format::Json {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseAddItemDetail>,
                                > = self.0.transmute();
                                r#detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#detail.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseAddItemDetail,
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
                Ok(ClaimResponseAddItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: r#item_sequence.unwrap_or(vec![]),
                    r#detail_sequence: r#detail_sequence.unwrap_or(vec![]),
                    r#subdetail_sequence: r#subdetail_sequence.unwrap_or(vec![]),
                    r#provider: r#provider.unwrap_or(vec![]),
                    r#product_or_service: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#product_or_service.unwrap_or(Default::default())
                    } else {
                        r#product_or_service
                            .ok_or(serde::de::Error::missing_field("productOrService"))?
                    },
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#body_site,
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseAddItem>>
{
    type Value = Box<ClaimResponseAddItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseAddItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseAddItem>>
{
    type Value = Vec<ClaimResponseAddItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseAddItem>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseAddItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseAddItem> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseTotal;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseTotal> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.total", field
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
        if self.value.r#category.id.as_deref() == Some("$invalid") {
            return missing_field_error("category");
        } else {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if self.value.r#amount.id.as_deref() == Some("$invalid") {
            return missing_field_error("amount");
        } else {
            self.with_context(&self.value.r#amount, |ctx| {
                state.serialize_entry("amount", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseTotal>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseTotal>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseTotal> {
    type Value = ClaimResponseTotal;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseTotal>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseTotal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseTotal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseTotal, V::Error>
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "amount")]
                    Amount,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "category", "amount"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r4::types::Money>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ClaimResponseTotal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#category.unwrap_or(Default::default())
                    } else {
                        r#category.ok_or(serde::de::Error::missing_field("category"))?
                    },
                    r#amount: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#amount.unwrap_or(Default::default())
                    } else {
                        r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ClaimResponseTotal>> {
    type Value = Box<ClaimResponseTotal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseTotal>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ClaimResponseTotal>> {
    type Value = Vec<ClaimResponseTotal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseTotal>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseTotal>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseTotal> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponsePayment;
impl serde::ser::Serialize for SerializationContext<&ClaimResponsePayment> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.payment", field
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
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#adjustment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("adjustment", ctx))?;
        }
        if let Some(some) = self.value.r#adjustment_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("adjustmentReason", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("date", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_date", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("date", ctx))?;
        }
        if self.value.r#amount.id.as_deref() == Some("$invalid") {
            return missing_field_error("amount");
        } else {
            self.with_context(&self.value.r#amount, |ctx| {
                state.serialize_entry("amount", ctx)
            })?;
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponsePayment>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponsePayment>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponsePayment> {
    type Value = ClaimResponsePayment;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponsePayment>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponsePayment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponsePayment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponsePayment, V::Error>
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
                    #[serde(rename = "adjustment")]
                    Adjustment,
                    #[serde(rename = "adjustmentReason")]
                    AdjustmentReason,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "amount")]
                    Amount,
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
                            "type",
                            "adjustment",
                            "adjustmentReason",
                            "date",
                            "amount",
                            "identifier",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#adjustment: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#adjustment_reason: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#date: Option<fhirbolt_model::r4::types::Date> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjustment => {
                            if r#adjustment.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#adjustment = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AdjustmentReason => {
                            if r#adjustment_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustmentReason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#adjustment_reason = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Date => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Date,
                                > = self.0.transmute();
                                r#date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("date");
                            }
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ClaimResponsePayment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#adjustment,
                    r#adjustment_reason,
                    r#date,
                    r#amount: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#amount.unwrap_or(Default::default())
                    } else {
                        r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                    },
                    r#identifier,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponsePayment>>
{
    type Value = Box<ClaimResponsePayment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponsePayment>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponsePayment>>
{
    type Value = Vec<ClaimResponsePayment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponsePayment>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponsePayment>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponsePayment> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseProcessNote;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseProcessNote> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.processNote", field
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("number", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_number", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("number", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("type", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_type", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#text.id.as_deref() == Some("$invalid") {
                return missing_field_error("text");
            }
            if let Some(some) = self.value.r#text.value.as_ref().map(Ok) {
                state.serialize_entry("text", &some?)?;
            }
            if self.value.r#text.id.is_some() || !self.value.r#text.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#text.id.as_ref(),
                    extension: &self.value.r#text.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_text", ctx)
                })?;
            }
        } else if self.value.r#text.id.as_deref() == Some("$invalid") {
            return missing_field_error("text");
        } else {
            self.with_context(&self.value.r#text, |ctx| state.serialize_entry("text", ctx))?;
        }
        if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseProcessNote>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseProcessNote>> {
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
    for &mut DeserializationContext<ClaimResponseProcessNote>
{
    type Value = ClaimResponseProcessNote;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseProcessNote>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseProcessNote")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseProcessNote, V::Error>
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
                    #[serde(rename = "number")]
                    Number,
                    #[serde(rename = "_number")]
                    NumberPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "language")]
                    Language,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "number",
                            "type",
                            "text",
                            "language",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#number: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#language: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Number => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("number"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("number"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NumberPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_number"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("number");
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Text => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#text = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#language = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ClaimResponseProcessNote {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number,
                    r#type,
                    r#text: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#text.unwrap_or(Default::default())
                    } else {
                        r#text.ok_or(serde::de::Error::missing_field("text"))?
                    },
                    r#language,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseProcessNote>>
{
    type Value = Box<ClaimResponseProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseProcessNote>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseProcessNote>>
{
    type Value = Vec<ClaimResponseProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseProcessNote>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseProcessNote>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseProcessNote> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseInsurance;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseInsurance> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.insurance", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#sequence.id.as_deref() == Some("$invalid") {
                return missing_field_error("sequence");
            }
            if let Some(some) = self.value.r#sequence.value.as_ref().map(Ok) {
                state.serialize_entry("sequence", &some?)?;
            }
            if self.value.r#sequence.id.is_some() || !self.value.r#sequence.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#sequence.id.as_ref(),
                    extension: &self.value.r#sequence.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_sequence", ctx)
                })?;
            }
        } else if self.value.r#sequence.id.as_deref() == Some("$invalid") {
            return missing_field_error("sequence");
        } else {
            self.with_context(&self.value.r#sequence, |ctx| {
                state.serialize_entry("sequence", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#focal.id.as_deref() == Some("$invalid") {
                return missing_field_error("focal");
            }
            if let Some(some) = self.value.r#focal.value.as_ref().map(Ok) {
                state.serialize_entry("focal", &some?)?;
            }
            if self.value.r#focal.id.is_some() || !self.value.r#focal.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#focal.id.as_ref(),
                    extension: &self.value.r#focal.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_focal", ctx)
                })?;
            }
        } else if self.value.r#focal.id.as_deref() == Some("$invalid") {
            return missing_field_error("focal");
        } else {
            self.with_context(&self.value.r#focal, |ctx| {
                state.serialize_entry("focal", ctx)
            })?;
        }
        if self.value.r#coverage.id.as_deref() == Some("$invalid") {
            return missing_field_error("coverage");
        } else {
            self.with_context(&self.value.r#coverage, |ctx| {
                state.serialize_entry("coverage", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#business_arrangement.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("businessArrangement", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_businessArrangement", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#business_arrangement.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("businessArrangement", ctx)
            })?;
        }
        if let Some(some) = self.value.r#claim_response.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("claimResponse", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseInsurance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseInsurance>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseInsurance> {
    type Value = ClaimResponseInsurance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseInsurance>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseInsurance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseInsurance, V::Error>
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
                    #[serde(rename = "sequence")]
                    Sequence,
                    #[serde(rename = "_sequence")]
                    SequencePrimitiveElement,
                    #[serde(rename = "focal")]
                    Focal,
                    #[serde(rename = "_focal")]
                    FocalPrimitiveElement,
                    #[serde(rename = "coverage")]
                    Coverage,
                    #[serde(rename = "businessArrangement")]
                    BusinessArrangement,
                    #[serde(rename = "_businessArrangement")]
                    BusinessArrangementPrimitiveElement,
                    #[serde(rename = "claimResponse")]
                    ClaimResponse,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "sequence",
                            "focal",
                            "coverage",
                            "businessArrangement",
                            "claimResponse",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#focal: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#coverage: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#business_arrangement: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#claim_response: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Sequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sequence");
                            }
                        }
                        Field::Focal => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#focal.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focal"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#focal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focal"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#focal = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FocalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#focal.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_focal"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("focal");
                            }
                        }
                        Field::Coverage => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#coverage = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BusinessArrangement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#business_arrangement.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "businessArrangement",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#business_arrangement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "businessArrangement",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#business_arrangement =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BusinessArrangementPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#business_arrangement.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_businessArrangement",
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
                                return unknown_field_error("businessArrangement");
                            }
                        }
                        Field::ClaimResponse => {
                            if r#claim_response.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimResponse"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#claim_response = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ClaimResponseInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#sequence.unwrap_or(Default::default())
                    } else {
                        r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                    },
                    r#focal: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#focal.unwrap_or(Default::default())
                    } else {
                        r#focal.ok_or(serde::de::Error::missing_field("focal"))?
                    },
                    r#coverage: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#coverage.unwrap_or(Default::default())
                    } else {
                        r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                    },
                    r#business_arrangement,
                    r#claim_response,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ClaimResponseInsurance>>
{
    type Value = Box<ClaimResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseInsurance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ClaimResponseInsurance>>
{
    type Value = Vec<ClaimResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseInsurance>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseInsurance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseInsurance> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponseError;
impl serde::ser::Serialize for SerializationContext<&ClaimResponseError> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse.error", field
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#item_sequence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("itemSequence", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_itemSequence", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#item_sequence.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("itemSequence", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#detail_sequence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("detailSequence", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_detailSequence", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#detail_sequence.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("detailSequence", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#sub_detail_sequence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("subDetailSequence", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_subDetailSequence", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#sub_detail_sequence.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subDetailSequence", ctx))?;
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponseError>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponseError>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponseError> {
    type Value = ClaimResponseError;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponseError>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponseError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseError")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseError, V::Error>
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
                    #[serde(rename = "itemSequence")]
                    ItemSequence,
                    #[serde(rename = "_itemSequence")]
                    ItemSequencePrimitiveElement,
                    #[serde(rename = "detailSequence")]
                    DetailSequence,
                    #[serde(rename = "_detailSequence")]
                    DetailSequencePrimitiveElement,
                    #[serde(rename = "subDetailSequence")]
                    SubDetailSequence,
                    #[serde(rename = "_subDetailSequence")]
                    SubDetailSequencePrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "itemSequence",
                            "detailSequence",
                            "subDetailSequence",
                            "code",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#item_sequence: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#detail_sequence: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#sub_detail_sequence: Option<fhirbolt_model::r4::types::PositiveInt> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#item_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemSequence"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#item_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemSequence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#item_sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#item_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_itemSequence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("itemSequence");
                            }
                        }
                        Field::DetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#detail_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailSequence",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#detail_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailSequence",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#detail_sequence =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#detail_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_detailSequence",
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
                                return unknown_field_error("detailSequence");
                            }
                        }
                        Field::SubDetailSequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sub_detail_sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subDetailSequence",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#sub_detail_sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subDetailSequence",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#sub_detail_sequence =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetailSequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sub_detail_sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_subDetailSequence",
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
                                return unknown_field_error("subDetailSequence");
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ClaimResponseError {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence,
                    r#detail_sequence,
                    r#sub_detail_sequence,
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ClaimResponseError>> {
    type Value = Box<ClaimResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponseError>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ClaimResponseError>> {
    type Value = Vec<ClaimResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponseError>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponseError>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponseError> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::ClaimResponse;
impl crate::Resource for ClaimResponse {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&ClaimResponse> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ClaimResponse", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ClaimResponse")?;
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
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                state.serialize_entry("status", &some?)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#sub_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subType", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#use.id.as_deref() == Some("$invalid") {
                return missing_field_error("use");
            }
            if let Some(some) = self.value.r#use.value.as_ref().map(Ok) {
                state.serialize_entry("use", &some?)?;
            }
            if self.value.r#use.id.is_some() || !self.value.r#use.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#use.id.as_ref(),
                    extension: &self.value.r#use.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_use", ctx))?;
            }
        } else if self.value.r#use.id.as_deref() == Some("$invalid") {
            return missing_field_error("use");
        } else {
            self.with_context(&self.value.r#use, |ctx| state.serialize_entry("use", ctx))?;
        }
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        } else {
            self.with_context(&self.value.r#patient, |ctx| {
                state.serialize_entry("patient", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#created.id.as_deref() == Some("$invalid") {
                return missing_field_error("created");
            }
            if let Some(some) = self.value.r#created.value.as_ref().map(Ok) {
                state.serialize_entry("created", &some?)?;
            }
            if self.value.r#created.id.is_some() || !self.value.r#created.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#created.id.as_ref(),
                    extension: &self.value.r#created.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_created", ctx)
                })?;
            }
        } else if self.value.r#created.id.as_deref() == Some("$invalid") {
            return missing_field_error("created");
        } else {
            self.with_context(&self.value.r#created, |ctx| {
                state.serialize_entry("created", ctx)
            })?;
        }
        if self.value.r#insurer.id.as_deref() == Some("$invalid") {
            return missing_field_error("insurer");
        } else {
            self.with_context(&self.value.r#insurer, |ctx| {
                state.serialize_entry("insurer", ctx)
            })?;
        }
        if let Some(some) = self.value.r#requestor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestor", ctx))?;
        }
        if let Some(some) = self.value.r#request.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("request", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#outcome.id.as_deref() == Some("$invalid") {
                return missing_field_error("outcome");
            }
            if let Some(some) = self.value.r#outcome.value.as_ref().map(Ok) {
                state.serialize_entry("outcome", &some?)?;
            }
            if self.value.r#outcome.id.is_some() || !self.value.r#outcome.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#outcome.id.as_ref(),
                    extension: &self.value.r#outcome.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_outcome", ctx)
                })?;
            }
        } else if self.value.r#outcome.id.as_deref() == Some("$invalid") {
            return missing_field_error("outcome");
        } else {
            self.with_context(&self.value.r#outcome, |ctx| {
                state.serialize_entry("outcome", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#disposition.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("disposition", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_disposition", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#disposition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("disposition", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#pre_auth_ref.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("preAuthRef", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_preAuthRef", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#pre_auth_ref.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("preAuthRef", ctx))?;
        }
        if let Some(some) = self.value.r#pre_auth_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("preAuthPeriod", ctx))?;
        }
        if let Some(some) = self.value.r#payee_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("payeeType", ctx))?;
        }
        if !self.value.r#item.is_empty() {
            self.with_context(&self.value.r#item, |ctx| state.serialize_entry("item", ctx))?;
        }
        if !self.value.r#add_item.is_empty() {
            self.with_context(&self.value.r#add_item, |ctx| {
                state.serialize_entry("addItem", ctx)
            })?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        if !self.value.r#total.is_empty() {
            self.with_context(&self.value.r#total, |ctx| {
                state.serialize_entry("total", ctx)
            })?;
        }
        if let Some(some) = self.value.r#payment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("payment", ctx))?;
        }
        if let Some(some) = self.value.r#funds_reserve.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("fundsReserve", ctx))?;
        }
        if let Some(some) = self.value.r#form_code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("formCode", ctx))?;
        }
        if let Some(some) = self.value.r#form.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("form", ctx))?;
        }
        if !self.value.r#process_note.is_empty() {
            self.with_context(&self.value.r#process_note, |ctx| {
                state.serialize_entry("processNote", ctx)
            })?;
        }
        if !self.value.r#communication_request.is_empty() {
            self.with_context(&self.value.r#communication_request, |ctx| {
                state.serialize_entry("communicationRequest", ctx)
            })?;
        }
        if !self.value.r#insurance.is_empty() {
            self.with_context(&self.value.r#insurance, |ctx| {
                state.serialize_entry("insurance", ctx)
            })?;
        }
        if !self.value.r#error.is_empty() {
            self.with_context(&self.value.r#error, |ctx| {
                state.serialize_entry("error", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ClaimResponse>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ClaimResponse>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<ClaimResponse> {
    type Value = ClaimResponse;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ClaimResponse> {
    type Value = ClaimResponse;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ClaimResponse>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ClaimResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponse")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponse, V::Error>
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
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subType")]
                    SubType,
                    #[serde(rename = "use")]
                    Use,
                    #[serde(rename = "_use")]
                    UsePrimitiveElement,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "insurer")]
                    Insurer,
                    #[serde(rename = "requestor")]
                    Requestor,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "_outcome")]
                    OutcomePrimitiveElement,
                    #[serde(rename = "disposition")]
                    Disposition,
                    #[serde(rename = "_disposition")]
                    DispositionPrimitiveElement,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    #[serde(rename = "preAuthPeriod")]
                    PreAuthPeriod,
                    #[serde(rename = "payeeType")]
                    PayeeType,
                    #[serde(rename = "item")]
                    Item,
                    #[serde(rename = "addItem")]
                    AddItem,
                    #[serde(rename = "adjudication")]
                    Adjudication,
                    #[serde(rename = "total")]
                    Total,
                    #[serde(rename = "payment")]
                    Payment,
                    #[serde(rename = "fundsReserve")]
                    FundsReserve,
                    #[serde(rename = "formCode")]
                    FormCode,
                    #[serde(rename = "form")]
                    Form,
                    #[serde(rename = "processNote")]
                    ProcessNote,
                    #[serde(rename = "communicationRequest")]
                    CommunicationRequest,
                    #[serde(rename = "insurance")]
                    Insurance,
                    #[serde(rename = "error")]
                    Error,
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
                            "status",
                            "type",
                            "subType",
                            "use",
                            "patient",
                            "created",
                            "insurer",
                            "requestor",
                            "request",
                            "outcome",
                            "disposition",
                            "preAuthRef",
                            "preAuthPeriod",
                            "payeeType",
                            "item",
                            "addItem",
                            "adjudication",
                            "total",
                            "payment",
                            "fundsReserve",
                            "formCode",
                            "form",
                            "processNote",
                            "communicationRequest",
                            "insurance",
                            "error",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#use: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#created: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#insurer: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#requestor: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#request: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#outcome: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#disposition: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#pre_auth_ref: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#pre_auth_period: Option<Box<fhirbolt_model::r4::types::Period>> = None;
                let mut r#payee_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#item: Option<Vec<fhirbolt_model::r4::resources::ClaimResponseItem>> =
                    None;
                let mut r#add_item: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseAddItem>,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseItemAdjudication>,
                > = None;
                let mut r#total: Option<Vec<fhirbolt_model::r4::resources::ClaimResponseTotal>> =
                    None;
                let mut r#payment: Option<fhirbolt_model::r4::resources::ClaimResponsePayment> =
                    None;
                let mut r#funds_reserve: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#form_code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#form: Option<Box<fhirbolt_model::r4::types::Attachment>> = None;
                let mut r#process_note: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseProcessNote>,
                > = None;
                let mut r#communication_request: Option<Vec<fhirbolt_model::r4::types::Reference>> =
                    None;
                let mut r#insurance: Option<
                    Vec<fhirbolt_model::r4::resources::ClaimResponseInsurance>,
                > = None;
                let mut r#error: Option<Vec<fhirbolt_model::r4::resources::ClaimResponseError>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "ClaimResponse" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"ClaimResponse",
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
                                    fhirbolt_model::r4::types::Id,
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
                                Box<fhirbolt_model::r4::types::Meta>,
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
                                    fhirbolt_model::r4::types::Uri,
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
                                    fhirbolt_model::r4::types::Code,
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SubType => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#sub_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Use => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#use.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#use = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UsePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#use.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_use"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("use");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Created => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#created.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DateTime,
                                > = self.0.transmute();
                                r#created = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CreatedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_created"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("created");
                            }
                        }
                        Field::Insurer => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#insurer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Requestor => {
                            if r#requestor.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#requestor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#request = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Outcome => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#outcome = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OutcomePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcome"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("outcome");
                            }
                        }
                        Field::Disposition => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disposition"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#disposition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disposition"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#disposition = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DispositionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_disposition"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("disposition");
                            }
                        }
                        Field::PreAuthRef => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#pre_auth_ref.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#pre_auth_ref = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preAuthRef");
                            }
                        }
                        Field::PreAuthPeriod => {
                            if r#pre_auth_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("preAuthPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#pre_auth_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PayeeType => {
                            if r#payee_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("payeeType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#payee_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Item => {
                            if self.0.from == crate::context::Format::Json {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseItem>,
                                > = self.0.transmute();
                                r#item = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#item.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItem,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AddItem => {
                            if self.0.from == crate::context::Format::Json {
                                if r#add_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addItem"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseAddItem>,
                                > = self.0.transmute();
                                r#add_item = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#add_item.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseAddItem,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: ClaimResponseItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseItemAdjudication,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Total => {
                            if self.0.from == crate::context::Format::Json {
                                if r#total.is_some() {
                                    return Err(serde::de::Error::duplicate_field("total"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseTotal>,
                                > = self.0.transmute();
                                r#total = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#total.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseTotal,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Payment => {
                            if r#payment.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::ClaimResponsePayment,
                            > = self.0.transmute();
                            r#payment = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::FundsReserve => {
                            if r#funds_reserve.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundsReserve"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#funds_reserve = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::FormCode => {
                            if r#form_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("formCode"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#form_code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Attachment>,
                            > = self.0.transmute();
                            r#form = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProcessNote => {
                            if self.0.from == crate::context::Format::Json {
                                if r#process_note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processNote"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseProcessNote>,
                                > = self.0.transmute();
                                r#process_note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#process_note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseProcessNote,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommunicationRequest => {
                            if self.0.from == crate::context::Format::Json {
                                if r#communication_request.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "communicationRequest",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#communication_request =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#communication_request.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Insurance => {
                            if self.0.from == crate::context::Format::Json {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseInsurance>,
                                > = self.0.transmute();
                                r#insurance = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#insurance.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseInsurance,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Error => {
                            if self.0.from == crate::context::Format::Json {
                                if r#error.is_some() {
                                    return Err(serde::de::Error::duplicate_field("error"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::ClaimResponseError>,
                                > = self.0.transmute();
                                r#error = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#error.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::ClaimResponseError,
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
                Ok(ClaimResponse {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#sub_type,
                    r#use: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#use.unwrap_or(Default::default())
                    } else {
                        r#use.ok_or(serde::de::Error::missing_field("use"))?
                    },
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                    },
                    r#created: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#created.unwrap_or(Default::default())
                    } else {
                        r#created.ok_or(serde::de::Error::missing_field("created"))?
                    },
                    r#insurer: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#insurer.unwrap_or(Default::default())
                    } else {
                        r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?
                    },
                    r#requestor,
                    r#request,
                    r#outcome: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#outcome.unwrap_or(Default::default())
                    } else {
                        r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                    },
                    r#disposition,
                    r#pre_auth_ref,
                    r#pre_auth_period,
                    r#payee_type,
                    r#item: r#item.unwrap_or(vec![]),
                    r#add_item: r#add_item.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#total: r#total.unwrap_or(vec![]),
                    r#payment,
                    r#funds_reserve,
                    r#form_code,
                    r#form,
                    r#process_note: r#process_note.unwrap_or(vec![]),
                    r#communication_request: r#communication_request.unwrap_or(vec![]),
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#error: r#error.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ClaimResponse>> {
    type Value = Box<ClaimResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ClaimResponse>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ClaimResponse>> {
    type Value = Vec<ClaimResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ClaimResponse>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ClaimResponse>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ClaimResponse> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
