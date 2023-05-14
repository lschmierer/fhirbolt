// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::EncounterParticipant;
impl serde::ser::Serialize for SerializationContext<&EncounterParticipant> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter.participant", field
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
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        if let Some(some) = self.value.r#actor.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("actor", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<EncounterParticipant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<EncounterParticipant>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<EncounterParticipant> {
    type Value = EncounterParticipant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<EncounterParticipant>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = EncounterParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterParticipant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterParticipant, V::Error>
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
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "actor")]
                    Actor,
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
                            "period",
                            "actor",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#actor: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Actor => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#actor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(EncounterParticipant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#period,
                    r#actor,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<EncounterParticipant>>
{
    type Value = Box<EncounterParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<EncounterParticipant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<EncounterParticipant>>
{
    type Value = Vec<EncounterParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<EncounterParticipant>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<EncounterParticipant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<EncounterParticipant> =
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
use fhirbolt_model::r5::resources::EncounterReason;
impl serde::ser::Serialize for SerializationContext<&EncounterReason> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter.reason", field
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
        if !self.value.r#use.is_empty() {
            self.with_context(&self.value.r#use, |ctx| state.serialize_entry("use", ctx))?;
        }
        if !self.value.r#value.is_empty() {
            self.with_context(&self.value.r#value, |ctx| {
                state.serialize_entry("value", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<EncounterReason>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<EncounterReason>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<EncounterReason> {
    type Value = EncounterReason;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<EncounterReason>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = EncounterReason;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterReason")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterReason, V::Error>
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
                    #[serde(rename = "use")]
                    Use,
                    #[serde(rename = "value")]
                    Value,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "use", "value"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#use: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Use => {
                            if self.0.from == crate::context::Format::Json {
                                if r#use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#use = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#use.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Value => {
                            if self.0.from == crate::context::Format::Json {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("value"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#value = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#value.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
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
                Ok(EncounterReason {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#use: r#use.unwrap_or(vec![]),
                    r#value: r#value.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<EncounterReason>> {
    type Value = Box<EncounterReason>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<EncounterReason>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<EncounterReason>> {
    type Value = Vec<EncounterReason>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<EncounterReason>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<EncounterReason>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<EncounterReason> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::EncounterDiagnosis;
impl serde::ser::Serialize for SerializationContext<&EncounterDiagnosis> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter.diagnosis", field
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
        if !self.value.r#condition.is_empty() {
            self.with_context(&self.value.r#condition, |ctx| {
                state.serialize_entry("condition", ctx)
            })?;
        }
        if !self.value.r#use.is_empty() {
            self.with_context(&self.value.r#use, |ctx| state.serialize_entry("use", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<EncounterDiagnosis>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<EncounterDiagnosis>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<EncounterDiagnosis> {
    type Value = EncounterDiagnosis;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<EncounterDiagnosis>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = EncounterDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterDiagnosis")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterDiagnosis, V::Error>
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
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "use")]
                    Use,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "condition", "use"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#condition: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#use: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Condition => {
                            if self.0.from == crate::context::Format::Json {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#condition = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Use => {
                            if self.0.from == crate::context::Format::Json {
                                if r#use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#use = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#use.get_or_insert(Default::default());
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
                Ok(EncounterDiagnosis {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#use: r#use.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<EncounterDiagnosis>> {
    type Value = Box<EncounterDiagnosis>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<EncounterDiagnosis>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<EncounterDiagnosis>> {
    type Value = Vec<EncounterDiagnosis>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<EncounterDiagnosis>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<EncounterDiagnosis>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<EncounterDiagnosis> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::EncounterAdmission;
impl serde::ser::Serialize for SerializationContext<&EncounterAdmission> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter.admission", field
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
        if let Some(some) = self.value.r#pre_admission_identifier.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("preAdmissionIdentifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#origin.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("origin", ctx))?;
        }
        if let Some(some) = self.value.r#admit_source.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("admitSource", ctx))?;
        }
        if let Some(some) = self.value.r#re_admission.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reAdmission", ctx))?;
        }
        if let Some(some) = self.value.r#destination.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("destination", ctx))?;
        }
        if let Some(some) = self.value.r#discharge_disposition.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("dischargeDisposition", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<EncounterAdmission>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<EncounterAdmission>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<EncounterAdmission> {
    type Value = EncounterAdmission;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<EncounterAdmission>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = EncounterAdmission;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterAdmission")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterAdmission, V::Error>
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
                    #[serde(rename = "preAdmissionIdentifier")]
                    PreAdmissionIdentifier,
                    #[serde(rename = "origin")]
                    Origin,
                    #[serde(rename = "admitSource")]
                    AdmitSource,
                    #[serde(rename = "reAdmission")]
                    ReAdmission,
                    #[serde(rename = "destination")]
                    Destination,
                    #[serde(rename = "dischargeDisposition")]
                    DischargeDisposition,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "preAdmissionIdentifier",
                            "origin",
                            "admitSource",
                            "reAdmission",
                            "destination",
                            "dischargeDisposition",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#pre_admission_identifier: Option<
                    Box<fhirbolt_model::r5::types::Identifier>,
                > = None;
                let mut r#origin: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#admit_source: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#re_admission: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#destination: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#discharge_disposition: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreAdmissionIdentifier => {
                            if r#pre_admission_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "preAdmissionIdentifier",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#pre_admission_identifier =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Origin => {
                            if r#origin.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#origin = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AdmitSource => {
                            if r#admit_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("admitSource"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#admit_source = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ReAdmission => {
                            if r#re_admission.is_some() {
                                return Err(serde::de::Error::duplicate_field("reAdmission"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#re_admission = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Destination => {
                            if r#destination.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#destination = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::DischargeDisposition => {
                            if r#discharge_disposition.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "dischargeDisposition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#discharge_disposition =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(EncounterAdmission {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#pre_admission_identifier,
                    r#origin,
                    r#admit_source,
                    r#re_admission,
                    r#destination,
                    r#discharge_disposition,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<EncounterAdmission>> {
    type Value = Box<EncounterAdmission>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<EncounterAdmission>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<EncounterAdmission>> {
    type Value = Vec<EncounterAdmission>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<EncounterAdmission>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<EncounterAdmission>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<EncounterAdmission> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::EncounterLocation;
impl serde::ser::Serialize for SerializationContext<&EncounterLocation> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter.location", field
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
        if self.value.r#location.id.as_deref() == Some("$invalid") {
            return missing_field_error("location");
        } else {
            self.with_context(&self.value.r#location, |ctx| {
                state.serialize_entry("location", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("status", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        if let Some(some) = self.value.r#form.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("form", ctx))?;
        }
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<EncounterLocation>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<EncounterLocation>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<EncounterLocation> {
    type Value = EncounterLocation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<EncounterLocation>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = EncounterLocation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EncounterLocation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EncounterLocation, V::Error>
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
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "form")]
                    Form,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "location",
                            "status",
                            "form",
                            "period",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#form: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    fhirbolt_model::r5::types::Code,
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
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#form = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(EncounterLocation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#location: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#location.unwrap_or(Default::default())
                    } else {
                        r#location.ok_or(serde::de::Error::missing_field("location"))?
                    },
                    r#status,
                    r#form,
                    r#period,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<EncounterLocation>> {
    type Value = Box<EncounterLocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<EncounterLocation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<EncounterLocation>> {
    type Value = Vec<EncounterLocation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<EncounterLocation>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<EncounterLocation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<EncounterLocation> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::Encounter;
impl crate::Resource for Encounter {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&Encounter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Encounter", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Encounter")?;
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
        if !self.value.r#class.is_empty() {
            self.with_context(&self.value.r#class, |ctx| {
                state.serialize_entry("class", ctx)
            })?;
        }
        if let Some(some) = self.value.r#priority.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("priority", ctx))?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#service_type.is_empty() {
            self.with_context(&self.value.r#service_type, |ctx| {
                state.serialize_entry("serviceType", ctx)
            })?;
        }
        if let Some(some) = self.value.r#subject.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subject", ctx))?;
        }
        if let Some(some) = self.value.r#subject_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subjectStatus", ctx))?;
        }
        if !self.value.r#episode_of_care.is_empty() {
            self.with_context(&self.value.r#episode_of_care, |ctx| {
                state.serialize_entry("episodeOfCare", ctx)
            })?;
        }
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if !self.value.r#care_team.is_empty() {
            self.with_context(&self.value.r#care_team, |ctx| {
                state.serialize_entry("careTeam", ctx)
            })?;
        }
        if let Some(some) = self.value.r#part_of.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("partOf", ctx))?;
        }
        if let Some(some) = self.value.r#service_provider.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("serviceProvider", ctx))?;
        }
        if !self.value.r#participant.is_empty() {
            self.with_context(&self.value.r#participant, |ctx| {
                state.serialize_entry("participant", ctx)
            })?;
        }
        if !self.value.r#appointment.is_empty() {
            self.with_context(&self.value.r#appointment, |ctx| {
                state.serialize_entry("appointment", ctx)
            })?;
        }
        if !self.value.r#virtual_service.is_empty() {
            self.with_context(&self.value.r#virtual_service, |ctx| {
                state.serialize_entry("virtualService", ctx)
            })?;
        }
        if let Some(some) = self.value.r#actual_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("actualPeriod", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#planned_start_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("plannedStartDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_plannedStartDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#planned_start_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("plannedStartDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#planned_end_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("plannedEndDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_plannedEndDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#planned_end_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("plannedEndDate", ctx))?;
        }
        if let Some(some) = self.value.r#length.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("length", ctx))?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if !self.value.r#diagnosis.is_empty() {
            self.with_context(&self.value.r#diagnosis, |ctx| {
                state.serialize_entry("diagnosis", ctx)
            })?;
        }
        if !self.value.r#account.is_empty() {
            self.with_context(&self.value.r#account, |ctx| {
                state.serialize_entry("account", ctx)
            })?;
        }
        if !self.value.r#diet_preference.is_empty() {
            self.with_context(&self.value.r#diet_preference, |ctx| {
                state.serialize_entry("dietPreference", ctx)
            })?;
        }
        if !self.value.r#special_arrangement.is_empty() {
            self.with_context(&self.value.r#special_arrangement, |ctx| {
                state.serialize_entry("specialArrangement", ctx)
            })?;
        }
        if !self.value.r#special_courtesy.is_empty() {
            self.with_context(&self.value.r#special_courtesy, |ctx| {
                state.serialize_entry("specialCourtesy", ctx)
            })?;
        }
        if let Some(some) = self.value.r#admission.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("admission", ctx))?;
        }
        if !self.value.r#location.is_empty() {
            self.with_context(&self.value.r#location, |ctx| {
                state.serialize_entry("location", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Encounter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Encounter>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Encounter> {
    type Value = Encounter;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Encounter> {
    type Value = Encounter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Encounter>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Encounter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Encounter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Encounter, V::Error>
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
                    #[serde(rename = "class")]
                    Class,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "serviceType")]
                    ServiceType,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "subjectStatus")]
                    SubjectStatus,
                    #[serde(rename = "episodeOfCare")]
                    EpisodeOfCare,
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "careTeam")]
                    CareTeam,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "serviceProvider")]
                    ServiceProvider,
                    #[serde(rename = "participant")]
                    Participant,
                    #[serde(rename = "appointment")]
                    Appointment,
                    #[serde(rename = "virtualService")]
                    VirtualService,
                    #[serde(rename = "actualPeriod")]
                    ActualPeriod,
                    #[serde(rename = "plannedStartDate")]
                    PlannedStartDate,
                    #[serde(rename = "_plannedStartDate")]
                    PlannedStartDatePrimitiveElement,
                    #[serde(rename = "plannedEndDate")]
                    PlannedEndDate,
                    #[serde(rename = "_plannedEndDate")]
                    PlannedEndDatePrimitiveElement,
                    #[serde(rename = "length")]
                    Length,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "diagnosis")]
                    Diagnosis,
                    #[serde(rename = "account")]
                    Account,
                    #[serde(rename = "dietPreference")]
                    DietPreference,
                    #[serde(rename = "specialArrangement")]
                    SpecialArrangement,
                    #[serde(rename = "specialCourtesy")]
                    SpecialCourtesy,
                    #[serde(rename = "admission")]
                    Admission,
                    #[serde(rename = "location")]
                    Location,
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
                            "class",
                            "priority",
                            "type",
                            "serviceType",
                            "subject",
                            "subjectStatus",
                            "episodeOfCare",
                            "basedOn",
                            "careTeam",
                            "partOf",
                            "serviceProvider",
                            "participant",
                            "appointment",
                            "virtualService",
                            "actualPeriod",
                            "plannedStartDate",
                            "plannedEndDate",
                            "length",
                            "reason",
                            "diagnosis",
                            "account",
                            "dietPreference",
                            "specialArrangement",
                            "specialCourtesy",
                            "admission",
                            "location",
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
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#class: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#priority: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#service_type: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#subject_status: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#episode_of_care: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#based_on: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#care_team: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#part_of: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#service_provider: Option<Box<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#participant: Option<
                    Vec<fhirbolt_model::r5::resources::EncounterParticipant>,
                > = None;
                let mut r#appointment: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#virtual_service: Option<
                    Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
                > = None;
                let mut r#actual_period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#planned_start_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#planned_end_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#length: Option<Box<fhirbolt_model::r5::types::Duration>> = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::resources::EncounterReason>> =
                    None;
                let mut r#diagnosis: Option<
                    Vec<fhirbolt_model::r5::resources::EncounterDiagnosis>,
                > = None;
                let mut r#account: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#diet_preference: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#special_arrangement: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#special_courtesy: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#admission: Option<fhirbolt_model::r5::resources::EncounterAdmission> =
                    None;
                let mut r#location: Option<Vec<fhirbolt_model::r5::resources::EncounterLocation>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Encounter" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Encounter",
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
                                    Box<fhirbolt_model::r5::types::Id>,
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
                                Box<fhirbolt_model::r5::types::Meta>,
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
                                    fhirbolt_model::r5::types::Uri,
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
                                    fhirbolt_model::r5::types::Code,
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
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r5::types::Code,
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
                        Field::Class => {
                            if self.0.from == crate::context::Format::Json {
                                if r#class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("class"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#class = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#class.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
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
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
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
                        Field::ServiceType => {
                            if self.0.from == crate::context::Format::Json {
                                if r#service_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serviceType"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#service_type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#service_type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SubjectStatus => {
                            if r#subject_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectStatus"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#subject_status = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::EpisodeOfCare => {
                            if self.0.from == crate::context::Format::Json {
                                if r#episode_of_care.is_some() {
                                    return Err(serde::de::Error::duplicate_field("episodeOfCare"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#episode_of_care =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#episode_of_care.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BasedOn => {
                            if self.0.from == crate::context::Format::Json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#based_on = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CareTeam => {
                            if self.0.from == crate::context::Format::Json {
                                if r#care_team.is_some() {
                                    return Err(serde::de::Error::duplicate_field("careTeam"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#care_team = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#care_team.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PartOf => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#part_of = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ServiceProvider => {
                            if r#service_provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceProvider"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#service_provider = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Participant => {
                            if self.0.from == crate::context::Format::Json {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::EncounterParticipant>,
                                > = self.0.transmute();
                                r#participant = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#participant.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::EncounterParticipant,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Appointment => {
                            if self.0.from == crate::context::Format::Json {
                                if r#appointment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("appointment"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#appointment = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#appointment.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VirtualService => {
                            if self.0.from == crate::context::Format::Json {
                                if r#virtual_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "virtualService",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
                                > = self.0.transmute();
                                r#virtual_service =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#virtual_service.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::VirtualServiceDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ActualPeriod => {
                            if r#actual_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("actualPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#actual_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PlannedStartDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#planned_start_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "plannedStartDate",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#planned_start_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "plannedStartDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#planned_start_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PlannedStartDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#planned_start_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_plannedStartDate",
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
                                return unknown_field_error("plannedStartDate");
                            }
                        }
                        Field::PlannedEndDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#planned_end_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "plannedEndDate",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#planned_end_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "plannedEndDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#planned_end_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PlannedEndDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#planned_end_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_plannedEndDate",
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
                                return unknown_field_error("plannedEndDate");
                            }
                        }
                        Field::Length => {
                            if r#length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Duration>,
                            > = self.0.transmute();
                            r#length = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Reason => {
                            if self.0.from == crate::context::Format::Json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::EncounterReason>,
                                > = self.0.transmute();
                                r#reason = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::EncounterReason,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Diagnosis => {
                            if self.0.from == crate::context::Format::Json {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diagnosis"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::EncounterDiagnosis>,
                                > = self.0.transmute();
                                r#diagnosis = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#diagnosis.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::EncounterDiagnosis,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Account => {
                            if self.0.from == crate::context::Format::Json {
                                if r#account.is_some() {
                                    return Err(serde::de::Error::duplicate_field("account"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#account = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#account.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DietPreference => {
                            if self.0.from == crate::context::Format::Json {
                                if r#diet_preference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dietPreference",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#diet_preference =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#diet_preference.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SpecialArrangement => {
                            if self.0.from == crate::context::Format::Json {
                                if r#special_arrangement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialArrangement",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#special_arrangement =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#special_arrangement.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SpecialCourtesy => {
                            if self.0.from == crate::context::Format::Json {
                                if r#special_courtesy.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialCourtesy",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#special_courtesy =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#special_courtesy.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Admission => {
                            if r#admission.is_some() {
                                return Err(serde::de::Error::duplicate_field("admission"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::EncounterAdmission,
                            > = self.0.transmute();
                            r#admission = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Location => {
                            if self.0.from == crate::context::Format::Json {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::EncounterLocation>,
                                > = self.0.transmute();
                                r#location = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#location.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::EncounterLocation,
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
                Ok(Encounter {
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
                    r#class: r#class.unwrap_or(vec![]),
                    r#priority,
                    r#type: r#type.unwrap_or(vec![]),
                    r#service_type: r#service_type.unwrap_or(vec![]),
                    r#subject,
                    r#subject_status,
                    r#episode_of_care: r#episode_of_care.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#care_team: r#care_team.unwrap_or(vec![]),
                    r#part_of,
                    r#service_provider,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#appointment: r#appointment.unwrap_or(vec![]),
                    r#virtual_service: r#virtual_service.unwrap_or(vec![]),
                    r#actual_period,
                    r#planned_start_date,
                    r#planned_end_date,
                    r#length,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                    r#account: r#account.unwrap_or(vec![]),
                    r#diet_preference: r#diet_preference.unwrap_or(vec![]),
                    r#special_arrangement: r#special_arrangement.unwrap_or(vec![]),
                    r#special_courtesy: r#special_courtesy.unwrap_or(vec![]),
                    r#admission,
                    r#location: r#location.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Encounter>> {
    type Value = Box<Encounter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Encounter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Encounter>> {
    type Value = Vec<Encounter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Encounter>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Encounter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Encounter> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
