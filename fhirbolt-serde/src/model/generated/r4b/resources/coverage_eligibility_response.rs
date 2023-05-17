// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit;
impl serde::ser::Serialize
    for SerializationContext<&CoverageEligibilityResponseInsuranceItemBenefit>
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
                "CoverageEligibilityResponse.insurance.item.benefit", field
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
        {
            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
            if let Some(some) = self.value.r#allowed.as_ref() {
                match some {
                    _Enum::UnsignedInt(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("allowedUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_allowedUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("allowedUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("allowedString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_allowedString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("allowedString", ctx)
                            })?;
                        }
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("allowedMoney", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("allowed is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
            if let Some(some) = self.value.r#used.as_ref() {
                match some {
                    _Enum::UnsignedInt(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("usedUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_usedUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("usedUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("usedString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_usedString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("usedString", ctx)
                            })?;
                        }
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("usedMoney", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("used is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<CoverageEligibilityResponseInsuranceItemBenefit>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<CoverageEligibilityResponseInsuranceItemBenefit>>
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
    for &mut DeserializationContext<CoverageEligibilityResponseInsuranceItemBenefit>
{
    type Value = CoverageEligibilityResponseInsuranceItemBenefit;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<CoverageEligibilityResponseInsuranceItemBenefit>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = CoverageEligibilityResponseInsuranceItemBenefit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItemBenefit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsuranceItemBenefit, V::Error>
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
                    #[serde(rename = "allowedUnsignedInt")]
                    AllowedUnsignedInt,
                    #[serde(rename = "_allowedUnsignedInt")]
                    AllowedUnsignedIntPrimitiveElement,
                    #[serde(rename = "allowedString")]
                    AllowedString,
                    #[serde(rename = "_allowedString")]
                    AllowedStringPrimitiveElement,
                    #[serde(rename = "allowedMoney")]
                    AllowedMoney,
                    #[serde(rename = "usedUnsignedInt")]
                    UsedUnsignedInt,
                    #[serde(rename = "_usedUnsignedInt")]
                    UsedUnsignedIntPrimitiveElement,
                    #[serde(rename = "usedString")]
                    UsedString,
                    #[serde(rename = "_usedString")]
                    UsedStringPrimitiveElement,
                    #[serde(rename = "usedMoney")]
                    UsedMoney,
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
                            "allowedUnsignedInt",
                            "allowedString",
                            "allowedMoney",
                            "usedUnsignedInt",
                            "usedString",
                            "usedMoney",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#allowed : Option < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed > = None ;
                let mut r#used : Option < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed > = None ;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AllowedUnsignedInt => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#allowed.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allowedUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("allowed[x]"));
                                }
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allowedUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#allowed = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::AllowedUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#allowed.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_allowedUnsignedInt",
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
                                    return Err(serde::de::Error::duplicate_field("_allowed[x]"));
                                }
                            } else {
                                return unknown_field_error("allowedUnsignedInt");
                            }
                        }
                        Field::AllowedString => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#allowed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allowedString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("allowed[x]"));
                                }
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowedString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#allowed = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::AllowedStringPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#allowed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_allowedString",
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
                                    return Err(serde::de::Error::duplicate_field("_allowed[x]"));
                                }
                            } else {
                                return unknown_field_error("allowedString");
                            }
                        }
                        Field::AllowedMoney => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitAllowed as _Enum;
                            if r#allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#allowed =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::UsedUnsignedInt => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#used.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "usedUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("used[x]"));
                                }
                            } else {
                                if r#used.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "usedUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#used = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::UsedUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#used.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_usedUnsignedInt",
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
                                    return Err(serde::de::Error::duplicate_field("_used[x]"));
                                }
                            } else {
                                return unknown_field_error("usedUnsignedInt");
                            }
                        }
                        Field::UsedString => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#used.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "usedString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("used[x]"));
                                }
                            } else {
                                if r#used.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usedString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#used = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::UsedStringPrimitiveElement => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#used.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_usedString",
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
                                    return Err(serde::de::Error::duplicate_field("_used[x]"));
                                }
                            } else {
                                return unknown_field_error("usedString");
                            }
                        }
                        Field::UsedMoney => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefitUsed as _Enum;
                            if r#used.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#used =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(CoverageEligibilityResponseInsuranceItemBenefit {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#allowed,
                    r#used,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<CoverageEligibilityResponseInsuranceItemBenefit>>
{
    type Value = Box<CoverageEligibilityResponseInsuranceItemBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<CoverageEligibilityResponseInsuranceItemBenefit>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<CoverageEligibilityResponseInsuranceItemBenefit>>
{
    type Value = Vec<CoverageEligibilityResponseInsuranceItemBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<CoverageEligibilityResponseInsuranceItemBenefit>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<CoverageEligibilityResponseInsuranceItemBenefit>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    CoverageEligibilityResponseInsuranceItemBenefit,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem;
impl serde::ser::Serialize for SerializationContext<&CoverageEligibilityResponseInsuranceItem> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "CoverageEligibilityResponse.insurance.item", field
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
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#provider.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("provider", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#excluded.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("excluded", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_excluded", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#excluded.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("excluded", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("name", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if let Some(some) = self.value.r#network.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("network", ctx))?;
        }
        if let Some(some) = self.value.r#unit.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unit", ctx))?;
        }
        if let Some(some) = self.value.r#term.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("term", ctx))?;
        }
        if !self.value.r#benefit.is_empty() {
            self.with_context(&self.value.r#benefit, |ctx| {
                state.serialize_entry("benefit", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#authorization_required.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("authorizationRequired", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_authorizationRequired", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#authorization_required.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("authorizationRequired", ctx)
            })?;
        }
        if !self.value.r#authorization_supporting.is_empty() {
            self.with_context(&self.value.r#authorization_supporting, |ctx| {
                state.serialize_entry("authorizationSupporting", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#authorization_url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("authorizationUrl", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_authorizationUrl", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#authorization_url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("authorizationUrl", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<CoverageEligibilityResponseInsuranceItem>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<CoverageEligibilityResponseInsuranceItem>>
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
    for &mut DeserializationContext<CoverageEligibilityResponseInsuranceItem>
{
    type Value = CoverageEligibilityResponseInsuranceItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<CoverageEligibilityResponseInsuranceItem>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = CoverageEligibilityResponseInsuranceItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsuranceItem, V::Error>
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
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "excluded")]
                    Excluded,
                    #[serde(rename = "_excluded")]
                    ExcludedPrimitiveElement,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "network")]
                    Network,
                    #[serde(rename = "unit")]
                    Unit,
                    #[serde(rename = "term")]
                    Term,
                    #[serde(rename = "benefit")]
                    Benefit,
                    #[serde(rename = "authorizationRequired")]
                    AuthorizationRequired,
                    #[serde(rename = "_authorizationRequired")]
                    AuthorizationRequiredPrimitiveElement,
                    #[serde(rename = "authorizationSupporting")]
                    AuthorizationSupporting,
                    #[serde(rename = "authorizationUrl")]
                    AuthorizationUrl,
                    #[serde(rename = "_authorizationUrl")]
                    AuthorizationUrlPrimitiveElement,
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
                            "productOrService",
                            "modifier",
                            "provider",
                            "excluded",
                            "name",
                            "description",
                            "network",
                            "unit",
                            "term",
                            "benefit",
                            "authorizationRequired",
                            "authorizationSupporting",
                            "authorizationUrl",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#provider: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#excluded: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#network: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#term: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#benefit : Option < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > > = None ;
                let mut r#authorization_required: Option<fhirbolt_model::r4b::types::Boolean> =
                    None;
                let mut r#authorization_supporting: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#authorization_url: Option<fhirbolt_model::r4b::types::Uri> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
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
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Provider => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#provider = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Excluded => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("excluded"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#excluded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("excluded"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#excluded = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExcludedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_excluded"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("excluded");
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Network => {
                            if r#network.is_some() {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#network = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unit => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#unit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Term => {
                            if r#term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#term = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Benefit => {
                            if self.0.from == crate::context::Format::Json {
                                if r#benefit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("benefit"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit >> = self . 0 . transmute () ;
                                r#benefit = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#benefit.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AuthorizationRequired => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationRequired",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#authorization_required.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationRequired",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#authorization_required =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AuthorizationRequiredPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationRequired",
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
                                return unknown_field_error("authorizationRequired");
                            }
                        }
                        Field::AuthorizationSupporting => {
                            if self.0.from == crate::context::Format::Json {
                                if r#authorization_supporting.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationSupporting",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#authorization_supporting =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#authorization_supporting.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AuthorizationUrl => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationUrl",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#authorization_url.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationUrl",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#authorization_url =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AuthorizationUrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationUrl",
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
                                return unknown_field_error("authorizationUrl");
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
                Ok(CoverageEligibilityResponseInsuranceItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category,
                    r#product_or_service,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#provider,
                    r#excluded,
                    r#name,
                    r#description,
                    r#network,
                    r#unit,
                    r#term,
                    r#benefit: r#benefit.unwrap_or(vec![]),
                    r#authorization_required,
                    r#authorization_supporting: r#authorization_supporting.unwrap_or(vec![]),
                    r#authorization_url,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<CoverageEligibilityResponseInsuranceItem>>
{
    type Value = Box<CoverageEligibilityResponseInsuranceItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<CoverageEligibilityResponseInsuranceItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<CoverageEligibilityResponseInsuranceItem>>
{
    type Value = Vec<CoverageEligibilityResponseInsuranceItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<CoverageEligibilityResponseInsuranceItem>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<CoverageEligibilityResponseInsuranceItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    CoverageEligibilityResponseInsuranceItem,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance;
impl serde::ser::Serialize for SerializationContext<&CoverageEligibilityResponseInsurance> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "CoverageEligibilityResponse.insurance", field
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
        if self.value.r#coverage.id.as_deref() == Some("$invalid") {
            return missing_field_error("coverage");
        } else {
            self.with_context(&self.value.r#coverage, |ctx| {
                state.serialize_entry("coverage", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#inforce.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("inforce", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_inforce", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#inforce.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("inforce", ctx))?;
        }
        if let Some(some) = self.value.r#benefit_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("benefitPeriod", ctx))?;
        }
        if !self.value.r#item.is_empty() {
            self.with_context(&self.value.r#item, |ctx| state.serialize_entry("item", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<CoverageEligibilityResponseInsurance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<CoverageEligibilityResponseInsurance>> {
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
    for &mut DeserializationContext<CoverageEligibilityResponseInsurance>
{
    type Value = CoverageEligibilityResponseInsurance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<CoverageEligibilityResponseInsurance>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = CoverageEligibilityResponseInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsurance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsurance, V::Error>
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
                    #[serde(rename = "coverage")]
                    Coverage,
                    #[serde(rename = "inforce")]
                    Inforce,
                    #[serde(rename = "_inforce")]
                    InforcePrimitiveElement,
                    #[serde(rename = "benefitPeriod")]
                    BenefitPeriod,
                    #[serde(rename = "item")]
                    Item,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "coverage",
                            "inforce",
                            "benefitPeriod",
                            "item",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#coverage: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#inforce: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#benefit_period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#item: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>,
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
                        Field::Coverage => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#coverage = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Inforce => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inforce"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#inforce.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inforce"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#inforce = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::InforcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_inforce"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("inforce");
                            }
                        }
                        Field::BenefitPeriod => {
                            if r#benefit_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#benefit_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Item => {
                            if self.0.from == crate::context::Format::Json {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItem >> = self . 0 . transmute () ;
                                r#item = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#item.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItem > = self . 0 . transmute () ;
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
                Ok(CoverageEligibilityResponseInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#coverage: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#coverage.unwrap_or(Default::default())
                    } else {
                        r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                    },
                    r#inforce,
                    r#benefit_period,
                    r#item: r#item.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<CoverageEligibilityResponseInsurance>>
{
    type Value = Box<CoverageEligibilityResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<CoverageEligibilityResponseInsurance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<CoverageEligibilityResponseInsurance>>
{
    type Value = Vec<CoverageEligibilityResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<CoverageEligibilityResponseInsurance>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<CoverageEligibilityResponseInsurance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<CoverageEligibilityResponseInsurance> =
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
use fhirbolt_model::r4b::resources::CoverageEligibilityResponseError;
impl serde::ser::Serialize for SerializationContext<&CoverageEligibilityResponseError> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "CoverageEligibilityResponse.error", field
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
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<CoverageEligibilityResponseError>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<CoverageEligibilityResponseError>> {
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
    for &mut DeserializationContext<CoverageEligibilityResponseError>
{
    type Value = CoverageEligibilityResponseError;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<CoverageEligibilityResponseError>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = CoverageEligibilityResponseError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseError")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseError, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
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
                Ok(CoverageEligibilityResponseError {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<CoverageEligibilityResponseError>>
{
    type Value = Box<CoverageEligibilityResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<CoverageEligibilityResponseError>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<CoverageEligibilityResponseError>>
{
    type Value = Vec<CoverageEligibilityResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<CoverageEligibilityResponseError>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<CoverageEligibilityResponseError>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<CoverageEligibilityResponseError> =
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
use fhirbolt_model::r4b::resources::CoverageEligibilityResponse;
impl crate::Resource for CoverageEligibilityResponse {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for SerializationContext<&CoverageEligibilityResponse> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "CoverageEligibilityResponse", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "CoverageEligibilityResponse")?;
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
        if self.output == crate::context::Format::Json {
            if !self.value.r#purpose.is_empty() {
                let values = self
                    .value
                    .r#purpose
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("purpose", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#purpose
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#purpose
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
                        state.serialize_entry("_purpose", ctx)
                    })?;
                }
            }
        } else if !self.value.r#purpose.is_empty() {
            self.with_context(&self.value.r#purpose, |ctx| {
                state.serialize_entry("purpose", ctx)
            })?;
        }
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        } else {
            self.with_context(&self.value.r#patient, |ctx| {
                state.serialize_entry("patient", ctx)
            })?;
        }
        {
            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced as _Enum;
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
        if let Some(some) = self.value.r#requestor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requestor", ctx))?;
        }
        if self.value.r#request.id.as_deref() == Some("$invalid") {
            return missing_field_error("request");
        } else {
            self.with_context(&self.value.r#request, |ctx| {
                state.serialize_entry("request", ctx)
            })?;
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
        if self.value.r#insurer.id.as_deref() == Some("$invalid") {
            return missing_field_error("insurer");
        } else {
            self.with_context(&self.value.r#insurer, |ctx| {
                state.serialize_entry("insurer", ctx)
            })?;
        }
        if !self.value.r#insurance.is_empty() {
            self.with_context(&self.value.r#insurance, |ctx| {
                state.serialize_entry("insurance", ctx)
            })?;
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
        if let Some(some) = self.value.r#form.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("form", ctx))?;
        }
        if !self.value.r#error.is_empty() {
            self.with_context(&self.value.r#error, |ctx| {
                state.serialize_entry("error", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<CoverageEligibilityResponse>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<CoverageEligibilityResponse>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<CoverageEligibilityResponse> {
    type Value = CoverageEligibilityResponse;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<CoverageEligibilityResponse>
{
    type Value = CoverageEligibilityResponse;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<CoverageEligibilityResponse>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = CoverageEligibilityResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponse")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponse, V::Error>
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
                    #[serde(rename = "purpose")]
                    Purpose,
                    #[serde(rename = "_purpose")]
                    PurposePrimitiveElement,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "servicedDate")]
                    ServicedDate,
                    #[serde(rename = "_servicedDate")]
                    ServicedDatePrimitiveElement,
                    #[serde(rename = "servicedPeriod")]
                    ServicedPeriod,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
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
                    #[serde(rename = "insurer")]
                    Insurer,
                    #[serde(rename = "insurance")]
                    Insurance,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    #[serde(rename = "form")]
                    Form,
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
                            "purpose",
                            "patient",
                            "servicedDate",
                            "servicedPeriod",
                            "created",
                            "requestor",
                            "request",
                            "outcome",
                            "disposition",
                            "insurer",
                            "insurance",
                            "preAuthRef",
                            "form",
                            "error",
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
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#purpose: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#serviced: Option<
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced,
                > = None;
                let mut r#created: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#requestor: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#request: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#outcome: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#disposition: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#insurer: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#insurance: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>,
                > = None;
                let mut r#pre_auth_ref: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#form: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#error: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "CoverageEligibilityResponse" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"CoverageEligibilityResponse",
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
                                    fhirbolt_model::r4b::types::Code,
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
                        Field::Purpose => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#purpose.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#purpose.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PurposePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#purpose.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ServicedDate => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced as _Enum;
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
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#serviced =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced as _Enum;
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
                            use fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced as _Enum;
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#serviced =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
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
                                    fhirbolt_model::r4b::types::DateTime,
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
                        Field::Requestor => {
                            if r#requestor.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#requestor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
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
                                    fhirbolt_model::r4b::types::Code,
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
                                    fhirbolt_model::r4b::types::String,
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
                        Field::Insurer => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#insurer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Insurance => {
                            if self.0.from == crate::context::Format::Json {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsurance >> = self . 0 . transmute () ;
                                r#insurance = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#insurance.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsurance > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    fhirbolt_model::r4b::types::String,
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
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#form = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Error => {
                            if self.0.from == crate::context::Format::Json {
                                if r#error.is_some() {
                                    return Err(serde::de::Error::duplicate_field("error"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError >> = self . 0 . transmute () ;
                                r#error = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#error.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError > = self . 0 . transmute () ;
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
                Ok(CoverageEligibilityResponse {
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
                    r#purpose: r#purpose.unwrap_or(vec![]),
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                    },
                    r#serviced,
                    r#created: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#created.unwrap_or(Default::default())
                    } else {
                        r#created.ok_or(serde::de::Error::missing_field("created"))?
                    },
                    r#requestor,
                    r#request: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#request.unwrap_or(Default::default())
                    } else {
                        r#request.ok_or(serde::de::Error::missing_field("request"))?
                    },
                    r#outcome: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#outcome.unwrap_or(Default::default())
                    } else {
                        r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                    },
                    r#disposition,
                    r#insurer: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#insurer.unwrap_or(Default::default())
                    } else {
                        r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?
                    },
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#pre_auth_ref,
                    r#form,
                    r#error: r#error.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<CoverageEligibilityResponse>>
{
    type Value = Box<CoverageEligibilityResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<CoverageEligibilityResponse>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<CoverageEligibilityResponse>>
{
    type Value = Vec<CoverageEligibilityResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<CoverageEligibilityResponse>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<CoverageEligibilityResponse>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<CoverageEligibilityResponse> =
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
