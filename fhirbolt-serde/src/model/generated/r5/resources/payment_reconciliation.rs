// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::PaymentReconciliationAllocation;
impl serde::ser::Serialize for SerializationContext<&PaymentReconciliationAllocation> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "PaymentReconciliation.allocation", field
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
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        if let Some(some) = self.value.r#predecessor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("predecessor", ctx))?;
        }
        if let Some(some) = self.value.r#target.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("target", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
            if let Some(some) = self.value.r#target_item.as_ref() {
                match some {
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("targetItemString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_targetItemString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("targetItemString", ctx)
                            })?;
                        }
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("targetItemIdentifier", ctx)
                        })?;
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("targetItemPositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_targetItemPositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("targetItemPositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("target_item is invalid"))
                    }
                }
            }
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if let Some(some) = self.value.r#account.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("account", ctx))?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#submitter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("submitter", ctx))?;
        }
        if let Some(some) = self.value.r#response.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("response", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#responsible.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responsible", ctx))?;
        }
        if let Some(some) = self.value.r#payee.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("payee", ctx))?;
        }
        if let Some(some) = self.value.r#amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amount", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<PaymentReconciliationAllocation>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<PaymentReconciliationAllocation>> {
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
    for &mut DeserializationContext<PaymentReconciliationAllocation>
{
    type Value = PaymentReconciliationAllocation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<PaymentReconciliationAllocation>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = PaymentReconciliationAllocation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliationAllocation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PaymentReconciliationAllocation, V::Error>
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
                    #[serde(rename = "predecessor")]
                    Predecessor,
                    #[serde(rename = "target")]
                    Target,
                    #[serde(rename = "targetItemString")]
                    TargetItemString,
                    #[serde(rename = "_targetItemString")]
                    TargetItemStringPrimitiveElement,
                    #[serde(rename = "targetItemIdentifier")]
                    TargetItemIdentifier,
                    #[serde(rename = "targetItemPositiveInt")]
                    TargetItemPositiveInt,
                    #[serde(rename = "_targetItemPositiveInt")]
                    TargetItemPositiveIntPrimitiveElement,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "account")]
                    Account,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "submitter")]
                    Submitter,
                    #[serde(rename = "response")]
                    Response,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "responsible")]
                    Responsible,
                    #[serde(rename = "payee")]
                    Payee,
                    #[serde(rename = "amount")]
                    Amount,
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
                            "predecessor",
                            "target",
                            "targetItemString",
                            "targetItemIdentifier",
                            "targetItemPositiveInt",
                            "encounter",
                            "account",
                            "type",
                            "submitter",
                            "response",
                            "date",
                            "responsible",
                            "payee",
                            "amount",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#predecessor: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#target: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#target_item: Option<
                    fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem,
                > = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#account: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#submitter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#response: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#responsible: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#payee: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Predecessor => {
                            if r#predecessor.is_some() {
                                return Err(serde::de::Error::duplicate_field("predecessor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#predecessor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Target => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#target = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TargetItemString => {
                            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#target_item.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "targetItemString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("targetItem[x]"));
                                }
                            } else {
                                if r#target_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetItemString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::String>,
                                > = self.0.transmute();
                                r#target_item = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::TargetItemStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#target_item.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_targetItemString",
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_targetItem[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("targetItemString");
                            }
                        }
                        Field::TargetItemIdentifier => {
                            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
                            if r#target_item.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "targetItemIdentifier",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#target_item = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::TargetItemPositiveInt => {
                            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
                            if self.0.from_json {
                                let r#enum = r#target_item
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "targetItemPositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("targetItem[x]"));
                                }
                            } else {
                                if r#target_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetItemPositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::PositiveInt>,
                                > = self.0.transmute();
                                r#target_item = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::TargetItemPositiveIntPrimitiveElement => {
                            use fhirbolt_model::r5::resources::PaymentReconciliationAllocationTargetItem as _Enum;
                            if self.0.from_json {
                                let r#enum = r#target_item
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_targetItemPositiveInt",
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_targetItem[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("targetItemPositiveInt");
                            }
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Account => {
                            if r#account.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#account = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Submitter => {
                            if r#submitter.is_some() {
                                return Err(serde::de::Error::duplicate_field("submitter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#submitter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Response => {
                            if r#response.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#response = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Date => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DatePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Responsible => {
                            if r#responsible.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsible"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#responsible = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Payee => {
                            if r#payee.is_some() {
                                return Err(serde::de::Error::duplicate_field("payee"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#payee = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
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
                Ok(PaymentReconciliationAllocation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#predecessor,
                    r#target,
                    r#target_item,
                    r#encounter,
                    r#account,
                    r#type,
                    r#submitter,
                    r#response,
                    r#date,
                    r#responsible,
                    r#payee,
                    r#amount,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<PaymentReconciliationAllocation>>
{
    type Value = Box<PaymentReconciliationAllocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<PaymentReconciliationAllocation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<PaymentReconciliationAllocation>>
{
    type Value = Vec<PaymentReconciliationAllocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<PaymentReconciliationAllocation>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<PaymentReconciliationAllocation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<PaymentReconciliationAllocation> =
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
use fhirbolt_model::r5::resources::PaymentReconciliationProcessNote;
impl serde::ser::Serialize for SerializationContext<&PaymentReconciliationProcessNote> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "PaymentReconciliation.processNote", field
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
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("text", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<PaymentReconciliationProcessNote>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<PaymentReconciliationProcessNote>> {
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
    for &mut DeserializationContext<PaymentReconciliationProcessNote>
{
    type Value = PaymentReconciliationProcessNote;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<PaymentReconciliationProcessNote>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = PaymentReconciliationProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliationProcessNote")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PaymentReconciliationProcessNote, V::Error>
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
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "text"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<fhirbolt_model::r5::types::String> = None;
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#text = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(PaymentReconciliationProcessNote {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#text,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<PaymentReconciliationProcessNote>>
{
    type Value = Box<PaymentReconciliationProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<PaymentReconciliationProcessNote>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<PaymentReconciliationProcessNote>>
{
    type Value = Vec<PaymentReconciliationProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<PaymentReconciliationProcessNote>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<PaymentReconciliationProcessNote>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<PaymentReconciliationProcessNote> =
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
use fhirbolt_model::r5::resources::PaymentReconciliation;
impl crate::Resource for PaymentReconciliation {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&PaymentReconciliation> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "PaymentReconciliation", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "PaymentReconciliation")?;
        if self.output_json {
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
        if self.output_json {
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
        if self.output_json {
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#kind.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("kind", ctx))?;
        }
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#enterer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("enterer", ctx))?;
        }
        if let Some(some) = self.value.r#issuer_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("issuerType", ctx))?;
        }
        if let Some(some) = self.value.r#payment_issuer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("paymentIssuer", ctx))?;
        }
        if let Some(some) = self.value.r#request.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("request", ctx))?;
        }
        if let Some(some) = self.value.r#requestor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestor", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#outcome.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("outcome", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_outcome", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("outcome", ctx))?;
        }
        if self.output_json {
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
        if self.output_json {
            if self.value.r#date.id.as_deref() == Some("$invalid") {
                return missing_field_error("date");
            }
            if let Some(some) = self.value.r#date.value.as_ref().map(Ok) {
                state.serialize_entry("date", &some?)?;
            }
            if self.value.r#date.id.is_some() || !self.value.r#date.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#date.id.as_ref(),
                    extension: &self.value.r#date.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_date", ctx)
                })?;
            }
        } else if self.value.r#date.id.as_deref() == Some("$invalid") {
            return missing_field_error("date");
        } else {
            self.with_context(&self.value.r#date, |ctx| state.serialize_entry("date", ctx))?;
        }
        if let Some(some) = self.value.r#location.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("location", ctx))?;
        }
        if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#card_brand.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("cardBrand", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_cardBrand", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#card_brand.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("cardBrand", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#account_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("accountNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_accountNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#account_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("accountNumber", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#expiration_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("expirationDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expirationDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#expiration_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expirationDate", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#processor.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("processor", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_processor", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#processor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("processor", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#reference_number.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("referenceNumber", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_referenceNumber", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#reference_number.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceNumber", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#authorization.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("authorization", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_authorization", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#authorization.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("authorization", ctx))?;
        }
        if let Some(some) = self.value.r#tendered_amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tenderedAmount", ctx))?;
        }
        if let Some(some) = self.value.r#returned_amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("returnedAmount", ctx))?;
        }
        if self.value.r#amount.id.as_deref() == Some("$invalid") {
            return missing_field_error("amount");
        } else {
            self.with_context(&self.value.r#amount, |ctx| {
                state.serialize_entry("amount", ctx)
            })?;
        }
        if let Some(some) = self.value.r#payment_identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("paymentIdentifier", ctx))?;
        }
        if !self.value.r#allocation.is_empty() {
            self.with_context(&self.value.r#allocation, |ctx| {
                state.serialize_entry("allocation", ctx)
            })?;
        }
        if let Some(some) = self.value.r#form_code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("formCode", ctx))?;
        }
        if !self.value.r#process_note.is_empty() {
            self.with_context(&self.value.r#process_note, |ctx| {
                state.serialize_entry("processNote", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<PaymentReconciliation>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<PaymentReconciliation>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<PaymentReconciliation> {
    type Value = PaymentReconciliation;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<PaymentReconciliation> {
    type Value = PaymentReconciliation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<PaymentReconciliation>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = PaymentReconciliation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PaymentReconciliation, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "kind")]
                    Kind,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "enterer")]
                    Enterer,
                    #[serde(rename = "issuerType")]
                    IssuerType,
                    #[serde(rename = "paymentIssuer")]
                    PaymentIssuer,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "requestor")]
                    Requestor,
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "_outcome")]
                    OutcomePrimitiveElement,
                    #[serde(rename = "disposition")]
                    Disposition,
                    #[serde(rename = "_disposition")]
                    DispositionPrimitiveElement,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "cardBrand")]
                    CardBrand,
                    #[serde(rename = "_cardBrand")]
                    CardBrandPrimitiveElement,
                    #[serde(rename = "accountNumber")]
                    AccountNumber,
                    #[serde(rename = "_accountNumber")]
                    AccountNumberPrimitiveElement,
                    #[serde(rename = "expirationDate")]
                    ExpirationDate,
                    #[serde(rename = "_expirationDate")]
                    ExpirationDatePrimitiveElement,
                    #[serde(rename = "processor")]
                    Processor,
                    #[serde(rename = "_processor")]
                    ProcessorPrimitiveElement,
                    #[serde(rename = "referenceNumber")]
                    ReferenceNumber,
                    #[serde(rename = "_referenceNumber")]
                    ReferenceNumberPrimitiveElement,
                    #[serde(rename = "authorization")]
                    Authorization,
                    #[serde(rename = "_authorization")]
                    AuthorizationPrimitiveElement,
                    #[serde(rename = "tenderedAmount")]
                    TenderedAmount,
                    #[serde(rename = "returnedAmount")]
                    ReturnedAmount,
                    #[serde(rename = "amount")]
                    Amount,
                    #[serde(rename = "paymentIdentifier")]
                    PaymentIdentifier,
                    #[serde(rename = "allocation")]
                    Allocation,
                    #[serde(rename = "formCode")]
                    FormCode,
                    #[serde(rename = "processNote")]
                    ProcessNote,
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
                            "type",
                            "status",
                            "kind",
                            "period",
                            "created",
                            "enterer",
                            "issuerType",
                            "paymentIssuer",
                            "request",
                            "requestor",
                            "outcome",
                            "disposition",
                            "date",
                            "location",
                            "method",
                            "cardBrand",
                            "accountNumber",
                            "expirationDate",
                            "processor",
                            "referenceNumber",
                            "authorization",
                            "tenderedAmount",
                            "returnedAmount",
                            "amount",
                            "paymentIdentifier",
                            "allocation",
                            "formCode",
                            "processNote",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#kind: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#created: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#enterer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#issuer_type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#payment_issuer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#request: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#requestor: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#outcome: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#disposition: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#card_brand: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#account_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#expiration_date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#processor: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#reference_number: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#authorization: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#tendered_amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#returned_amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#payment_identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> =
                    None;
                let mut r#allocation: Option<
                    Vec<fhirbolt_model::r5::resources::PaymentReconciliationAllocation>,
                > = None;
                let mut r#form_code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#process_note: Option<
                    Vec<fhirbolt_model::r5::resources::PaymentReconciliationProcessNote>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "PaymentReconciliation" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"PaymentReconciliation",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Status => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Kind => {
                            if r#kind.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#kind = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Created => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#created = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CreatedPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Enterer => {
                            if r#enterer.is_some() {
                                return Err(serde::de::Error::duplicate_field("enterer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#enterer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::IssuerType => {
                            if r#issuer_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuerType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#issuer_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PaymentIssuer => {
                            if r#payment_issuer.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentIssuer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#payment_issuer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#request = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Requestor => {
                            if r#requestor.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#requestor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Outcome => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#outcome = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OutcomePrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#disposition = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DispositionPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Date => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DatePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::CardBrand => {
                            if self.0.from_json {
                                let some = r#card_brand.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cardBrand"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#card_brand.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cardBrand"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#card_brand = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CardBrandPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#card_brand.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_cardBrand"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("cardBrand");
                            }
                        }
                        Field::AccountNumber => {
                            if self.0.from_json {
                                let some = r#account_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accountNumber"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#account_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accountNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#account_number =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AccountNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#account_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_accountNumber",
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
                                return unknown_field_error("accountNumber");
                            }
                        }
                        Field::ExpirationDate => {
                            if self.0.from_json {
                                let some = r#expiration_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "expirationDate",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#expiration_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "expirationDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#expiration_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExpirationDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#expiration_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_expirationDate",
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
                                return unknown_field_error("expirationDate");
                            }
                        }
                        Field::Processor => {
                            if self.0.from_json {
                                let some = r#processor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processor"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#processor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#processor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProcessorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#processor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_processor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("processor");
                            }
                        }
                        Field::ReferenceNumber => {
                            if self.0.from_json {
                                let some = r#reference_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceNumber",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#reference_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceNumber",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#reference_number =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReferenceNumberPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#reference_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_referenceNumber",
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
                                return unknown_field_error("referenceNumber");
                            }
                        }
                        Field::Authorization => {
                            if self.0.from_json {
                                let some = r#authorization.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authorization"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#authorization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authorization"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#authorization = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AuthorizationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#authorization.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorization",
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
                                return unknown_field_error("authorization");
                            }
                        }
                        Field::TenderedAmount => {
                            if r#tendered_amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("tenderedAmount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tendered_amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ReturnedAmount => {
                            if r#returned_amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("returnedAmount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#returned_amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#amount = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PaymentIdentifier => {
                            if r#payment_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#payment_identifier =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Allocation => {
                            if self.0.from_json {
                                if r#allocation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allocation"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: PaymentReconciliationAllocation >> = self . 0 . transmute () ;
                                r#allocation = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#allocation.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::PaymentReconciliationAllocation,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FormCode => {
                            if r#form_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("formCode"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#form_code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProcessNote => {
                            if self.0.from_json {
                                if r#process_note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processNote"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: PaymentReconciliationProcessNote >> = self . 0 . transmute () ;
                                r#process_note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#process_note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::PaymentReconciliationProcessNote,
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
                Ok(PaymentReconciliation {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#kind,
                    r#period,
                    r#created: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#created.unwrap_or(Default::default())
                    } else {
                        r#created.ok_or(serde::de::Error::missing_field("created"))?
                    },
                    r#enterer,
                    r#issuer_type,
                    r#payment_issuer,
                    r#request,
                    r#requestor,
                    r#outcome,
                    r#disposition,
                    r#date: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#date.unwrap_or(Default::default())
                    } else {
                        r#date.ok_or(serde::de::Error::missing_field("date"))?
                    },
                    r#location,
                    r#method,
                    r#card_brand,
                    r#account_number,
                    r#expiration_date,
                    r#processor,
                    r#reference_number,
                    r#authorization,
                    r#tendered_amount,
                    r#returned_amount,
                    r#amount: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#amount.unwrap_or(Default::default())
                    } else {
                        r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                    },
                    r#payment_identifier,
                    r#allocation: r#allocation.unwrap_or(vec![]),
                    r#form_code,
                    r#process_note: r#process_note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<PaymentReconciliation>>
{
    type Value = Box<PaymentReconciliation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<PaymentReconciliation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<PaymentReconciliation>>
{
    type Value = Vec<PaymentReconciliation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<PaymentReconciliation>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<PaymentReconciliation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<PaymentReconciliation> =
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
