// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::FamilyMemberHistoryParticipant;
impl serde::ser::Serialize for SerializationContext<&FamilyMemberHistoryParticipant> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "FamilyMemberHistory.participant", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#function.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("function", ctx)));
        }
        if self.value.r#actor.id.as_deref() == Some("$invalid") {
            return missing_field_error("actor");
        } else {
            tri!(self.with_context(&self.value.r#actor, |ctx| state
                .serialize_entry("actor", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<FamilyMemberHistoryParticipant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<FamilyMemberHistoryParticipant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<FamilyMemberHistoryParticipant>
{
    type Value = FamilyMemberHistoryParticipant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<FamilyMemberHistoryParticipant>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = FamilyMemberHistoryParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistoryParticipant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<FamilyMemberHistoryParticipant, V::Error>
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
                    #[serde(rename = "function")]
                    Function,
                    #[serde(rename = "actor")]
                    Actor,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "function", "actor"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#function: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#function = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Actor => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#actor = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(FamilyMemberHistoryParticipant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#actor.unwrap_or(Default::default())
                    } else {
                        tri!(r#actor.ok_or(serde::de::Error::missing_field("actor")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<FamilyMemberHistoryParticipant>>
{
    type Value = Box<FamilyMemberHistoryParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<FamilyMemberHistoryParticipant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<FamilyMemberHistoryParticipant>>
{
    type Value = Vec<FamilyMemberHistoryParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<FamilyMemberHistoryParticipant>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<FamilyMemberHistoryParticipant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<FamilyMemberHistoryParticipant> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::FamilyMemberHistoryCondition;
impl serde::ser::Serialize for SerializationContext<&FamilyMemberHistoryCondition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "FamilyMemberHistory.condition", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            tri!(self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx)));
        }
        if let Some(some) = self.value.r#outcome.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("outcome", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#contributed_to_death.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("contributedToDeath", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_contributedToDeath", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#contributed_to_death.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("contributedToDeath", ctx)));
        }
        {
            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
            if let Some(some) = self.value.r#onset.as_ref() {
                match some {
                    _Enum::Age(ref value) => {
                        tri!(self.with_context(value, |ctx| state.serialize_entry("onsetAge", ctx)));
                    }
                    _Enum::Range(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("onsetRange", ctx)));
                    }
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("onsetPeriod", ctx)));
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("onsetString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_onsetString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("onsetString", ctx)));
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("onset is invalid")),
                }
            }
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<FamilyMemberHistoryCondition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<FamilyMemberHistoryCondition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<FamilyMemberHistoryCondition>
{
    type Value = FamilyMemberHistoryCondition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<FamilyMemberHistoryCondition>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = FamilyMemberHistoryCondition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistoryCondition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<FamilyMemberHistoryCondition, V::Error>
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
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "contributedToDeath")]
                    ContributedToDeath,
                    #[serde(rename = "_contributedToDeath")]
                    ContributedToDeathPrimitiveElement,
                    #[serde(rename = "onsetAge")]
                    OnsetAge,
                    #[serde(rename = "onsetRange")]
                    OnsetRange,
                    #[serde(rename = "onsetPeriod")]
                    OnsetPeriod,
                    #[serde(rename = "onsetString")]
                    OnsetString,
                    #[serde(rename = "_onsetString")]
                    OnsetStringPrimitiveElement,
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
                            "code",
                            "outcome",
                            "contributedToDeath",
                            "onsetAge",
                            "onsetRange",
                            "onsetPeriod",
                            "onsetString",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#outcome: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#contributed_to_death: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#onset: Option<
                    fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset,
                > = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#outcome = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ContributedToDeath => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#contributed_to_death.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributedToDeath",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#contributed_to_death.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributedToDeath",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#contributed_to_death =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ContributedToDeathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#contributed_to_death.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_contributedToDeath",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contributedToDeath");
                            }
                        }
                        Field::OnsetAge => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Age>,
                            > = self.0.transmute();
                            r#onset =
                                Some(_Enum::Age(tri!(map_access.next_value_seed(&mut *_context))));
                        }
                        Field::OnsetRange => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Range>,
                            > = self.0.transmute();
                            r#onset = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::OnsetPeriod => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#onset = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::OnsetString => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#onset.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "onsetString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("onset[x]"));
                                }
                            } else {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetString"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#onset = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::OnsetStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryConditionOnset as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#onset.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_onsetString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_onset[x]"));
                                }
                            } else {
                                return unknown_field_error("onsetString");
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(FamilyMemberHistoryCondition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        tri!(r#code.ok_or(serde::de::Error::missing_field("code")))
                    },
                    r#outcome,
                    r#contributed_to_death,
                    r#onset,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<FamilyMemberHistoryCondition>>
{
    type Value = Box<FamilyMemberHistoryCondition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<FamilyMemberHistoryCondition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<FamilyMemberHistoryCondition>>
{
    type Value = Vec<FamilyMemberHistoryCondition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<FamilyMemberHistoryCondition>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<FamilyMemberHistoryCondition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<FamilyMemberHistoryCondition> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedure;
impl serde::ser::Serialize for SerializationContext<&FamilyMemberHistoryProcedure> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "FamilyMemberHistory.procedure", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        } else {
            tri!(self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx)));
        }
        if let Some(some) = self.value.r#outcome.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("outcome", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#contributed_to_death.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("contributedToDeath", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_contributedToDeath", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#contributed_to_death.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("contributedToDeath", ctx)));
        }
        {
            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
            if let Some(some) = self.value.r#performed.as_ref() {
                match some {
                    _Enum::Age(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("performedAge", ctx)));
                    }
                    _Enum::Range(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("performedRange", ctx)));
                    }
                    _Enum::Period(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("performedPeriod", ctx)));
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("performedString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_performedString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("performedString", ctx)));
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("performedDateTime", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_performedDateTime", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("performedDateTime", ctx)));
                        }
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("performed is invalid"))
                    }
                }
            }
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<FamilyMemberHistoryProcedure>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<FamilyMemberHistoryProcedure>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<FamilyMemberHistoryProcedure>
{
    type Value = FamilyMemberHistoryProcedure;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<FamilyMemberHistoryProcedure>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = FamilyMemberHistoryProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistoryProcedure")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<FamilyMemberHistoryProcedure, V::Error>
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
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "contributedToDeath")]
                    ContributedToDeath,
                    #[serde(rename = "_contributedToDeath")]
                    ContributedToDeathPrimitiveElement,
                    #[serde(rename = "performedAge")]
                    PerformedAge,
                    #[serde(rename = "performedRange")]
                    PerformedRange,
                    #[serde(rename = "performedPeriod")]
                    PerformedPeriod,
                    #[serde(rename = "performedString")]
                    PerformedString,
                    #[serde(rename = "_performedString")]
                    PerformedStringPrimitiveElement,
                    #[serde(rename = "performedDateTime")]
                    PerformedDateTime,
                    #[serde(rename = "_performedDateTime")]
                    PerformedDateTimePrimitiveElement,
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
                            "code",
                            "outcome",
                            "contributedToDeath",
                            "performedAge",
                            "performedRange",
                            "performedPeriod",
                            "performedString",
                            "performedDateTime",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#outcome: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#contributed_to_death: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#performed: Option<
                    fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed,
                > = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#outcome = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ContributedToDeath => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#contributed_to_death.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributedToDeath",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#contributed_to_death.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contributedToDeath",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#contributed_to_death =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ContributedToDeathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#contributed_to_death.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_contributedToDeath",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contributedToDeath");
                            }
                        }
                        Field::PerformedAge => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Age>,
                            > = self.0.transmute();
                            r#performed =
                                Some(_Enum::Age(tri!(map_access.next_value_seed(&mut *_context))));
                        }
                        Field::PerformedRange => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Range>,
                            > = self.0.transmute();
                            r#performed = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::PerformedPeriod => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#performed = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::PerformedString => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#performed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performedString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("performed[x]"));
                                }
                            } else {
                                if r#performed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#performed = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::PerformedStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#performed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_performedString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_performed[x]"));
                                }
                            } else {
                                return unknown_field_error("performedString");
                            }
                        }
                        Field::PerformedDateTime => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#performed.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "performedDateTime",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("performed[x]"));
                                }
                            } else {
                                if r#performed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#performed = Some(_Enum::DateTime(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::PerformedDateTimePrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryProcedurePerformed as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#performed.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_performedDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_performed[x]"));
                                }
                            } else {
                                return unknown_field_error("performedDateTime");
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(FamilyMemberHistoryProcedure {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        tri!(r#code.ok_or(serde::de::Error::missing_field("code")))
                    },
                    r#outcome,
                    r#contributed_to_death,
                    r#performed,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<FamilyMemberHistoryProcedure>>
{
    type Value = Box<FamilyMemberHistoryProcedure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<FamilyMemberHistoryProcedure>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<FamilyMemberHistoryProcedure>>
{
    type Value = Vec<FamilyMemberHistoryProcedure>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<FamilyMemberHistoryProcedure>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<FamilyMemberHistoryProcedure>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<FamilyMemberHistoryProcedure> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::FamilyMemberHistory;
impl crate::Resource for FamilyMemberHistory {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&FamilyMemberHistory> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "FamilyMemberHistory", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "FamilyMemberHistory"));
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("id", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self
                        .with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("id", ctx)));
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("meta", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("implicitRules", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_implicitRules", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("language", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_language", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("language", ctx)));
        }
        if let Some(some) = self.value.r#text.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("text", ctx)));
        }
        if !self.value.r#contained.is_empty() {
            tri!(self.with_context(&self.value.r#contained, |ctx| state
                .serialize_entry("contained", ctx)));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if !self.value.r#identifier.is_empty() {
            tri!(self.with_context(&self.value.r#identifier, |ctx| state
                .serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#instantiates_canonical.is_empty() {
                let values = tri!(self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("instantiatesCanonical", &values));
                }
                let requires_elements = self
                    .value
                    .r#instantiates_canonical
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_canonical
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
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_instantiatesCanonical", ctx)));
                }
            }
        } else if !self.value.r#instantiates_canonical.is_empty() {
            tri!(
                self.with_context(&self.value.r#instantiates_canonical, |ctx| state
                    .serialize_entry("instantiatesCanonical", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#instantiates_uri.is_empty() {
                let values = tri!(self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("instantiatesUri", &values));
                }
                let requires_elements = self
                    .value
                    .r#instantiates_uri
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#instantiates_uri
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
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_instantiatesUri", ctx)));
                }
            }
        } else if !self.value.r#instantiates_uri.is_empty() {
            tri!(
                self.with_context(&self.value.r#instantiates_uri, |ctx| state
                    .serialize_entry("instantiatesUri", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("status", &some?));
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_status", ctx)));
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            tri!(self.with_context(&self.value.r#status, |ctx| state
                .serialize_entry("status", ctx)));
        }
        if let Some(some) = self.value.r#data_absent_reason.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("dataAbsentReason", ctx)));
        }
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        } else {
            tri!(self.with_context(&self.value.r#patient, |ctx| state
                .serialize_entry("patient", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("date", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_date", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#date.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("date", ctx)));
        }
        if !self.value.r#participant.is_empty() {
            tri!(self.with_context(&self.value.r#participant, |ctx| state
                .serialize_entry("participant", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("name", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_name", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("name", ctx)));
        }
        if self.value.r#relationship.id.as_deref() == Some("$invalid") {
            return missing_field_error("relationship");
        } else {
            tri!(self.with_context(&self.value.r#relationship, |ctx| state
                .serialize_entry("relationship", ctx)));
        }
        if let Some(some) = self.value.r#sex.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sex", ctx)));
        }
        {
            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
            if let Some(some) = self.value.r#born.as_ref() {
                match some {
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("bornPeriod", ctx)));
                    }
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("bornDate", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_bornDate", ctx)));
                            }
                        } else {
                            tri!(self
                                .with_context(value, |ctx| state.serialize_entry("bornDate", ctx)));
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("bornString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_bornString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("bornString", ctx)));
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("born is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r5::resources::FamilyMemberHistoryAge as _Enum;
            if let Some(some) = self.value.r#age.as_ref() {
                match some {
                    _Enum::Age(ref value) => {
                        tri!(self.with_context(value, |ctx| state.serialize_entry("ageAge", ctx)));
                    }
                    _Enum::Range(ref value) => {
                        tri!(self.with_context(value, |ctx| state.serialize_entry("ageRange", ctx)));
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("ageString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_ageString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("ageString", ctx)));
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("age is invalid")),
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#estimated_age.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("estimatedAge", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_estimatedAge", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#estimated_age.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("estimatedAge", ctx)));
        }
        {
            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
            if let Some(some) = self.value.r#deceased.as_ref() {
                match some {
                    _Enum::Boolean(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("deceasedBoolean", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_deceasedBoolean", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("deceasedBoolean", ctx)));
                        }
                    }
                    _Enum::Age(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("deceasedAge", ctx)));
                    }
                    _Enum::Range(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("deceasedRange", ctx)));
                    }
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("deceasedDate", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_deceasedDate", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("deceasedDate", ctx)));
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("deceasedString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_deceasedString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("deceasedString", ctx)));
                        }
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("deceased is invalid")),
                }
            }
        }
        if !self.value.r#reason.is_empty() {
            tri!(self.with_context(&self.value.r#reason, |ctx| state
                .serialize_entry("reason", ctx)));
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        if !self.value.r#condition.is_empty() {
            tri!(self.with_context(&self.value.r#condition, |ctx| state
                .serialize_entry("condition", ctx)));
        }
        if !self.value.r#procedure.is_empty() {
            tri!(self.with_context(&self.value.r#procedure, |ctx| state
                .serialize_entry("procedure", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<FamilyMemberHistory>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<FamilyMemberHistory>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<FamilyMemberHistory> {
    type Value = FamilyMemberHistory;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<FamilyMemberHistory> {
    type Value = FamilyMemberHistory;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<FamilyMemberHistory>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = FamilyMemberHistory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("FamilyMemberHistory")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<FamilyMemberHistory, V::Error>
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
                    #[serde(rename = "instantiatesCanonical")]
                    InstantiatesCanonical,
                    #[serde(rename = "_instantiatesCanonical")]
                    InstantiatesCanonicalPrimitiveElement,
                    #[serde(rename = "instantiatesUri")]
                    InstantiatesUri,
                    #[serde(rename = "_instantiatesUri")]
                    InstantiatesUriPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "dataAbsentReason")]
                    DataAbsentReason,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "participant")]
                    Participant,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "relationship")]
                    Relationship,
                    #[serde(rename = "sex")]
                    Sex,
                    #[serde(rename = "bornPeriod")]
                    BornPeriod,
                    #[serde(rename = "bornDate")]
                    BornDate,
                    #[serde(rename = "_bornDate")]
                    BornDatePrimitiveElement,
                    #[serde(rename = "bornString")]
                    BornString,
                    #[serde(rename = "_bornString")]
                    BornStringPrimitiveElement,
                    #[serde(rename = "ageAge")]
                    AgeAge,
                    #[serde(rename = "ageRange")]
                    AgeRange,
                    #[serde(rename = "ageString")]
                    AgeString,
                    #[serde(rename = "_ageString")]
                    AgeStringPrimitiveElement,
                    #[serde(rename = "estimatedAge")]
                    EstimatedAge,
                    #[serde(rename = "_estimatedAge")]
                    EstimatedAgePrimitiveElement,
                    #[serde(rename = "deceasedBoolean")]
                    DeceasedBoolean,
                    #[serde(rename = "_deceasedBoolean")]
                    DeceasedBooleanPrimitiveElement,
                    #[serde(rename = "deceasedAge")]
                    DeceasedAge,
                    #[serde(rename = "deceasedRange")]
                    DeceasedRange,
                    #[serde(rename = "deceasedDate")]
                    DeceasedDate,
                    #[serde(rename = "_deceasedDate")]
                    DeceasedDatePrimitiveElement,
                    #[serde(rename = "deceasedString")]
                    DeceasedString,
                    #[serde(rename = "_deceasedString")]
                    DeceasedStringPrimitiveElement,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "procedure")]
                    Procedure,
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
                            "instantiatesCanonical",
                            "instantiatesUri",
                            "status",
                            "dataAbsentReason",
                            "patient",
                            "date",
                            "participant",
                            "name",
                            "relationship",
                            "sex",
                            "bornPeriod",
                            "bornDate",
                            "bornString",
                            "ageAge",
                            "ageRange",
                            "ageString",
                            "estimatedAge",
                            "deceasedBoolean",
                            "deceasedAge",
                            "deceasedRange",
                            "deceasedDate",
                            "deceasedString",
                            "reason",
                            "note",
                            "condition",
                            "procedure",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#instantiates_canonical: Option<
                    Vec<fhirbolt_model::r5::types::Canonical>,
                > = None;
                let mut r#instantiates_uri: Option<Vec<fhirbolt_model::r5::types::Uri>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#data_absent_reason: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#patient: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#participant: Option<
                    Vec<fhirbolt_model::r5::resources::FamilyMemberHistoryParticipant>,
                > = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#relationship: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#sex: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#born: Option<fhirbolt_model::r5::resources::FamilyMemberHistoryBorn> =
                    None;
                let mut r#age: Option<fhirbolt_model::r5::resources::FamilyMemberHistoryAge> = None;
                let mut r#estimated_age: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#deceased: Option<
                    fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased,
                > = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#condition: Option<
                    Vec<fhirbolt_model::r5::resources::FamilyMemberHistoryCondition>,
                > = None;
                let mut r#procedure: Option<
                    Vec<fhirbolt_model::r5::resources::FamilyMemberHistoryProcedure>,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "FamilyMemberHistory" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"FamilyMemberHistory",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#id = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            r#meta = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesCanonical => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "instantiatesCanonical",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                        "_instantiatesCanonical",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("instantiatesCanonical");
                            }
                        }
                        Field::InstantiatesUri => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#instantiates_uri.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#instantiates_uri.get_or_insert(
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
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("instantiatesUri");
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::DataAbsentReason => {
                            if r#data_absent_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataAbsentReason"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#data_absent_reason =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Date => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#date = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("date");
                            }
                        }
                        Field::Participant => {
                            if self.0.from == crate::context::Format::Json {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: FamilyMemberHistoryParticipant >> = self . 0 . transmute () ;
                                r#participant =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#participant.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::FamilyMemberHistoryParticipant,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Relationship => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#relationship = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Sex => {
                            if r#sex.is_some() {
                                return Err(serde::de::Error::duplicate_field("sex"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#sex = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::BornPeriod => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
                            if r#born.is_some() {
                                return Err(serde::de::Error::duplicate_field("bornPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#born = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::BornDate => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#born.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("bornDate"));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("born[x]"));
                                }
                            } else {
                                if r#born.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bornDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#born = Some(_Enum::Date(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::BornDatePrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#born.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_bornDate"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_born[x]"));
                                }
                            } else {
                                return unknown_field_error("bornDate");
                            }
                        }
                        Field::BornString => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#born.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "bornString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("born[x]"));
                                }
                            } else {
                                if r#born.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bornString"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#born = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::BornStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryBorn as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#born.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_bornString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_born[x]"));
                                }
                            } else {
                                return unknown_field_error("bornString");
                            }
                        }
                        Field::AgeAge => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryAge as _Enum;
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Age>,
                            > = self.0.transmute();
                            r#age =
                                Some(_Enum::Age(tri!(map_access.next_value_seed(&mut *_context))));
                        }
                        Field::AgeRange => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryAge as _Enum;
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Range>,
                            > = self.0.transmute();
                            r#age = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::AgeString => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryAge as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#age.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("ageString"));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("age[x]"));
                                }
                            } else {
                                if r#age.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ageString"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#age = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::AgeStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryAge as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#age.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_ageString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_age[x]"));
                                }
                            } else {
                                return unknown_field_error("ageString");
                            }
                        }
                        Field::EstimatedAge => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#estimated_age.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("estimatedAge"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#estimated_age.is_some() {
                                    return Err(serde::de::Error::duplicate_field("estimatedAge"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#estimated_age =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::EstimatedAgePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#estimated_age.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_estimatedAge"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("estimatedAge");
                            }
                        }
                        Field::DeceasedBoolean => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deceasedBoolean",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("deceased[x]"));
                                }
                            } else {
                                if r#deceased.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deceasedBoolean",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#deceased = Some(_Enum::Boolean(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::DeceasedBooleanPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deceasedBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                                }
                            } else {
                                return unknown_field_error("deceasedBoolean");
                            }
                        }
                        Field::DeceasedAge => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if r#deceased.is_some() {
                                return Err(serde::de::Error::duplicate_field("deceasedAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Age>,
                            > = self.0.transmute();
                            r#deceased =
                                Some(_Enum::Age(tri!(map_access.next_value_seed(&mut *_context))));
                        }
                        Field::DeceasedRange => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if r#deceased.is_some() {
                                return Err(serde::de::Error::duplicate_field("deceasedRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Range>,
                            > = self.0.transmute();
                            r#deceased = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::DeceasedDate => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deceasedDate",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("deceased[x]"));
                                }
                            } else {
                                if r#deceased.is_some() {
                                    return Err(serde::de::Error::duplicate_field("deceasedDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#deceased = Some(_Enum::Date(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::DeceasedDatePrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deceasedDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                                }
                            } else {
                                return unknown_field_error("deceasedDate");
                            }
                        }
                        Field::DeceasedString => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deceasedString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("deceased[x]"));
                                }
                            } else {
                                if r#deceased.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "deceasedString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#deceased = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::DeceasedStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::FamilyMemberHistoryDeceased as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#deceased.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deceasedString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                                }
                            } else {
                                return unknown_field_error("deceasedString");
                            }
                        }
                        Field::Reason => {
                            if self.0.from == crate::context::Format::Json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#reason = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Condition => {
                            if self.0.from == crate::context::Format::Json {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::FamilyMemberHistoryCondition,
                                    >,
                                > = self.0.transmute();
                                r#condition =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::FamilyMemberHistoryCondition,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Procedure => {
                            if self.0.from == crate::context::Format::Json {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::FamilyMemberHistoryProcedure,
                                    >,
                                > = self.0.transmute();
                                r#procedure =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#procedure.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::FamilyMemberHistoryProcedure,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(FamilyMemberHistory {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        tri!(r#status.ok_or(serde::de::Error::missing_field("status")))
                    },
                    r#data_absent_reason,
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        tri!(r#patient.ok_or(serde::de::Error::missing_field("patient")))
                    },
                    r#date,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#name,
                    r#relationship: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#relationship.unwrap_or(Default::default())
                    } else {
                        tri!(r#relationship.ok_or(serde::de::Error::missing_field("relationship")))
                    },
                    r#sex,
                    r#born,
                    r#age,
                    r#estimated_age,
                    r#deceased,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#procedure: r#procedure.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<FamilyMemberHistory>>
{
    type Value = Box<FamilyMemberHistory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<FamilyMemberHistory>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<FamilyMemberHistory>>
{
    type Value = Vec<FamilyMemberHistory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<FamilyMemberHistory>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<FamilyMemberHistory>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<FamilyMemberHistory> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
