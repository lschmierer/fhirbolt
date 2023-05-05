// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::MedicationDispensePerformer;
impl serde::ser::Serialize for SerializationContext<&MedicationDispensePerformer> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationDispense.performer", field
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
        if let Some(some) = self.value.r#function.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("function", ctx))?;
        }
        if self.value.r#actor.id.as_deref() == Some("$invalid") {
            return missing_field_error("actor");
        } else {
            self.with_context(&self.value.r#actor, |ctx| {
                state.serialize_entry("actor", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationDispensePerformer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationDispensePerformer>> {
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
    for &mut DeserializationContext<MedicationDispensePerformer>
{
    type Value = MedicationDispensePerformer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationDispensePerformer>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationDispensePerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispensePerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationDispensePerformer, V::Error>
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
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#function = Some(map_access.next_value_seed(&mut *_context)?);
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
                Ok(MedicationDispensePerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#actor.unwrap_or(Default::default())
                    } else {
                        r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicationDispensePerformer>>
{
    type Value = Box<MedicationDispensePerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationDispensePerformer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicationDispensePerformer>>
{
    type Value = Vec<MedicationDispensePerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationDispensePerformer>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationDispensePerformer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationDispensePerformer> =
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
use fhirbolt_model::r5::resources::MedicationDispenseSubstitution;
impl serde::ser::Serialize for SerializationContext<&MedicationDispenseSubstitution> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationDispense.substitution", field
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
            if self.value.r#was_substituted.id.as_deref() == Some("$invalid") {
                return missing_field_error("wasSubstituted");
            }
            if let Some(some) = self.value.r#was_substituted.value.as_ref().map(Ok) {
                state.serialize_entry("wasSubstituted", &some?)?;
            }
            if self.value.r#was_substituted.id.is_some()
                || !self.value.r#was_substituted.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#was_substituted.id.as_ref(),
                    extension: &self.value.r#was_substituted.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_wasSubstituted", ctx)
                })?;
            }
        } else if self.value.r#was_substituted.id.as_deref() == Some("$invalid") {
            return missing_field_error("wasSubstituted");
        } else {
            self.with_context(&self.value.r#was_substituted, |ctx| {
                state.serialize_entry("wasSubstituted", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if let Some(some) = self.value.r#responsible_party.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responsibleParty", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationDispenseSubstitution>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationDispenseSubstitution>> {
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
    for &mut DeserializationContext<MedicationDispenseSubstitution>
{
    type Value = MedicationDispenseSubstitution;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationDispenseSubstitution>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationDispenseSubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispenseSubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationDispenseSubstitution, V::Error>
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
                    #[serde(rename = "wasSubstituted")]
                    WasSubstituted,
                    #[serde(rename = "_wasSubstituted")]
                    WasSubstitutedPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "responsibleParty")]
                    ResponsibleParty,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "wasSubstituted",
                            "type",
                            "reason",
                            "responsibleParty",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#was_substituted: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#responsible_party: Option<Box<fhirbolt_model::r5::types::Reference>> =
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
                        Field::WasSubstituted => {
                            if self.0.from_json {
                                let some = r#was_substituted.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "wasSubstituted",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#was_substituted.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "wasSubstituted",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#was_substituted =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WasSubstitutedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#was_substituted.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_wasSubstituted",
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
                                return unknown_field_error("wasSubstituted");
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
                        Field::ResponsibleParty => {
                            if r#responsible_party.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsibleParty"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#responsible_party = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicationDispenseSubstitution {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#was_substituted: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#was_substituted.unwrap_or(Default::default())
                    } else {
                        r#was_substituted
                            .ok_or(serde::de::Error::missing_field("wasSubstituted"))?
                    },
                    r#type,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#responsible_party,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicationDispenseSubstitution>>
{
    type Value = Box<MedicationDispenseSubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationDispenseSubstitution>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicationDispenseSubstitution>>
{
    type Value = Vec<MedicationDispenseSubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationDispenseSubstitution>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationDispenseSubstitution>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationDispenseSubstitution> =
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
use fhirbolt_model::r5::resources::MedicationDispense;
impl crate::Resource for MedicationDispense {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&MedicationDispense> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationDispense", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationDispense")?;
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
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if !self.value.r#part_of.is_empty() {
            self.with_context(&self.value.r#part_of, |ctx| {
                state.serialize_entry("partOf", ctx)
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
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if let Some(some) = self.value.r#not_performed_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("notPerformedReason", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status_changed.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("statusChanged", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusChanged", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status_changed.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("statusChanged", ctx))?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if self.value.r#medication.id.as_deref() == Some("$invalid") {
            return missing_field_error("medication");
        } else {
            self.with_context(&self.value.r#medication, |ctx| {
                state.serialize_entry("medication", ctx)
            })?;
        }
        if self.value.r#subject.id.as_deref() == Some("$invalid") {
            return missing_field_error("subject");
        } else {
            self.with_context(&self.value.r#subject, |ctx| {
                state.serialize_entry("subject", ctx)
            })?;
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if !self.value.r#supporting_information.is_empty() {
            self.with_context(&self.value.r#supporting_information, |ctx| {
                state.serialize_entry("supportingInformation", ctx)
            })?;
        }
        if !self.value.r#performer.is_empty() {
            self.with_context(&self.value.r#performer, |ctx| {
                state.serialize_entry("performer", ctx)
            })?;
        }
        if let Some(some) = self.value.r#location.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("location", ctx))?;
        }
        if !self.value.r#authorizing_prescription.is_empty() {
            self.with_context(&self.value.r#authorizing_prescription, |ctx| {
                state.serialize_entry("authorizingPrescription", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#days_supply.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("daysSupply", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#recorded.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("recorded", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_recorded", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#recorded.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("recorded", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#when_prepared.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("whenPrepared", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_whenPrepared", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#when_prepared.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("whenPrepared", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#when_handed_over.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("whenHandedOver", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_whenHandedOver", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#when_handed_over.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("whenHandedOver", ctx))?;
        }
        if let Some(some) = self.value.r#destination.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("destination", ctx))?;
        }
        if !self.value.r#receiver.is_empty() {
            self.with_context(&self.value.r#receiver, |ctx| {
                state.serialize_entry("receiver", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#rendered_dosage_instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("renderedDosageInstruction", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_renderedDosageInstruction", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#rendered_dosage_instruction.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("renderedDosageInstruction", ctx)
            })?;
        }
        if !self.value.r#dosage_instruction.is_empty() {
            self.with_context(&self.value.r#dosage_instruction, |ctx| {
                state.serialize_entry("dosageInstruction", ctx)
            })?;
        }
        if let Some(some) = self.value.r#substitution.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("substitution", ctx))?;
        }
        if !self.value.r#event_history.is_empty() {
            self.with_context(&self.value.r#event_history, |ctx| {
                state.serialize_entry("eventHistory", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationDispense>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationDispense>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<MedicationDispense> {
    type Value = MedicationDispense;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<MedicationDispense> {
    type Value = MedicationDispense;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationDispense>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationDispense;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationDispense")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationDispense, V::Error>
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
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "notPerformedReason")]
                    NotPerformedReason,
                    #[serde(rename = "statusChanged")]
                    StatusChanged,
                    #[serde(rename = "_statusChanged")]
                    StatusChangedPrimitiveElement,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "medication")]
                    Medication,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "supportingInformation")]
                    SupportingInformation,
                    #[serde(rename = "performer")]
                    Performer,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "authorizingPrescription")]
                    AuthorizingPrescription,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "daysSupply")]
                    DaysSupply,
                    #[serde(rename = "recorded")]
                    Recorded,
                    #[serde(rename = "_recorded")]
                    RecordedPrimitiveElement,
                    #[serde(rename = "whenPrepared")]
                    WhenPrepared,
                    #[serde(rename = "_whenPrepared")]
                    WhenPreparedPrimitiveElement,
                    #[serde(rename = "whenHandedOver")]
                    WhenHandedOver,
                    #[serde(rename = "_whenHandedOver")]
                    WhenHandedOverPrimitiveElement,
                    #[serde(rename = "destination")]
                    Destination,
                    #[serde(rename = "receiver")]
                    Receiver,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "renderedDosageInstruction")]
                    RenderedDosageInstruction,
                    #[serde(rename = "_renderedDosageInstruction")]
                    RenderedDosageInstructionPrimitiveElement,
                    #[serde(rename = "dosageInstruction")]
                    DosageInstruction,
                    #[serde(rename = "substitution")]
                    Substitution,
                    #[serde(rename = "eventHistory")]
                    EventHistory,
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
                            "basedOn",
                            "partOf",
                            "status",
                            "notPerformedReason",
                            "statusChanged",
                            "category",
                            "medication",
                            "subject",
                            "encounter",
                            "supportingInformation",
                            "performer",
                            "location",
                            "authorizingPrescription",
                            "type",
                            "quantity",
                            "daysSupply",
                            "recorded",
                            "whenPrepared",
                            "whenHandedOver",
                            "destination",
                            "receiver",
                            "note",
                            "renderedDosageInstruction",
                            "dosageInstruction",
                            "substitution",
                            "eventHistory",
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
                let mut r#based_on: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#part_of: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#not_performed_reason: Option<
                    Box<fhirbolt_model::r5::types::CodeableReference>,
                > = None;
                let mut r#status_changed: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#category: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#medication: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#supporting_information: Option<
                    Vec<fhirbolt_model::r5::types::Reference>,
                > = None;
                let mut r#performer: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationDispensePerformer>,
                > = None;
                let mut r#location: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#authorizing_prescription: Option<
                    Vec<fhirbolt_model::r5::types::Reference>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#days_supply: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#recorded: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#when_prepared: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#when_handed_over: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#destination: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#receiver: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#rendered_dosage_instruction: Option<fhirbolt_model::r5::types::Markdown> =
                    None;
                let mut r#dosage_instruction: Option<Vec<fhirbolt_model::r5::types::Dosage>> = None;
                let mut r#substitution: Option<
                    fhirbolt_model::r5::resources::MedicationDispenseSubstitution,
                > = None;
                let mut r#event_history: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationDispense" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationDispense",
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
                        Field::BasedOn => {
                            if self.0.from_json {
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
                        Field::PartOf => {
                            if self.0.from_json {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#part_of = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#part_of.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
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
                        Field::NotPerformedReason => {
                            if r#not_performed_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "notPerformedReason",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#not_performed_reason =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::StatusChanged => {
                            if self.0.from_json {
                                let some = r#status_changed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusChanged"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status_changed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusChanged"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#status_changed =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusChangedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status_changed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_statusChanged",
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
                                return unknown_field_error("statusChanged");
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Medication => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field("medication"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#medication = Some(map_access.next_value_seed(&mut *_context)?);
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
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SupportingInformation => {
                            if self.0.from_json {
                                if r#supporting_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInformation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#supporting_information =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#supporting_information.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Performer => {
                            if self.0.from_json {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::MedicationDispensePerformer>,
                                > = self.0.transmute();
                                r#performer = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#performer.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::MedicationDispensePerformer,
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
                        Field::AuthorizingPrescription => {
                            if self.0.from_json {
                                if r#authorizing_prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizingPrescription",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#authorizing_prescription =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#authorizing_prescription.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::DaysSupply => {
                            if r#days_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field("daysSupply"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#days_supply = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Recorded => {
                            if self.0.from_json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#recorded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#recorded = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RecordedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recorded"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recorded");
                            }
                        }
                        Field::WhenPrepared => {
                            if self.0.from_json {
                                let some = r#when_prepared.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whenPrepared"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#when_prepared.is_some() {
                                    return Err(serde::de::Error::duplicate_field("whenPrepared"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#when_prepared = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WhenPreparedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#when_prepared.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_whenPrepared"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("whenPrepared");
                            }
                        }
                        Field::WhenHandedOver => {
                            if self.0.from_json {
                                let some = r#when_handed_over.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "whenHandedOver",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#when_handed_over.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "whenHandedOver",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#when_handed_over =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WhenHandedOverPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#when_handed_over.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_whenHandedOver",
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
                                return unknown_field_error("whenHandedOver");
                            }
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
                        Field::Receiver => {
                            if self.0.from_json {
                                if r#receiver.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receiver"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#receiver = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#receiver.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RenderedDosageInstruction => {
                            if self.0.from_json {
                                let some =
                                    r#rendered_dosage_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "renderedDosageInstruction",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#rendered_dosage_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "renderedDosageInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#rendered_dosage_instruction =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RenderedDosageInstructionPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#rendered_dosage_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_renderedDosageInstruction",
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
                                return unknown_field_error("renderedDosageInstruction");
                            }
                        }
                        Field::DosageInstruction => {
                            if self.0.from_json {
                                if r#dosage_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dosageInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Dosage>,
                                > = self.0.transmute();
                                r#dosage_instruction =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#dosage_instruction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Dosage,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Substitution => {
                            if r#substitution.is_some() {
                                return Err(serde::de::Error::duplicate_field("substitution"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::MedicationDispenseSubstitution,
                            > = self.0.transmute();
                            r#substitution = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::EventHistory => {
                            if self.0.from_json {
                                if r#event_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventHistory"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#event_history = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#event_history.get_or_insert(Default::default());
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
                Ok(MedicationDispense {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#not_performed_reason,
                    r#status_changed,
                    r#category: r#category.unwrap_or(vec![]),
                    r#medication: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#medication.unwrap_or(Default::default())
                    } else {
                        r#medication.ok_or(serde::de::Error::missing_field("medication"))?
                    },
                    r#subject: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#subject.unwrap_or(Default::default())
                    } else {
                        r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                    },
                    r#encounter,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#location,
                    r#authorizing_prescription: r#authorizing_prescription.unwrap_or(vec![]),
                    r#type,
                    r#quantity,
                    r#days_supply,
                    r#recorded,
                    r#when_prepared,
                    r#when_handed_over,
                    r#destination,
                    r#receiver: r#receiver.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#rendered_dosage_instruction,
                    r#dosage_instruction: r#dosage_instruction.unwrap_or(vec![]),
                    r#substitution,
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<MedicationDispense>> {
    type Value = Box<MedicationDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationDispense>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<MedicationDispense>> {
    type Value = Vec<MedicationDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationDispense>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationDispense>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationDispense> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
