// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::ExplanationOfBenefitRelated;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitRelated> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.related", field
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
        if let Some(some) = self.value.r#claim.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("claim", ctx))?;
        }
        if let Some(some) = self.value.r#relationship.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("relationship", ctx))?;
        }
        if let Some(some) = self.value.r#reference.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reference", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitRelated>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitRelated>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitRelated>
{
    type Value = ExplanationOfBenefitRelated;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitRelated>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitRelated;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitRelated")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitRelated, V::Error>
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
                    #[serde(rename = "claim")]
                    Claim,
                    #[serde(rename = "relationship")]
                    Relationship,
                    #[serde(rename = "reference")]
                    Reference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "claim",
                            "relationship",
                            "reference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#claim: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#relationship: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#reference: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
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
                        Field::Claim => {
                            if r#claim.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#claim = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Relationship => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#relationship = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Reference => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#reference = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitRelated {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#claim,
                    r#relationship,
                    r#reference,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitRelated>>
{
    type Value = Box<ExplanationOfBenefitRelated>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitRelated>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitRelated>>
{
    type Value = Vec<ExplanationOfBenefitRelated>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitRelated>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitRelated>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitRelated> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitEvent;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitEvent> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.event", field
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
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitEventWhen as _Enum;
            match self.value.r#when {
                _Enum::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("whenDateTime", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_whenDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("whenDateTime", ctx))?;
                    }
                }
                _Enum::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("whenPeriod", ctx))?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("when is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitEvent>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitEvent>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitEvent>
{
    type Value = ExplanationOfBenefitEvent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitEvent>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitEvent, V::Error>
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
                    #[serde(rename = "whenDateTime")]
                    WhenDateTime,
                    #[serde(rename = "_whenDateTime")]
                    WhenDateTimePrimitiveElement,
                    #[serde(rename = "whenPeriod")]
                    WhenPeriod,
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
                            "whenDateTime",
                            "whenPeriod",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#when: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitEventWhen,
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::WhenDateTime => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitEventWhen as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#when.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "whenDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("when[x]"));
                                }
                            } else {
                                if r#when.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whenDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::DateTime>,
                                > = self.0.transmute();
                                r#when = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::WhenDateTimePrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitEventWhen as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#when.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_whenDateTime",
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
                                    return Err(serde::de::Error::duplicate_field("_when[x]"));
                                }
                            } else {
                                return unknown_field_error("whenDateTime");
                            }
                        }
                        Field::WhenPeriod => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitEventWhen as _Enum;
                            if r#when.is_some() {
                                return Err(serde::de::Error::duplicate_field("whenPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#when =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitEvent {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#when: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#when.unwrap_or(Default::default())
                    } else {
                        r#when.ok_or(serde::de::Error::missing_field("when[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitEvent>>
{
    type Value = Box<ExplanationOfBenefitEvent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitEvent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitEvent>>
{
    type Value = Vec<ExplanationOfBenefitEvent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitEvent>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitEvent>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitEvent> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitPayee;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitPayee> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.payee", field
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
        if let Some(some) = self.value.r#party.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("party", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitPayee>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitPayee>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitPayee>
{
    type Value = ExplanationOfBenefitPayee;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitPayee>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitPayee;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitPayee")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitPayee, V::Error>
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
                    #[serde(rename = "party")]
                    Party,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "party"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#party: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Party => {
                            if r#party.is_some() {
                                return Err(serde::de::Error::duplicate_field("party"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#party = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitPayee {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#party,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitPayee>>
{
    type Value = Box<ExplanationOfBenefitPayee>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitPayee>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitPayee>>
{
    type Value = Vec<ExplanationOfBenefitPayee>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitPayee>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitPayee>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitPayee> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitCareTeam;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitCareTeam> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.careTeam", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if self.value.r#provider.id.as_deref() == Some("$invalid") {
            return missing_field_error("provider");
        }
        self.with_context(&self.value.r#provider, |ctx| {
            state.serialize_entry("provider", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#responsible.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("responsible", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_responsible", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#responsible.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responsible", ctx))?;
        }
        if let Some(some) = self.value.r#role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
        }
        if let Some(some) = self.value.r#specialty.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("specialty", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitCareTeam>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitCareTeam>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitCareTeam>
{
    type Value = ExplanationOfBenefitCareTeam;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitCareTeam>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitCareTeam;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitCareTeam")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitCareTeam, V::Error>
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
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "responsible")]
                    Responsible,
                    #[serde(rename = "_responsible")]
                    ResponsiblePrimitiveElement,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "specialty")]
                    Specialty,
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
                            "provider",
                            "responsible",
                            "role",
                            "specialty",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#provider: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#responsible: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#role: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#specialty: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Provider => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#provider = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Responsible => {
                            if self.0.from_json {
                                let some = r#responsible.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#responsible.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#responsible = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ResponsiblePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#responsible.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_responsible"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("responsible");
                            }
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#role = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Specialty => {
                            if r#specialty.is_some() {
                                return Err(serde::de::Error::duplicate_field("specialty"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#specialty = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitCareTeam {
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
                    r#provider: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#provider.unwrap_or(Default::default())
                    } else {
                        r#provider.ok_or(serde::de::Error::missing_field("provider"))?
                    },
                    r#responsible,
                    r#role,
                    r#specialty,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitCareTeam>>
{
    type Value = Box<ExplanationOfBenefitCareTeam>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitCareTeam>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitCareTeam>>
{
    type Value = Vec<ExplanationOfBenefitCareTeam>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitCareTeam>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitCareTeam>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitCareTeam> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfo;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitSupportingInfo> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.supportingInfo", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if self.value.r#category.id.as_deref() == Some("$invalid") {
            return missing_field_error("category");
        }
        self.with_context(&self.value.r#category, |ctx| {
            state.serialize_entry("category", ctx)
        })?;
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoTiming as _Enum;
            if let Some(some) = self.value.r#timing.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("timingDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_timingDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("timingDate", ctx)
                            })?;
                        }
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("timingPeriod", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("timing is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
            if let Some(some) = self.value.r#value.as_ref() {
                match some {
                    _Enum::Boolean(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueBoolean", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueString", ctx)
                            })?;
                        }
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueQuantity", ctx)
                        })?;
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueAttachment", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueReference", ctx)
                        })?;
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueIdentifier", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("value is invalid")),
                }
            }
        }
        if let Some(some) = self.value.r#reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reason", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitSupportingInfo>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitSupportingInfo>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitSupportingInfo>
{
    type Value = ExplanationOfBenefitSupportingInfo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitSupportingInfo>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitSupportingInfo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitSupportingInfo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitSupportingInfo, V::Error>
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "timingDate")]
                    TimingDate,
                    #[serde(rename = "_timingDate")]
                    TimingDatePrimitiveElement,
                    #[serde(rename = "timingPeriod")]
                    TimingPeriod,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    #[serde(rename = "valueIdentifier")]
                    ValueIdentifier,
                    #[serde(rename = "reason")]
                    Reason,
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
                            "category",
                            "code",
                            "timingDate",
                            "timingPeriod",
                            "valueBoolean",
                            "valueString",
                            "valueQuantity",
                            "valueAttachment",
                            "valueReference",
                            "valueIdentifier",
                            "reason",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#timing: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoTiming,
                > = None;
                let mut r#value: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue,
                > = None;
                let mut r#reason: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TimingDate => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoTiming as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#timing.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            } else {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Date>,
                                > = self.0.transmute();
                                r#timing =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::TimingDatePrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoTiming as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#timing.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timingDate",
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
                                    return Err(serde::de::Error::duplicate_field("_timing[x]"));
                                }
                            } else {
                                return unknown_field_error("timingDate");
                            }
                        }
                        Field::TimingPeriod => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoTiming as _Enum;
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#timing =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueBoolean => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::Boolean>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
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
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueString => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::String>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
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
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueQuantity => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAttachment => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Attachment>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueIdentifier => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfoValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Reason => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#reason = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitSupportingInfo {
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
                    r#category: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#category.unwrap_or(Default::default())
                    } else {
                        r#category.ok_or(serde::de::Error::missing_field("category"))?
                    },
                    r#code,
                    r#timing,
                    r#value,
                    r#reason,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitSupportingInfo>>
{
    type Value = Box<ExplanationOfBenefitSupportingInfo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitSupportingInfo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitSupportingInfo>>
{
    type Value = Vec<ExplanationOfBenefitSupportingInfo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitSupportingInfo>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitSupportingInfo>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitSupportingInfo> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosis;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitDiagnosis> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.diagnosis", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosisDiagnosis as _Enum;
            match self.value.r#diagnosis {
                _Enum::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("diagnosisCodeableConcept", ctx)
                    })?;
                }
                _Enum::Reference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("diagnosisReference", ctx)
                    })?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("diagnosis is a required field"))
                }
            }
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#on_admission.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("onAdmission", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitDiagnosis>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitDiagnosis>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitDiagnosis>
{
    type Value = ExplanationOfBenefitDiagnosis;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitDiagnosis>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitDiagnosis")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitDiagnosis, V::Error>
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
                    #[serde(rename = "diagnosisCodeableConcept")]
                    DiagnosisCodeableConcept,
                    #[serde(rename = "diagnosisReference")]
                    DiagnosisReference,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "onAdmission")]
                    OnAdmission,
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
                            "diagnosisCodeableConcept",
                            "diagnosisReference",
                            "type",
                            "onAdmission",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#diagnosis: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosisDiagnosis,
                > = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#on_admission: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::DiagnosisCodeableConcept => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosisDiagnosis as _Enum;
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#diagnosis = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DiagnosisReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosisDiagnosis as _Enum;
                            if r#diagnosis.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#diagnosis = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OnAdmission => {
                            if r#on_admission.is_some() {
                                return Err(serde::de::Error::duplicate_field("onAdmission"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#on_admission = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitDiagnosis {
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
                    r#diagnosis: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#diagnosis.unwrap_or(Default::default())
                    } else {
                        r#diagnosis.ok_or(serde::de::Error::missing_field("diagnosis[x]"))?
                    },
                    r#type: r#type.unwrap_or(vec![]),
                    r#on_admission,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitDiagnosis>>
{
    type Value = Box<ExplanationOfBenefitDiagnosis>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitDiagnosis>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitDiagnosis>>
{
    type Value = Vec<ExplanationOfBenefitDiagnosis>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitDiagnosis>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitDiagnosis>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitDiagnosis> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitProcedure;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitProcedure> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.procedure", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
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
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitProcedureProcedure as _Enum;
            match self.value.r#procedure {
                _Enum::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("procedureCodeableConcept", ctx)
                    })?;
                }
                _Enum::Reference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("procedureReference", ctx)
                    })?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("procedure is a required field"))
                }
            }
        }
        if !self.value.r#udi.is_empty() {
            self.with_context(&self.value.r#udi, |ctx| state.serialize_entry("udi", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitProcedure>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitProcedure>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitProcedure>
{
    type Value = ExplanationOfBenefitProcedure;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitProcedure>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitProcedure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitProcedure, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "procedureCodeableConcept")]
                    ProcedureCodeableConcept,
                    #[serde(rename = "procedureReference")]
                    ProcedureReference,
                    #[serde(rename = "udi")]
                    Udi,
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
                            "type",
                            "date",
                            "procedureCodeableConcept",
                            "procedureReference",
                            "udi",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#procedure: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitProcedureProcedure,
                > = None;
                let mut r#udi: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    fhirbolt_model::r5::types::DateTime,
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
                        Field::ProcedureCodeableConcept => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitProcedureProcedure as _Enum;
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#procedure = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ProcedureReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitProcedureProcedure as _Enum;
                            if r#procedure.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "procedureReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#procedure = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Udi => {
                            if self.0.from_json {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#udi = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#udi.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
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
                Ok(ExplanationOfBenefitProcedure {
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
                    r#type: r#type.unwrap_or(vec![]),
                    r#date,
                    r#procedure: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#procedure.unwrap_or(Default::default())
                    } else {
                        r#procedure.ok_or(serde::de::Error::missing_field("procedure[x]"))?
                    },
                    r#udi: r#udi.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitProcedure>>
{
    type Value = Box<ExplanationOfBenefitProcedure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitProcedure>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitProcedure>>
{
    type Value = Vec<ExplanationOfBenefitProcedure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitProcedure>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitProcedure>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitProcedure> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitInsurance;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitInsurance> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.insurance", field
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
        }
        self.with_context(&self.value.r#focal, |ctx| {
            state.serialize_entry("focal", ctx)
        })?;
        if self.value.r#coverage.id.as_deref() == Some("$invalid") {
            return missing_field_error("coverage");
        }
        self.with_context(&self.value.r#coverage, |ctx| {
            state.serialize_entry("coverage", ctx)
        })?;
        if self.output_json {
            if !self.value.r#pre_auth_ref.is_empty() {
                let values = self
                    .value
                    .r#pre_auth_ref
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("preAuthRef", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#pre_auth_ref
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#pre_auth_ref
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
                        state.serialize_entry("_preAuthRef", ctx)
                    })?;
                }
            }
        } else if !self.value.r#pre_auth_ref.is_empty() {
            self.with_context(&self.value.r#pre_auth_ref, |ctx| {
                state.serialize_entry("preAuthRef", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitInsurance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitInsurance>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitInsurance>
{
    type Value = ExplanationOfBenefitInsurance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitInsurance>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitInsurance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitInsurance, V::Error>
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
                    #[serde(rename = "focal")]
                    Focal,
                    #[serde(rename = "_focal")]
                    FocalPrimitiveElement,
                    #[serde(rename = "coverage")]
                    Coverage,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "focal",
                            "coverage",
                            "preAuthRef",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#focal: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#coverage: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#pre_auth_ref: Option<Vec<fhirbolt_model::r5::types::String>> = None;
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
                        Field::Focal => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#focal = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FocalPrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#coverage = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PreAuthRef => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#pre_auth_ref.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("preAuthRef");
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
                Ok(ExplanationOfBenefitInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
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
                    r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitInsurance>>
{
    type Value = Box<ExplanationOfBenefitInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitInsurance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitInsurance>>
{
    type Value = Vec<ExplanationOfBenefitInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitInsurance>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitInsurance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitInsurance> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitAccident;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitAccident> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.accident", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitAccidentLocation as _Enum;
            if let Some(some) = self.value.r#location.as_ref() {
                match some {
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitAccident>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitAccident>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitAccident>
{
    type Value = ExplanationOfBenefitAccident;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitAccident>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitAccident;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAccident")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAccident, V::Error>
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
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "locationAddress")]
                    LocationAddress,
                    #[serde(rename = "locationReference")]
                    LocationReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "date",
                            "type",
                            "locationAddress",
                            "locationReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#location: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitAccidentLocation,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::LocationAddress => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAccidentLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Address>,
                            > = self.0.transmute();
                            r#location =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAccidentLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
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
                Ok(ExplanationOfBenefitAccident {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#date,
                    r#type,
                    r#location,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitAccident>>
{
    type Value = Box<ExplanationOfBenefitAccident>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitAccident>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitAccident>>
{
    type Value = Vec<ExplanationOfBenefitAccident>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitAccident>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitAccident>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitAccident> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItemBodySite;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItemBodySite> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item.bodySite", field
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
        if !self.value.r#site.is_empty() {
            self.with_context(&self.value.r#site, |ctx| state.serialize_entry("site", ctx))?;
        }
        if !self.value.r#sub_site.is_empty() {
            self.with_context(&self.value.r#sub_site, |ctx| {
                state.serialize_entry("subSite", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItemBodySite>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItemBodySite>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItemBodySite>
{
    type Value = ExplanationOfBenefitItemBodySite;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItemBodySite>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItemBodySite;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemBodySite")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemBodySite, V::Error>
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
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "subSite")]
                    SubSite,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "site", "subSite"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#site: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#sub_site: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Site => {
                            if self.0.from_json {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubSite => {
                            if self.0.from_json {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#sub_site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
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
                Ok(ExplanationOfBenefitItemBodySite {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#site: r#site.unwrap_or(vec![]),
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItemBodySite>>
{
    type Value = Box<ExplanationOfBenefitItemBodySite>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItemBodySite>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItemBodySite>>
{
    type Value = Vec<ExplanationOfBenefitItemBodySite>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitItemBodySite>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItemBodySite>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItemBodySite> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItemReviewOutcome> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item.reviewOutcome", field
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
        if let Some(some) = self.value.r#decision.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("decision", ctx))?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if self.output_json {
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItemReviewOutcome>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItemReviewOutcome>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItemReviewOutcome>
{
    type Value = ExplanationOfBenefitItemReviewOutcome;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItemReviewOutcome>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItemReviewOutcome;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemReviewOutcome")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemReviewOutcome, V::Error>
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
                    #[serde(rename = "decision")]
                    Decision,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    #[serde(rename = "preAuthPeriod")]
                    PreAuthPeriod,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "decision",
                            "reason",
                            "preAuthRef",
                            "preAuthPeriod",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#decision: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#pre_auth_ref: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#pre_auth_period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
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
                        Field::Decision => {
                            if r#decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#decision = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Reason => {
                            if self.0.from_json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#reason = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAuthRef => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#pre_auth_ref = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#pre_auth_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitItemReviewOutcome {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#decision,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#pre_auth_ref,
                    r#pre_auth_period,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItemReviewOutcome>>
{
    type Value = Box<ExplanationOfBenefitItemReviewOutcome>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItemReviewOutcome>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItemReviewOutcome>>
{
    type Value = Vec<ExplanationOfBenefitItemReviewOutcome>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitItemReviewOutcome>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItemReviewOutcome>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItemReviewOutcome> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItemAdjudication> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item.adjudication", field
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
        }
        self.with_context(&self.value.r#category, |ctx| {
            state.serialize_entry("category", ctx)
        })?;
        if let Some(some) = self.value.r#reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reason", ctx))?;
        }
        if let Some(some) = self.value.r#amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amount", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItemAdjudication>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItemAdjudication>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItemAdjudication>
{
    type Value = ExplanationOfBenefitItemAdjudication;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItemAdjudication>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItemAdjudication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemAdjudication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemAdjudication, V::Error>
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
                    #[serde(rename = "quantity")]
                    Quantity,
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
                            "quantity",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#reason: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Reason => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#reason = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitItemAdjudication {
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
                    r#quantity,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItemAdjudication>>
{
    type Value = Box<ExplanationOfBenefitItemAdjudication>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItemAdjudication>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItemAdjudication>>
{
    type Value = Vec<ExplanationOfBenefitItemAdjudication>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitItemAdjudication>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItemAdjudication>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItemAdjudication> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItemDetailSubDetail;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItemDetailSubDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item.detail.subDetail", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
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
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if !self.value.r#udi.is_empty() {
            self.with_context(&self.value.r#udi, |ctx| state.serialize_entry("udi", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItemDetailSubDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItemDetailSubDetail>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItemDetailSubDetail>
{
    type Value = ExplanationOfBenefitItemDetailSubDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItemDetailSubDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemDetailSubDetail, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "programCode")]
                    ProgramCode,
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "udi")]
                    Udi,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "sequence",
                            "traceNumber",
                            "revenue",
                            "category",
                            "productOrService",
                            "productOrServiceEnd",
                            "modifier",
                            "programCode",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "udi",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#program_code: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#udi: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProgramCode => {
                            if self.0.from_json {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Udi => {
                            if self.0.from_json {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#udi = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#udi.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitItemDetailSubDetail {
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
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#revenue,
                    r#category,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItemDetailSubDetail>>
{
    type Value = Box<ExplanationOfBenefitItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItemDetailSubDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItemDetailSubDetail>>
{
    type Value = Vec<ExplanationOfBenefitItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitItemDetailSubDetail>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItemDetailSubDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItemDetailSubDetail> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItemDetail;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItemDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item.detail", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
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
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if !self.value.r#udi.is_empty() {
            self.with_context(&self.value.r#udi, |ctx| state.serialize_entry("udi", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
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
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItemDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItemDetail>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItemDetail>
{
    type Value = ExplanationOfBenefitItemDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItemDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItemDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitItemDetail, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "programCode")]
                    ProgramCode,
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "udi")]
                    Udi,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "sequence",
                            "traceNumber",
                            "revenue",
                            "category",
                            "productOrService",
                            "productOrServiceEnd",
                            "modifier",
                            "programCode",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "udi",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                            "subDetail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#program_code: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#udi: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
                > = None;
                let mut r#sub_detail: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemDetailSubDetail>,
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProgramCode => {
                            if self.0.from_json {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Udi => {
                            if self.0.from_json {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#udi = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#udi.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetail => {
                            if self.0.from_json {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemDetailSubDetail >> = self . 0 . transmute () ;
                                r#sub_detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_detail.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemDetailSubDetail > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitItemDetail {
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
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#revenue,
                    r#category,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItemDetail>>
{
    type Value = Box<ExplanationOfBenefitItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItemDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItemDetail>>
{
    type Value = Vec<ExplanationOfBenefitItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitItemDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItemDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItemDetail> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitItem;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitItem> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.item", field
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
        }
        self.with_context(&self.value.r#sequence, |ctx| {
            state.serialize_entry("sequence", ctx)
        })?;
        if self.output_json {
            if !self.value.r#care_team_sequence.is_empty() {
                let values = self
                    .value
                    .r#care_team_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("careTeamSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#care_team_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#care_team_sequence
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
                        state.serialize_entry("_careTeamSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#care_team_sequence.is_empty() {
            self.with_context(&self.value.r#care_team_sequence, |ctx| {
                state.serialize_entry("careTeamSequence", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#diagnosis_sequence.is_empty() {
                let values = self
                    .value
                    .r#diagnosis_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("diagnosisSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#diagnosis_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#diagnosis_sequence
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
                        state.serialize_entry("_diagnosisSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#diagnosis_sequence.is_empty() {
            self.with_context(&self.value.r#diagnosis_sequence, |ctx| {
                state.serialize_entry("diagnosisSequence", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#procedure_sequence.is_empty() {
                let values = self
                    .value
                    .r#procedure_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("procedureSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#procedure_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#procedure_sequence
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
                        state.serialize_entry("_procedureSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#procedure_sequence.is_empty() {
            self.with_context(&self.value.r#procedure_sequence, |ctx| {
                state.serialize_entry("procedureSequence", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#information_sequence.is_empty() {
                let values = self
                    .value
                    .r#information_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("informationSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#information_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#information_sequence
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
                        state.serialize_entry("_informationSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#information_sequence.is_empty() {
            self.with_context(&self.value.r#information_sequence, |ctx| {
                state.serialize_entry("informationSequence", ctx)
            })?;
        }
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#category.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("category", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
            })?;
        }
        if !self.value.r#request.is_empty() {
            self.with_context(&self.value.r#request, |ctx| {
                state.serialize_entry("request", ctx)
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
            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemServiced as _Enum;
            if let Some(some) = self.value.r#serviced.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output_json {
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
            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemLocation as _Enum;
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
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if !self.value.r#udi.is_empty() {
            self.with_context(&self.value.r#udi, |ctx| state.serialize_entry("udi", ctx))?;
        }
        if !self.value.r#body_site.is_empty() {
            self.with_context(&self.value.r#body_site, |ctx| {
                state.serialize_entry("bodySite", ctx)
            })?;
        }
        if !self.value.r#encounter.is_empty() {
            self.with_context(&self.value.r#encounter, |ctx| {
                state.serialize_entry("encounter", ctx)
            })?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
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
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitItem>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitItem>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitItem>
{
    type Value = ExplanationOfBenefitItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitItem>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitItem, V::Error>
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
                    #[serde(rename = "careTeamSequence")]
                    CareTeamSequence,
                    #[serde(rename = "_careTeamSequence")]
                    CareTeamSequencePrimitiveElement,
                    #[serde(rename = "diagnosisSequence")]
                    DiagnosisSequence,
                    #[serde(rename = "_diagnosisSequence")]
                    DiagnosisSequencePrimitiveElement,
                    #[serde(rename = "procedureSequence")]
                    ProcedureSequence,
                    #[serde(rename = "_procedureSequence")]
                    ProcedureSequencePrimitiveElement,
                    #[serde(rename = "informationSequence")]
                    InformationSequence,
                    #[serde(rename = "_informationSequence")]
                    InformationSequencePrimitiveElement,
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "request")]
                    Request,
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
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "udi")]
                    Udi,
                    #[serde(rename = "bodySite")]
                    BodySite,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "sequence",
                            "careTeamSequence",
                            "diagnosisSequence",
                            "procedureSequence",
                            "informationSequence",
                            "traceNumber",
                            "revenue",
                            "category",
                            "productOrService",
                            "productOrServiceEnd",
                            "request",
                            "modifier",
                            "programCode",
                            "servicedDate",
                            "servicedPeriod",
                            "locationCodeableConcept",
                            "locationAddress",
                            "locationReference",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "udi",
                            "bodySite",
                            "encounter",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                            "detail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#care_team_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> =
                    None;
                let mut r#diagnosis_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> =
                    None;
                let mut r#procedure_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> =
                    None;
                let mut r#information_sequence: Option<
                    Vec<fhirbolt_model::r5::types::PositiveInt>,
                > = None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#request: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#program_code: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#serviced: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemServiced,
                > = None;
                let mut r#location: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemLocation,
                > = None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#udi: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#body_site: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemBodySite>,
                > = None;
                let mut r#encounter: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
                > = None;
                let mut r#detail: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemDetail>,
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
                        Field::Sequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#sequence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::CareTeamSequence => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#care_team_sequence.get_or_insert(
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
                                        "careTeamSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#care_team_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CareTeamSequencePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#care_team_sequence.get_or_insert(
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
                                        "_careTeamSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("careTeamSequence");
                            }
                        }
                        Field::DiagnosisSequence => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#diagnosis_sequence.get_or_insert(
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
                                        "diagnosisSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#diagnosis_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DiagnosisSequencePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#diagnosis_sequence.get_or_insert(
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
                                        "_diagnosisSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("diagnosisSequence");
                            }
                        }
                        Field::ProcedureSequence => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#procedure_sequence.get_or_insert(
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
                                        "procedureSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#procedure_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProcedureSequencePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#procedure_sequence.get_or_insert(
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
                                        "_procedureSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("procedureSequence");
                            }
                        }
                        Field::InformationSequence => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#information_sequence.get_or_insert(
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
                                        "informationSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#information_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::InformationSequencePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#information_sequence.get_or_insert(
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
                                        "_informationSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("informationSequence");
                            }
                        }
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Request => {
                            if self.0.from_json {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#request = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#request.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProgramCode => {
                            if self.0.from_json {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ServicedDate => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemServiced as _Enum;
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::Date>,
                                > = self.0.transmute();
                                r#serviced =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemServiced as _Enum;
                            if self.0.from_json {
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
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemServiced as _Enum;
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#serviced =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationCodeableConcept => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::LocationAddress => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Address>,
                            > = self.0.transmute();
                            r#location =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Udi => {
                            if self.0.from_json {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#udi = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#udi.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BodySite => {
                            if self.0.from_json {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemBodySite >> = self . 0 . transmute () ;
                                r#body_site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#body_site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemBodySite,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Encounter => {
                            if self.0.from_json {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#encounter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Detail => {
                            if self.0.from_json {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemDetail >> = self . 0 . transmute () ;
                                r#detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#detail.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemDetail,
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
                Ok(ExplanationOfBenefitItem {
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
                    r#care_team_sequence: r#care_team_sequence.unwrap_or(vec![]),
                    r#diagnosis_sequence: r#diagnosis_sequence.unwrap_or(vec![]),
                    r#procedure_sequence: r#procedure_sequence.unwrap_or(vec![]),
                    r#information_sequence: r#information_sequence.unwrap_or(vec![]),
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#revenue,
                    r#category,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#request: r#request.unwrap_or(vec![]),
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#udi: r#udi.unwrap_or(vec![]),
                    r#body_site: r#body_site.unwrap_or(vec![]),
                    r#encounter: r#encounter.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitItem>>
{
    type Value = Box<ExplanationOfBenefitItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitItem>>
{
    type Value = Vec<ExplanationOfBenefitItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitItem>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitItem> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemBodySite;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitAddItemBodySite> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.addItem.bodySite", field
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
        if !self.value.r#site.is_empty() {
            self.with_context(&self.value.r#site, |ctx| state.serialize_entry("site", ctx))?;
        }
        if !self.value.r#sub_site.is_empty() {
            self.with_context(&self.value.r#sub_site, |ctx| {
                state.serialize_entry("subSite", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitAddItemBodySite>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitAddItemBodySite>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitAddItemBodySite>
{
    type Value = ExplanationOfBenefitAddItemBodySite;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitAddItemBodySite>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitAddItemBodySite;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItemBodySite")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItemBodySite, V::Error>
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
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "subSite")]
                    SubSite,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "site", "subSite"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#site: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#sub_site: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Site => {
                            if self.0.from_json {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubSite => {
                            if self.0.from_json {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#sub_site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_site.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
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
                Ok(ExplanationOfBenefitAddItemBodySite {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#site: r#site.unwrap_or(vec![]),
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitAddItemBodySite>>
{
    type Value = Box<ExplanationOfBenefitAddItemBodySite>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitAddItemBodySite>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitAddItemBodySite>>
{
    type Value = Vec<ExplanationOfBenefitAddItemBodySite>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitAddItemBodySite>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitAddItemBodySite>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitAddItemBodySite> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemDetailSubDetail;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitAddItemDetailSubDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.addItem.detail.subDetail", field
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
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
            })?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
        }
        if !self.value.r#adjudication.is_empty() {
            self.with_context(&self.value.r#adjudication, |ctx| {
                state.serialize_entry("adjudication", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<ExplanationOfBenefitAddItemDetailSubDetail>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<ExplanationOfBenefitAddItemDetailSubDetail>>
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
    for &mut DeserializationContext<ExplanationOfBenefitAddItemDetailSubDetail>
{
    type Value = ExplanationOfBenefitAddItemDetailSubDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<ExplanationOfBenefitAddItemDetailSubDetail>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitAddItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItemDetailSubDetail, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "traceNumber",
                            "revenue",
                            "productOrService",
                            "productOrServiceEnd",
                            "modifier",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
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
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitAddItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#revenue,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitAddItemDetailSubDetail>>
{
    type Value = Box<ExplanationOfBenefitAddItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitAddItemDetailSubDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitAddItemDetailSubDetail>>
{
    type Value = Vec<ExplanationOfBenefitAddItemDetailSubDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitAddItemDetailSubDetail>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitAddItemDetailSubDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    ExplanationOfBenefitAddItemDetailSubDetail,
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemDetail;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitAddItemDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.addItem.detail", field
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
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
            })?;
        }
        if !self.value.r#modifier.is_empty() {
            self.with_context(&self.value.r#modifier, |ctx| {
                state.serialize_entry("modifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
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
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitAddItemDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitAddItemDetail>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitAddItemDetail>
{
    type Value = ExplanationOfBenefitAddItemDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitAddItemDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitAddItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItemDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItemDetail, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "traceNumber",
                            "revenue",
                            "productOrService",
                            "productOrServiceEnd",
                            "modifier",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                            "subDetail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
                > = None;
                let mut r#sub_detail: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemDetailSubDetail>,
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
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetail => {
                            if self.0.from_json {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemDetailSubDetail >> = self . 0 . transmute () ;
                                r#sub_detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_detail.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemDetailSubDetail > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitAddItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#revenue,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitAddItemDetail>>
{
    type Value = Box<ExplanationOfBenefitAddItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitAddItemDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitAddItemDetail>>
{
    type Value = Vec<ExplanationOfBenefitAddItemDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitAddItemDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitAddItemDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitAddItemDetail> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItem;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitAddItem> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.addItem", field
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
        if self.output_json {
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
        if self.output_json {
            if !self.value.r#sub_detail_sequence.is_empty() {
                let values = self
                    .value
                    .r#sub_detail_sequence
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("subDetailSequence", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#sub_detail_sequence
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#sub_detail_sequence
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
                        state.serialize_entry("_subDetailSequence", ctx)
                    })?;
                }
            }
        } else if !self.value.r#sub_detail_sequence.is_empty() {
            self.with_context(&self.value.r#sub_detail_sequence, |ctx| {
                state.serialize_entry("subDetailSequence", ctx)
            })?;
        }
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
        }
        if !self.value.r#provider.is_empty() {
            self.with_context(&self.value.r#provider, |ctx| {
                state.serialize_entry("provider", ctx)
            })?;
        }
        if let Some(some) = self.value.r#revenue.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("revenue", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("productOrService", ctx))?;
        }
        if let Some(some) = self.value.r#product_or_service_end.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("productOrServiceEnd", ctx)
            })?;
        }
        if !self.value.r#request.is_empty() {
            self.with_context(&self.value.r#request, |ctx| {
                state.serialize_entry("request", ctx)
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
            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemServiced as _Enum;
            if let Some(some) = self.value.r#serviced.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output_json {
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
            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemLocation as _Enum;
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
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#tax.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("tax", ctx))?;
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if !self.value.r#body_site.is_empty() {
            self.with_context(&self.value.r#body_site, |ctx| {
                state.serialize_entry("bodySite", ctx)
            })?;
        }
        if self.output_json {
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
        if let Some(some) = self.value.r#review_outcome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reviewOutcome", ctx))?;
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
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitAddItem>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitAddItem>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitAddItem>
{
    type Value = ExplanationOfBenefitAddItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitAddItem>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitAddItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitAddItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitAddItem, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "revenue")]
                    Revenue,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "productOrServiceEnd")]
                    ProductOrServiceEnd,
                    #[serde(rename = "request")]
                    Request,
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
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "tax")]
                    Tax,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "bodySite")]
                    BodySite,
                    #[serde(rename = "noteNumber")]
                    NoteNumber,
                    #[serde(rename = "_noteNumber")]
                    NoteNumberPrimitiveElement,
                    #[serde(rename = "reviewOutcome")]
                    ReviewOutcome,
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
                            "subDetailSequence",
                            "traceNumber",
                            "provider",
                            "revenue",
                            "productOrService",
                            "productOrServiceEnd",
                            "request",
                            "modifier",
                            "programCode",
                            "servicedDate",
                            "servicedPeriod",
                            "locationCodeableConcept",
                            "locationAddress",
                            "locationReference",
                            "patientPaid",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "tax",
                            "net",
                            "bodySite",
                            "noteNumber",
                            "reviewOutcome",
                            "adjudication",
                            "detail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#item_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#detail_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> =
                    None;
                let mut r#sub_detail_sequence: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> =
                    None;
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#provider: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#revenue: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#product_or_service_end: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#request: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#program_code: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#serviced: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemServiced,
                > = None;
                let mut r#location: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemLocation,
                > = None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#tax: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#net: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#body_site: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemBodySite>,
                > = None;
                let mut r#note_number: Option<Vec<fhirbolt_model::r5::types::PositiveInt>> = None;
                let mut r#review_outcome: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitItemReviewOutcome,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
                > = None;
                let mut r#detail: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemDetail>,
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
                        Field::ItemSequence => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ItemSequencePrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DetailSequencePrimitiveElement => {
                            if self.0.from_json {
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
                        Field::SubDetailSequence => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#sub_detail_sequence.get_or_insert(
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
                                        "subDetailSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#sub_detail_sequence.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubDetailSequencePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#sub_detail_sequence.get_or_insert(
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
                                        "_subDetailSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("subDetailSequence");
                            }
                        }
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Provider => {
                            if self.0.from_json {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#provider = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#provider.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Revenue => {
                            if r#revenue.is_some() {
                                return Err(serde::de::Error::duplicate_field("revenue"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#revenue = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProductOrServiceEnd => {
                            if r#product_or_service_end.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productOrServiceEnd",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product_or_service_end =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Request => {
                            if self.0.from_json {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#request = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#request.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProgramCode => {
                            if self.0.from_json {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program_code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ServicedDate => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemServiced as _Enum;
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::Date>,
                                > = self.0.transmute();
                                r#serviced =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemServiced as _Enum;
                            if self.0.from_json {
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
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemServiced as _Enum;
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#serviced =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationCodeableConcept => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::LocationAddress => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Address>,
                            > = self.0.transmute();
                            r#location =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::LocationReference => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitAddItemLocation as _Enum;
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#unit_price = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#factor = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::Tax => {
                            if r#tax.is_some() {
                                return Err(serde::de::Error::duplicate_field("tax"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#tax = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#net = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BodySite => {
                            if self.0.from_json {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemBodySite >> = self . 0 . transmute () ;
                                r#body_site = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#body_site.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemBodySite > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumber => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            if self.0.from_json {
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
                        Field::ReviewOutcome => {
                            if r#review_outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewOutcome"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemReviewOutcome > = self . 0 . transmute () ;
                            r#review_outcome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Detail => {
                            if self.0.from_json {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemDetail >> = self . 0 . transmute () ;
                                r#detail = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#detail.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitAddItemDetail > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitAddItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: r#item_sequence.unwrap_or(vec![]),
                    r#detail_sequence: r#detail_sequence.unwrap_or(vec![]),
                    r#sub_detail_sequence: r#sub_detail_sequence.unwrap_or(vec![]),
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
                    r#provider: r#provider.unwrap_or(vec![]),
                    r#revenue,
                    r#product_or_service,
                    r#product_or_service_end,
                    r#request: r#request.unwrap_or(vec![]),
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#patient_paid,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#tax,
                    r#net,
                    r#body_site: r#body_site.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#review_outcome,
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitAddItem>>
{
    type Value = Box<ExplanationOfBenefitAddItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitAddItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitAddItem>>
{
    type Value = Vec<ExplanationOfBenefitAddItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitAddItem>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitAddItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitAddItem> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitTotal;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitTotal> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.total", field
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
        }
        self.with_context(&self.value.r#category, |ctx| {
            state.serialize_entry("category", ctx)
        })?;
        if self.value.r#amount.id.as_deref() == Some("$invalid") {
            return missing_field_error("amount");
        }
        self.with_context(&self.value.r#amount, |ctx| {
            state.serialize_entry("amount", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitTotal>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitTotal>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitTotal>
{
    type Value = ExplanationOfBenefitTotal;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitTotal>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitTotal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitTotal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefitTotal, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
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
                Ok(ExplanationOfBenefitTotal {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitTotal>>
{
    type Value = Box<ExplanationOfBenefitTotal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitTotal>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitTotal>>
{
    type Value = Vec<ExplanationOfBenefitTotal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitTotal>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitTotal>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitTotal> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitPayment;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitPayment> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.payment", field
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
        if let Some(some) = self.value.r#adjustment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("adjustment", ctx))?;
        }
        if let Some(some) = self.value.r#adjustment_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("adjustmentReason", ctx))?;
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
        if let Some(some) = self.value.r#amount.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("amount", ctx))?;
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitPayment>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitPayment>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitPayment>
{
    type Value = ExplanationOfBenefitPayment;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitPayment>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitPayment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitPayment")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitPayment, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#adjustment: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#adjustment_reason: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#amount: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Adjustment => {
                            if r#adjustment.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#adjustment = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AdjustmentReason => {
                            if r#adjustment_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustmentReason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#adjustment_reason = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#amount = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ExplanationOfBenefitPayment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#adjustment,
                    r#adjustment_reason,
                    r#date,
                    r#amount,
                    r#identifier,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitPayment>>
{
    type Value = Box<ExplanationOfBenefitPayment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitPayment>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitPayment>>
{
    type Value = Vec<ExplanationOfBenefitPayment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitPayment>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitPayment>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitPayment> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitProcessNote;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitProcessNote> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.processNote", field
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
        if let Some(some) = self.value.r#type.as_ref() {
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
        if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitProcessNote>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitProcessNote>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitProcessNote>
{
    type Value = ExplanationOfBenefitProcessNote;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitProcessNote>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitProcessNote")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitProcessNote, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#number: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#text: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#language: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Number => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#number = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NumberPrimitiveElement => {
                            if self.0.from_json {
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
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
                Ok(ExplanationOfBenefitProcessNote {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number,
                    r#type,
                    r#text,
                    r#language,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitProcessNote>>
{
    type Value = Box<ExplanationOfBenefitProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitProcessNote>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitProcessNote>>
{
    type Value = Vec<ExplanationOfBenefitProcessNote>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitProcessNote>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitProcessNote>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitProcessNote> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancial;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitBenefitBalanceFinancial> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.benefitBalance.financial", field
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
        {
            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
            if let Some(some) = self.value.r#allowed.as_ref() {
                match some {
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
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
                        if self.output_json {
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
            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialUsed as _Enum;
            if let Some(some) = self.value.r#used.as_ref() {
                match some {
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
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
    for SerializationContext<&Box<ExplanationOfBenefitBenefitBalanceFinancial>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<ExplanationOfBenefitBenefitBalanceFinancial>>
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
    for &mut DeserializationContext<ExplanationOfBenefitBenefitBalanceFinancial>
{
    type Value = ExplanationOfBenefitBenefitBalanceFinancial;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<ExplanationOfBenefitBenefitBalanceFinancial>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitBenefitBalanceFinancial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitBenefitBalanceFinancial")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitBenefitBalanceFinancial, V::Error>
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
                            "usedMoney",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#allowed : Option < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitBenefitBalanceFinancialAllowed > = None ;
                let mut r#used: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialUsed,
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AllowedUnsignedInt => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#allowed = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::AllowedUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
                            if self.0.from_json {
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
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::String>,
                                > = self.0.transmute();
                                r#allowed = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::AllowedStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
                            if self.0.from_json {
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
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialAllowed as _Enum;
                            if r#allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#allowed =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::UsedUnsignedInt => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialUsed as _Enum;
                            if self.0.from_json {
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
                                    Box<fhirbolt_model::r5::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#used = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::UsedUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialUsed as _Enum;
                            if self.0.from_json {
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
                        Field::UsedMoney => {
                            use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancialUsed as _Enum;
                            if r#used.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
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
                Ok(ExplanationOfBenefitBenefitBalanceFinancial {
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
    for &mut DeserializationContext<Box<ExplanationOfBenefitBenefitBalanceFinancial>>
{
    type Value = Box<ExplanationOfBenefitBenefitBalanceFinancial>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitBenefitBalanceFinancial>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitBenefitBalanceFinancial>>
{
    type Value = Vec<ExplanationOfBenefitBenefitBalanceFinancial>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ExplanationOfBenefitBenefitBalanceFinancial>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitBenefitBalanceFinancial>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    ExplanationOfBenefitBenefitBalanceFinancial,
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
use fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalance;
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefitBenefitBalance> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit.benefitBalance", field
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
        }
        self.with_context(&self.value.r#category, |ctx| {
            state.serialize_entry("category", ctx)
        })?;
        if self.output_json {
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
        if self.output_json {
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
        if self.output_json {
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
        if !self.value.r#financial.is_empty() {
            self.with_context(&self.value.r#financial, |ctx| {
                state.serialize_entry("financial", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefitBenefitBalance>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefitBenefitBalance>> {
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
    for &mut DeserializationContext<ExplanationOfBenefitBenefitBalance>
{
    type Value = ExplanationOfBenefitBenefitBalance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefitBenefitBalance>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefitBenefitBalance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefitBenefitBalance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ExplanationOfBenefitBenefitBalance, V::Error>
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
                    #[serde(rename = "financial")]
                    Financial,
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
                            "excluded",
                            "name",
                            "description",
                            "network",
                            "unit",
                            "term",
                            "financial",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#category: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#excluded: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#network: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#term: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#financial: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalanceFinancial>,
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#category = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Excluded => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#excluded = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExcludedPrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
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
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#network = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unit => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#unit = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Term => {
                            if r#term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#term = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Financial => {
                            if self.0.from_json {
                                if r#financial.is_some() {
                                    return Err(serde::de::Error::duplicate_field("financial"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitBenefitBalanceFinancial >> = self . 0 . transmute () ;
                                r#financial = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#financial.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitBenefitBalanceFinancial > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefitBenefitBalance {
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
                    r#excluded,
                    r#name,
                    r#description,
                    r#network,
                    r#unit,
                    r#term,
                    r#financial: r#financial.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefitBenefitBalance>>
{
    type Value = Box<ExplanationOfBenefitBenefitBalance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefitBenefitBalance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefitBenefitBalance>>
{
    type Value = Vec<ExplanationOfBenefitBenefitBalance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefitBenefitBalance>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefitBenefitBalance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefitBenefitBalance> =
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
use fhirbolt_model::r5::resources::ExplanationOfBenefit;
impl crate::Resource for ExplanationOfBenefit {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&ExplanationOfBenefit> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ExplanationOfBenefit", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ExplanationOfBenefit")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
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
        if !self.value.r#trace_number.is_empty() {
            self.with_context(&self.value.r#trace_number, |ctx| {
                state.serialize_entry("traceNumber", ctx)
            })?;
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
        }
        self.with_context(&self.value.r#status, |ctx| {
            state.serialize_entry("status", ctx)
        })?;
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if let Some(some) = self.value.r#sub_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subType", ctx))?;
        }
        if self.output_json {
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
        }
        self.with_context(&self.value.r#use, |ctx| state.serialize_entry("use", ctx))?;
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        }
        self.with_context(&self.value.r#patient, |ctx| {
            state.serialize_entry("patient", ctx)
        })?;
        if let Some(some) = self.value.r#billable_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("billablePeriod", ctx))?;
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
        }
        self.with_context(&self.value.r#created, |ctx| {
            state.serialize_entry("created", ctx)
        })?;
        if let Some(some) = self.value.r#enterer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("enterer", ctx))?;
        }
        if let Some(some) = self.value.r#insurer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("insurer", ctx))?;
        }
        if let Some(some) = self.value.r#provider.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("provider", ctx))?;
        }
        if let Some(some) = self.value.r#priority.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
        }
        if let Some(some) = self.value.r#funds_reserve_requested.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("fundsReserveRequested", ctx)
            })?;
        }
        if let Some(some) = self.value.r#funds_reserve.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("fundsReserve", ctx))?;
        }
        if !self.value.r#related.is_empty() {
            self.with_context(&self.value.r#related, |ctx| {
                state.serialize_entry("related", ctx)
            })?;
        }
        if let Some(some) = self.value.r#prescription.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("prescription", ctx))?;
        }
        if let Some(some) = self.value.r#original_prescription.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("originalPrescription", ctx)
            })?;
        }
        if !self.value.r#event.is_empty() {
            self.with_context(&self.value.r#event, |ctx| {
                state.serialize_entry("event", ctx)
            })?;
        }
        if let Some(some) = self.value.r#payee.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("payee", ctx))?;
        }
        if let Some(some) = self.value.r#referral.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referral", ctx))?;
        }
        if !self.value.r#encounter.is_empty() {
            self.with_context(&self.value.r#encounter, |ctx| {
                state.serialize_entry("encounter", ctx)
            })?;
        }
        if let Some(some) = self.value.r#facility.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("facility", ctx))?;
        }
        if let Some(some) = self.value.r#claim.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("claim", ctx))?;
        }
        if let Some(some) = self.value.r#claim_response.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("claimResponse", ctx))?;
        }
        if self.output_json {
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
        }
        self.with_context(&self.value.r#outcome, |ctx| {
            state.serialize_entry("outcome", ctx)
        })?;
        if let Some(some) = self.value.r#decision.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("decision", ctx))?;
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
            if !self.value.r#pre_auth_ref.is_empty() {
                let values = self
                    .value
                    .r#pre_auth_ref
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("preAuthRef", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#pre_auth_ref
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#pre_auth_ref
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
                        state.serialize_entry("_preAuthRef", ctx)
                    })?;
                }
            }
        } else if !self.value.r#pre_auth_ref.is_empty() {
            self.with_context(&self.value.r#pre_auth_ref, |ctx| {
                state.serialize_entry("preAuthRef", ctx)
            })?;
        }
        if !self.value.r#pre_auth_ref_period.is_empty() {
            self.with_context(&self.value.r#pre_auth_ref_period, |ctx| {
                state.serialize_entry("preAuthRefPeriod", ctx)
            })?;
        }
        if let Some(some) = self.value.r#diagnosis_related_group.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("diagnosisRelatedGroup", ctx)
            })?;
        }
        if !self.value.r#care_team.is_empty() {
            self.with_context(&self.value.r#care_team, |ctx| {
                state.serialize_entry("careTeam", ctx)
            })?;
        }
        if !self.value.r#supporting_info.is_empty() {
            self.with_context(&self.value.r#supporting_info, |ctx| {
                state.serialize_entry("supportingInfo", ctx)
            })?;
        }
        if !self.value.r#diagnosis.is_empty() {
            self.with_context(&self.value.r#diagnosis, |ctx| {
                state.serialize_entry("diagnosis", ctx)
            })?;
        }
        if !self.value.r#procedure.is_empty() {
            self.with_context(&self.value.r#procedure, |ctx| {
                state.serialize_entry("procedure", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#precedence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("precedence", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_precedence", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#precedence.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("precedence", ctx))?;
        }
        if !self.value.r#insurance.is_empty() {
            self.with_context(&self.value.r#insurance, |ctx| {
                state.serialize_entry("insurance", ctx)
            })?;
        }
        if let Some(some) = self.value.r#accident.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("accident", ctx))?;
        }
        if let Some(some) = self.value.r#patient_paid.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patientPaid", ctx))?;
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
        if let Some(some) = self.value.r#benefit_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("benefitPeriod", ctx))?;
        }
        if !self.value.r#benefit_balance.is_empty() {
            self.with_context(&self.value.r#benefit_balance, |ctx| {
                state.serialize_entry("benefitBalance", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ExplanationOfBenefit>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ExplanationOfBenefit>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<ExplanationOfBenefit> {
    type Value = ExplanationOfBenefit;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ExplanationOfBenefit> {
    type Value = ExplanationOfBenefit;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ExplanationOfBenefit>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ExplanationOfBenefit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ExplanationOfBenefit")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ExplanationOfBenefit, V::Error>
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
                    #[serde(rename = "traceNumber")]
                    TraceNumber,
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
                    #[serde(rename = "billablePeriod")]
                    BillablePeriod,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "enterer")]
                    Enterer,
                    #[serde(rename = "insurer")]
                    Insurer,
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "fundsReserveRequested")]
                    FundsReserveRequested,
                    #[serde(rename = "fundsReserve")]
                    FundsReserve,
                    #[serde(rename = "related")]
                    Related,
                    #[serde(rename = "prescription")]
                    Prescription,
                    #[serde(rename = "originalPrescription")]
                    OriginalPrescription,
                    #[serde(rename = "event")]
                    Event,
                    #[serde(rename = "payee")]
                    Payee,
                    #[serde(rename = "referral")]
                    Referral,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "facility")]
                    Facility,
                    #[serde(rename = "claim")]
                    Claim,
                    #[serde(rename = "claimResponse")]
                    ClaimResponse,
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "_outcome")]
                    OutcomePrimitiveElement,
                    #[serde(rename = "decision")]
                    Decision,
                    #[serde(rename = "disposition")]
                    Disposition,
                    #[serde(rename = "_disposition")]
                    DispositionPrimitiveElement,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    #[serde(rename = "preAuthRefPeriod")]
                    PreAuthRefPeriod,
                    #[serde(rename = "diagnosisRelatedGroup")]
                    DiagnosisRelatedGroup,
                    #[serde(rename = "careTeam")]
                    CareTeam,
                    #[serde(rename = "supportingInfo")]
                    SupportingInfo,
                    #[serde(rename = "diagnosis")]
                    Diagnosis,
                    #[serde(rename = "procedure")]
                    Procedure,
                    #[serde(rename = "precedence")]
                    Precedence,
                    #[serde(rename = "_precedence")]
                    PrecedencePrimitiveElement,
                    #[serde(rename = "insurance")]
                    Insurance,
                    #[serde(rename = "accident")]
                    Accident,
                    #[serde(rename = "patientPaid")]
                    PatientPaid,
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
                    #[serde(rename = "formCode")]
                    FormCode,
                    #[serde(rename = "form")]
                    Form,
                    #[serde(rename = "processNote")]
                    ProcessNote,
                    #[serde(rename = "benefitPeriod")]
                    BenefitPeriod,
                    #[serde(rename = "benefitBalance")]
                    BenefitBalance,
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
                            "traceNumber",
                            "status",
                            "type",
                            "subType",
                            "use",
                            "patient",
                            "billablePeriod",
                            "created",
                            "enterer",
                            "insurer",
                            "provider",
                            "priority",
                            "fundsReserveRequested",
                            "fundsReserve",
                            "related",
                            "prescription",
                            "originalPrescription",
                            "event",
                            "payee",
                            "referral",
                            "encounter",
                            "facility",
                            "claim",
                            "claimResponse",
                            "outcome",
                            "decision",
                            "disposition",
                            "preAuthRef",
                            "preAuthRefPeriod",
                            "diagnosisRelatedGroup",
                            "careTeam",
                            "supportingInfo",
                            "diagnosis",
                            "procedure",
                            "precedence",
                            "insurance",
                            "accident",
                            "patientPaid",
                            "item",
                            "addItem",
                            "adjudication",
                            "total",
                            "payment",
                            "formCode",
                            "form",
                            "processNote",
                            "benefitPeriod",
                            "benefitBalance",
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
                let mut r#trace_number: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#use: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#billable_period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#created: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#enterer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#insurer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#provider: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#priority: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#funds_reserve_requested: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#funds_reserve: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#related: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitRelated>,
                > = None;
                let mut r#prescription: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#original_prescription: Option<Box<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#event: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitEvent>,
                > = None;
                let mut r#payee: Option<fhirbolt_model::r5::resources::ExplanationOfBenefitPayee> =
                    None;
                let mut r#referral: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#facility: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#claim: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#claim_response: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#outcome: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#decision: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#disposition: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#pre_auth_ref: Option<Vec<fhirbolt_model::r5::types::String>> = None;
                let mut r#pre_auth_ref_period: Option<Vec<fhirbolt_model::r5::types::Period>> =
                    None;
                let mut r#diagnosis_related_group: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#care_team: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitCareTeam>,
                > = None;
                let mut r#supporting_info: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitSupportingInfo>,
                > = None;
                let mut r#diagnosis: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosis>,
                > = None;
                let mut r#procedure: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitProcedure>,
                > = None;
                let mut r#precedence: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#insurance: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitInsurance>,
                > = None;
                let mut r#accident: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitAccident,
                > = None;
                let mut r#patient_paid: Option<Box<fhirbolt_model::r5::types::Money>> = None;
                let mut r#item: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItem>,
                > = None;
                let mut r#add_item: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitAddItem>,
                > = None;
                let mut r#adjudication: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItemAdjudication>,
                > = None;
                let mut r#total: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitTotal>,
                > = None;
                let mut r#payment: Option<
                    fhirbolt_model::r5::resources::ExplanationOfBenefitPayment,
                > = None;
                let mut r#form_code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#form: Option<Box<fhirbolt_model::r5::types::Attachment>> = None;
                let mut r#process_note: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitProcessNote>,
                > = None;
                let mut r#benefit_period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#benefit_balance: Option<
                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitBenefitBalance>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "ExplanationOfBenefit" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"ExplanationOfBenefit",
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
                        Field::TraceNumber => {
                            if self.0.from_json {
                                if r#trace_number.is_some() {
                                    return Err(serde::de::Error::duplicate_field("traceNumber"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#trace_number = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#trace_number.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SubType => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#sub_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Use => {
                            if self.0.from_json {
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
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#use = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UsePrimitiveElement => {
                            if self.0.from_json {
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
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BillablePeriod => {
                            if r#billable_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("billablePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#billable_period = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Insurer => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#insurer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Provider => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#provider = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Priority => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#priority = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::FundsReserveRequested => {
                            if r#funds_reserve_requested.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fundsReserveRequested",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#funds_reserve_requested =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::FundsReserve => {
                            if r#funds_reserve.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundsReserve"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#funds_reserve = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Related => {
                            if self.0.from_json {
                                if r#related.is_some() {
                                    return Err(serde::de::Error::duplicate_field("related"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitRelated>,
                                > = self.0.transmute();
                                r#related = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#related.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitRelated,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Prescription => {
                            if r#prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field("prescription"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#prescription = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::OriginalPrescription => {
                            if r#original_prescription.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originalPrescription",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#original_prescription =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Event => {
                            if self.0.from_json {
                                if r#event.is_some() {
                                    return Err(serde::de::Error::duplicate_field("event"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitEvent>,
                                > = self.0.transmute();
                                r#event = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#event.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitEvent,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Payee => {
                            if r#payee.is_some() {
                                return Err(serde::de::Error::duplicate_field("payee"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::ExplanationOfBenefitPayee,
                            > = self.0.transmute();
                            r#payee = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Referral => {
                            if r#referral.is_some() {
                                return Err(serde::de::Error::duplicate_field("referral"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#referral = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Encounter => {
                            if self.0.from_json {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#encounter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Facility => {
                            if r#facility.is_some() {
                                return Err(serde::de::Error::duplicate_field("facility"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#facility = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Claim => {
                            if r#claim.is_some() {
                                return Err(serde::de::Error::duplicate_field("claim"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#claim = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ClaimResponse => {
                            if r#claim_response.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimResponse"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#claim_response = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Decision => {
                            if r#decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#decision = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::PreAuthRef => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#pre_auth_ref.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("preAuthRef");
                            }
                        }
                        Field::PreAuthRefPeriod => {
                            if self.0.from_json {
                                if r#pre_auth_ref_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preAuthRefPeriod",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Period>,
                                > = self.0.transmute();
                                r#pre_auth_ref_period =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#pre_auth_ref_period.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Period,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DiagnosisRelatedGroup => {
                            if r#diagnosis_related_group.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "diagnosisRelatedGroup",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#diagnosis_related_group =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::CareTeam => {
                            if self.0.from_json {
                                if r#care_team.is_some() {
                                    return Err(serde::de::Error::duplicate_field("careTeam"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::ExplanationOfBenefitCareTeam,
                                    >,
                                > = self.0.transmute();
                                r#care_team = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#care_team.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitCareTeam,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SupportingInfo => {
                            if self.0.from_json {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitSupportingInfo >> = self . 0 . transmute () ;
                                r#supporting_info =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#supporting_info.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitSupportingInfo > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Diagnosis => {
                            if self.0.from_json {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diagnosis"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitDiagnosis >> = self . 0 . transmute () ;
                                r#diagnosis = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#diagnosis.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitDiagnosis,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Procedure => {
                            if self.0.from_json {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitProcedure >> = self . 0 . transmute () ;
                                r#procedure = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#procedure.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitProcedure,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Precedence => {
                            if self.0.from_json {
                                let some = r#precedence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("precedence"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#precedence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("precedence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#precedence = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PrecedencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#precedence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_precedence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("precedence");
                            }
                        }
                        Field::Insurance => {
                            if self.0.from_json {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitInsurance >> = self . 0 . transmute () ;
                                r#insurance = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#insurance.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitInsurance,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Accident => {
                            if r#accident.is_some() {
                                return Err(serde::de::Error::duplicate_field("accident"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::ExplanationOfBenefitAccident,
                            > = self.0.transmute();
                            r#accident = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PatientPaid => {
                            if r#patient_paid.is_some() {
                                return Err(serde::de::Error::duplicate_field("patientPaid"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Money>,
                            > = self.0.transmute();
                            r#patient_paid = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Item => {
                            if self.0.from_json {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitItem>,
                                > = self.0.transmute();
                                r#item = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#item.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitItem,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AddItem => {
                            if self.0.from_json {
                                if r#add_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addItem"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitAddItem>,
                                > = self.0.transmute();
                                r#add_item = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#add_item.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitAddItem,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Adjudication => {
                            if self.0.from_json {
                                if r#adjudication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("adjudication"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication >> = self . 0 . transmute () ;
                                r#adjudication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#adjudication.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitItemAdjudication > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Total => {
                            if self.0.from_json {
                                if r#total.is_some() {
                                    return Err(serde::de::Error::duplicate_field("total"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::ExplanationOfBenefitTotal>,
                                > = self.0.transmute();
                                r#total = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#total.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitTotal,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Payment => {
                            if r#payment.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::ExplanationOfBenefitPayment,
                            > = self.0.transmute();
                            r#payment = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Attachment>,
                            > = self.0.transmute();
                            r#form = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ProcessNote => {
                            if self.0.from_json {
                                if r#process_note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processNote"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitProcessNote >> = self . 0 . transmute () ;
                                r#process_note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#process_note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::ExplanationOfBenefitProcessNote,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BenefitPeriod => {
                            if r#benefit_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#benefit_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::BenefitBalance => {
                            if self.0.from_json {
                                if r#benefit_balance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "benefitBalance",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitBenefitBalance >> = self . 0 . transmute () ;
                                r#benefit_balance =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#benefit_balance.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: ExplanationOfBenefitBenefitBalance > = self . 0 . transmute () ;
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
                Ok(ExplanationOfBenefit {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#trace_number: r#trace_number.unwrap_or(vec![]),
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
                    r#billable_period,
                    r#created: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#created.unwrap_or(Default::default())
                    } else {
                        r#created.ok_or(serde::de::Error::missing_field("created"))?
                    },
                    r#enterer,
                    r#insurer,
                    r#provider,
                    r#priority,
                    r#funds_reserve_requested,
                    r#funds_reserve,
                    r#related: r#related.unwrap_or(vec![]),
                    r#prescription,
                    r#original_prescription,
                    r#event: r#event.unwrap_or(vec![]),
                    r#payee,
                    r#referral,
                    r#encounter: r#encounter.unwrap_or(vec![]),
                    r#facility,
                    r#claim,
                    r#claim_response,
                    r#outcome: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#outcome.unwrap_or(Default::default())
                    } else {
                        r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                    },
                    r#decision,
                    r#disposition,
                    r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                    r#pre_auth_ref_period: r#pre_auth_ref_period.unwrap_or(vec![]),
                    r#diagnosis_related_group,
                    r#care_team: r#care_team.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                    r#procedure: r#procedure.unwrap_or(vec![]),
                    r#precedence,
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#accident,
                    r#patient_paid,
                    r#item: r#item.unwrap_or(vec![]),
                    r#add_item: r#add_item.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#total: r#total.unwrap_or(vec![]),
                    r#payment,
                    r#form_code,
                    r#form,
                    r#process_note: r#process_note.unwrap_or(vec![]),
                    r#benefit_period,
                    r#benefit_balance: r#benefit_balance.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ExplanationOfBenefit>>
{
    type Value = Box<ExplanationOfBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ExplanationOfBenefit>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ExplanationOfBenefit>>
{
    type Value = Vec<ExplanationOfBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ExplanationOfBenefit>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ExplanationOfBenefit>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ExplanationOfBenefit> =
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
