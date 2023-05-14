// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::Population;
impl serde::ser::Serialize for SerializationContext<&Population> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Population", field
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
            use fhirbolt_model::r4::types::PopulationAge as _Enum;
            if let Some(some) = self.value.r#age.as_ref() {
                match some {
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("ageRange", ctx))?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("ageCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("age is invalid")),
                }
            }
        }
        if let Some(some) = self.value.r#gender.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("gender", ctx))?;
        }
        if let Some(some) = self.value.r#race.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("race", ctx))?;
        }
        if let Some(some) = self.value.r#physiological_condition.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("physiologicalCondition", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Population>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Population>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Population> {
    type Value = Population;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Population>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Population;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Population")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Population, V::Error>
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
                    #[serde(rename = "ageRange")]
                    AgeRange,
                    #[serde(rename = "ageCodeableConcept")]
                    AgeCodeableConcept,
                    #[serde(rename = "gender")]
                    Gender,
                    #[serde(rename = "race")]
                    Race,
                    #[serde(rename = "physiologicalCondition")]
                    PhysiologicalCondition,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "ageRange",
                            "ageCodeableConcept",
                            "gender",
                            "race",
                            "physiologicalCondition",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#age: Option<fhirbolt_model::r4::types::PopulationAge> = None;
                let mut r#gender: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#race: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#physiological_condition: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
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
                        Field::AgeRange => {
                            use fhirbolt_model::r4::types::PopulationAge as _Enum;
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#age = Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::AgeCodeableConcept => {
                            use fhirbolt_model::r4::types::PopulationAge as _Enum;
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ageCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#age = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Gender => {
                            if r#gender.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#gender = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Race => {
                            if r#race.is_some() {
                                return Err(serde::de::Error::duplicate_field("race"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#race = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PhysiologicalCondition => {
                            if r#physiological_condition.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "physiologicalCondition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#physiological_condition =
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
                Ok(Population {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#age,
                    r#gender,
                    r#race,
                    r#physiological_condition,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Population>> {
    type Value = Box<Population>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Population>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Population>> {
    type Value = Vec<Population>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Population>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Population>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Population> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
