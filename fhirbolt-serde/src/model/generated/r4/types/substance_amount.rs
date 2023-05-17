// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::SubstanceAmountReferenceRange;
impl serde::ser::Serialize for SerializationContext<&SubstanceAmountReferenceRange> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceAmount.referenceRange", field
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
        if let Some(some) = self.value.r#low_limit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("lowLimit", ctx))?;
        }
        if let Some(some) = self.value.r#high_limit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("highLimit", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceAmountReferenceRange>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceAmountReferenceRange>> {
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
    for &mut DeserializationContext<SubstanceAmountReferenceRange>
{
    type Value = SubstanceAmountReferenceRange;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceAmountReferenceRange>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceAmountReferenceRange;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmountReferenceRange")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceAmountReferenceRange, V::Error>
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
                    #[serde(rename = "lowLimit")]
                    LowLimit,
                    #[serde(rename = "highLimit")]
                    HighLimit,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "lowLimit", "highLimit"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#low_limit: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#high_limit: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
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
                        Field::LowLimit => {
                            if r#low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("lowLimit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#low_limit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::HighLimit => {
                            if r#high_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("highLimit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#high_limit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SubstanceAmountReferenceRange {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#low_limit,
                    r#high_limit,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubstanceAmountReferenceRange>>
{
    type Value = Box<SubstanceAmountReferenceRange>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceAmountReferenceRange>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubstanceAmountReferenceRange>>
{
    type Value = Vec<SubstanceAmountReferenceRange>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceAmountReferenceRange>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceAmountReferenceRange>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceAmountReferenceRange> =
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
use fhirbolt_model::r4::types::SubstanceAmount;
impl serde::ser::Serialize for SerializationContext<&SubstanceAmount> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceAmount", field
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
        {
            use fhirbolt_model::r4::types::SubstanceAmountAmount as _Enum;
            if let Some(some) = self.value.r#amount.as_ref() {
                match some {
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("amountQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("amountRange", ctx))?;
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("amountString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_amountString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("amountString", ctx)
                            })?;
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("amount is invalid")),
                }
            }
        }
        if let Some(some) = self.value.r#amount_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amountType", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#amount_text.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("amountText", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_amountText", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#amount_text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amountText", ctx))?;
        }
        if let Some(some) = self.value.r#reference_range.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceRange", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceAmount>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceAmount>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SubstanceAmount> {
    type Value = SubstanceAmount;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceAmount>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceAmount;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceAmount")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceAmount, V::Error>
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
                    #[serde(rename = "amountQuantity")]
                    AmountQuantity,
                    #[serde(rename = "amountRange")]
                    AmountRange,
                    #[serde(rename = "amountString")]
                    AmountString,
                    #[serde(rename = "_amountString")]
                    AmountStringPrimitiveElement,
                    #[serde(rename = "amountType")]
                    AmountType,
                    #[serde(rename = "amountText")]
                    AmountText,
                    #[serde(rename = "_amountText")]
                    AmountTextPrimitiveElement,
                    #[serde(rename = "referenceRange")]
                    ReferenceRange,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "amountQuantity",
                            "amountRange",
                            "amountString",
                            "amountType",
                            "amountText",
                            "referenceRange",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#amount: Option<fhirbolt_model::r4::types::SubstanceAmountAmount> = None;
                let mut r#amount_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#amount_text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#reference_range: Option<
                    fhirbolt_model::r4::types::SubstanceAmountReferenceRange,
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
                        Field::AmountQuantity => {
                            use fhirbolt_model::r4::types::SubstanceAmountAmount as _Enum;
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#amount =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::AmountRange => {
                            use fhirbolt_model::r4::types::SubstanceAmountAmount as _Enum;
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#amount =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::AmountString => {
                            use fhirbolt_model::r4::types::SubstanceAmountAmount as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#amount.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "amountString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("amount[x]"));
                                }
                            } else {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4::types::String>,
                                > = self.0.transmute();
                                r#amount = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::AmountStringPrimitiveElement => {
                            use fhirbolt_model::r4::types::SubstanceAmountAmount as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#amount.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_amountString",
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
                                    return Err(serde::de::Error::duplicate_field("_amount[x]"));
                                }
                            } else {
                                return unknown_field_error("amountString");
                            }
                        }
                        Field::AmountType => {
                            if r#amount_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#amount_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AmountText => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#amount_text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountText"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#amount_text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amountText"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#amount_text = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AmountTextPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#amount_text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amountText"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("amountText");
                            }
                        }
                        Field::ReferenceRange => {
                            if r#reference_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceRange"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::types::SubstanceAmountReferenceRange,
                            > = self.0.transmute();
                            r#reference_range = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SubstanceAmount {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#amount,
                    r#amount_type,
                    r#amount_text,
                    r#reference_range,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SubstanceAmount>> {
    type Value = Box<SubstanceAmount>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceAmount>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SubstanceAmount>> {
    type Value = Vec<SubstanceAmount>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceAmount>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceAmount>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceAmount> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
