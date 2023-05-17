// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::AppointmentParticipant;
impl serde::ser::Serialize for SerializationContext<&AppointmentParticipant> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment.participant", field
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
        if !self.value.r#type.is_empty() {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        if let Some(some) = self.value.r#period.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("period", ctx)));
        }
        if let Some(some) = self.value.r#actor.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("actor", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#required.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("required", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_required", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#required.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("required", ctx)));
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
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AppointmentParticipant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AppointmentParticipant>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AppointmentParticipant> {
    type Value = AppointmentParticipant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AppointmentParticipant>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AppointmentParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentParticipant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AppointmentParticipant, V::Error>
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
                    #[serde(rename = "required")]
                    Required,
                    #[serde(rename = "_required")]
                    RequiredPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
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
                            "required",
                            "status",
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
                let mut r#required: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
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
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#period = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                        Field::Required => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("required"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#required.is_some() {
                                    return Err(serde::de::Error::duplicate_field("required"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#required = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RequiredPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_required"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("required");
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(AppointmentParticipant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#period,
                    r#actor,
                    r#required,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        tri!(r#status.ok_or(serde::de::Error::missing_field("status")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AppointmentParticipant>>
{
    type Value = Box<AppointmentParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AppointmentParticipant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AppointmentParticipant>>
{
    type Value = Vec<AppointmentParticipant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AppointmentParticipant>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AppointmentParticipant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AppointmentParticipant> =
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
use fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateWeeklyTemplate;
impl serde::ser::Serialize for SerializationContext<&AppointmentRecurrenceTemplateWeeklyTemplate> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment.recurrenceTemplate.weeklyTemplate", field
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#monday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("monday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_monday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#monday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("monday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#tuesday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("tuesday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_tuesday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#tuesday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("tuesday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#wednesday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("wednesday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_wednesday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#wednesday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("wednesday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#thursday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("thursday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_thursday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#thursday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("thursday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#friday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("friday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_friday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#friday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("friday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#saturday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("saturday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_saturday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#saturday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("saturday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#sunday.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("sunday", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_sunday", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#sunday.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sunday", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#week_interval.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("weekInterval", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_weekInterval", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#week_interval.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("weekInterval", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<AppointmentRecurrenceTemplateWeeklyTemplate>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<AppointmentRecurrenceTemplateWeeklyTemplate>>
{
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
    for &mut DeserializationContext<AppointmentRecurrenceTemplateWeeklyTemplate>
{
    type Value = AppointmentRecurrenceTemplateWeeklyTemplate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<AppointmentRecurrenceTemplateWeeklyTemplate>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AppointmentRecurrenceTemplateWeeklyTemplate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentRecurrenceTemplateWeeklyTemplate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AppointmentRecurrenceTemplateWeeklyTemplate, V::Error>
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
                    #[serde(rename = "monday")]
                    Monday,
                    #[serde(rename = "_monday")]
                    MondayPrimitiveElement,
                    #[serde(rename = "tuesday")]
                    Tuesday,
                    #[serde(rename = "_tuesday")]
                    TuesdayPrimitiveElement,
                    #[serde(rename = "wednesday")]
                    Wednesday,
                    #[serde(rename = "_wednesday")]
                    WednesdayPrimitiveElement,
                    #[serde(rename = "thursday")]
                    Thursday,
                    #[serde(rename = "_thursday")]
                    ThursdayPrimitiveElement,
                    #[serde(rename = "friday")]
                    Friday,
                    #[serde(rename = "_friday")]
                    FridayPrimitiveElement,
                    #[serde(rename = "saturday")]
                    Saturday,
                    #[serde(rename = "_saturday")]
                    SaturdayPrimitiveElement,
                    #[serde(rename = "sunday")]
                    Sunday,
                    #[serde(rename = "_sunday")]
                    SundayPrimitiveElement,
                    #[serde(rename = "weekInterval")]
                    WeekInterval,
                    #[serde(rename = "_weekInterval")]
                    WeekIntervalPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "monday",
                            "tuesday",
                            "wednesday",
                            "thursday",
                            "friday",
                            "saturday",
                            "sunday",
                            "weekInterval",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#monday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#tuesday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#wednesday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#thursday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#friday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#saturday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#sunday: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#week_interval: Option<fhirbolt_model::r5::types::PositiveInt> = None;
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
                        Field::Monday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#monday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#monday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#monday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MondayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#monday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_monday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("monday");
                            }
                        }
                        Field::Tuesday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#tuesday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("tuesday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#tuesday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("tuesday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#tuesday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::TuesdayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#tuesday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_tuesday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("tuesday");
                            }
                        }
                        Field::Wednesday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#wednesday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("wednesday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#wednesday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("wednesday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#wednesday =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::WednesdayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#wednesday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_wednesday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("wednesday");
                            }
                        }
                        Field::Thursday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#thursday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("thursday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#thursday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("thursday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#thursday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ThursdayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#thursday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_thursday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("thursday");
                            }
                        }
                        Field::Friday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#friday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("friday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#friday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("friday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#friday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FridayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#friday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_friday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("friday");
                            }
                        }
                        Field::Saturday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#saturday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("saturday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#saturday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("saturday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#saturday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SaturdayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#saturday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_saturday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("saturday");
                            }
                        }
                        Field::Sunday => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sunday.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sunday"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#sunday.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sunday"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#sunday = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SundayPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sunday.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sunday"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sunday");
                            }
                        }
                        Field::WeekInterval => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#week_interval.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("weekInterval"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#week_interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field("weekInterval"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#week_interval =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::WeekIntervalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#week_interval.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_weekInterval"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("weekInterval");
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
                Ok(AppointmentRecurrenceTemplateWeeklyTemplate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#monday,
                    r#tuesday,
                    r#wednesday,
                    r#thursday,
                    r#friday,
                    r#saturday,
                    r#sunday,
                    r#week_interval,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AppointmentRecurrenceTemplateWeeklyTemplate>>
{
    type Value = Box<AppointmentRecurrenceTemplateWeeklyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AppointmentRecurrenceTemplateWeeklyTemplate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AppointmentRecurrenceTemplateWeeklyTemplate>>
{
    type Value = Vec<AppointmentRecurrenceTemplateWeeklyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<AppointmentRecurrenceTemplateWeeklyTemplate>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AppointmentRecurrenceTemplateWeeklyTemplate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    AppointmentRecurrenceTemplateWeeklyTemplate,
                > = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateMonthlyTemplate;
impl serde::ser::Serialize for SerializationContext<&AppointmentRecurrenceTemplateMonthlyTemplate> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment.recurrenceTemplate.monthlyTemplate", field
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#day_of_month.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("dayOfMonth", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_dayOfMonth", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#day_of_month.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("dayOfMonth", ctx)));
        }
        if let Some(some) = self.value.r#nth_week_of_month.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("nthWeekOfMonth", ctx)));
        }
        if let Some(some) = self.value.r#day_of_week.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("dayOfWeek", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#month_interval.id.as_deref() == Some("$invalid") {
                return missing_field_error("monthInterval");
            }
            if let Some(some) = self.value.r#month_interval.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("monthInterval", &some?));
            }
            if self.value.r#month_interval.id.is_some()
                || !self.value.r#month_interval.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#month_interval.id.as_ref(),
                    extension: &self.value.r#month_interval.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_monthInterval", ctx)));
            }
        } else if self.value.r#month_interval.id.as_deref() == Some("$invalid") {
            return missing_field_error("monthInterval");
        } else {
            tri!(self.with_context(&self.value.r#month_interval, |ctx| state
                .serialize_entry("monthInterval", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<AppointmentRecurrenceTemplateMonthlyTemplate>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<AppointmentRecurrenceTemplateMonthlyTemplate>>
{
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
    for &mut DeserializationContext<AppointmentRecurrenceTemplateMonthlyTemplate>
{
    type Value = AppointmentRecurrenceTemplateMonthlyTemplate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<AppointmentRecurrenceTemplateMonthlyTemplate>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AppointmentRecurrenceTemplateMonthlyTemplate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentRecurrenceTemplateMonthlyTemplate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AppointmentRecurrenceTemplateMonthlyTemplate, V::Error>
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
                    #[serde(rename = "dayOfMonth")]
                    DayOfMonth,
                    #[serde(rename = "_dayOfMonth")]
                    DayOfMonthPrimitiveElement,
                    #[serde(rename = "nthWeekOfMonth")]
                    NthWeekOfMonth,
                    #[serde(rename = "dayOfWeek")]
                    DayOfWeek,
                    #[serde(rename = "monthInterval")]
                    MonthInterval,
                    #[serde(rename = "_monthInterval")]
                    MonthIntervalPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "dayOfMonth",
                            "nthWeekOfMonth",
                            "dayOfWeek",
                            "monthInterval",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#day_of_month: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#nth_week_of_month: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#day_of_week: Option<Box<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#month_interval: Option<fhirbolt_model::r5::types::PositiveInt> = None;
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
                        Field::DayOfMonth => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#day_of_month.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dayOfMonth"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#day_of_month.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dayOfMonth"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#day_of_month =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DayOfMonthPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#day_of_month.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dayOfMonth"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("dayOfMonth");
                            }
                        }
                        Field::NthWeekOfMonth => {
                            if r#nth_week_of_month.is_some() {
                                return Err(serde::de::Error::duplicate_field("nthWeekOfMonth"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#nth_week_of_month =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::DayOfWeek => {
                            if r#day_of_week.is_some() {
                                return Err(serde::de::Error::duplicate_field("dayOfWeek"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#day_of_week = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::MonthInterval => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#month_interval.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monthInterval"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#month_interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monthInterval"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#month_interval =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MonthIntervalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#month_interval.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_monthInterval",
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
                                return unknown_field_error("monthInterval");
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
                Ok(AppointmentRecurrenceTemplateMonthlyTemplate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#day_of_month,
                    r#nth_week_of_month,
                    r#day_of_week,
                    r#month_interval: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#month_interval.unwrap_or(Default::default())
                    } else {
                        tri!(r#month_interval
                            .ok_or(serde::de::Error::missing_field("monthInterval")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AppointmentRecurrenceTemplateMonthlyTemplate>>
{
    type Value = Box<AppointmentRecurrenceTemplateMonthlyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AppointmentRecurrenceTemplateMonthlyTemplate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AppointmentRecurrenceTemplateMonthlyTemplate>>
{
    type Value = Vec<AppointmentRecurrenceTemplateMonthlyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<AppointmentRecurrenceTemplateMonthlyTemplate>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AppointmentRecurrenceTemplateMonthlyTemplate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    AppointmentRecurrenceTemplateMonthlyTemplate,
                > = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateYearlyTemplate;
impl serde::ser::Serialize for SerializationContext<&AppointmentRecurrenceTemplateYearlyTemplate> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment.recurrenceTemplate.yearlyTemplate", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#year_interval.id.as_deref() == Some("$invalid") {
                return missing_field_error("yearInterval");
            }
            if let Some(some) = self.value.r#year_interval.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("yearInterval", &some?));
            }
            if self.value.r#year_interval.id.is_some()
                || !self.value.r#year_interval.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#year_interval.id.as_ref(),
                    extension: &self.value.r#year_interval.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_yearInterval", ctx)));
            }
        } else if self.value.r#year_interval.id.as_deref() == Some("$invalid") {
            return missing_field_error("yearInterval");
        } else {
            tri!(self.with_context(&self.value.r#year_interval, |ctx| state
                .serialize_entry("yearInterval", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<AppointmentRecurrenceTemplateYearlyTemplate>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<AppointmentRecurrenceTemplateYearlyTemplate>>
{
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
    for &mut DeserializationContext<AppointmentRecurrenceTemplateYearlyTemplate>
{
    type Value = AppointmentRecurrenceTemplateYearlyTemplate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<AppointmentRecurrenceTemplateYearlyTemplate>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AppointmentRecurrenceTemplateYearlyTemplate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentRecurrenceTemplateYearlyTemplate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AppointmentRecurrenceTemplateYearlyTemplate, V::Error>
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
                    #[serde(rename = "yearInterval")]
                    YearInterval,
                    #[serde(rename = "_yearInterval")]
                    YearIntervalPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "yearInterval"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#year_interval: Option<fhirbolt_model::r5::types::PositiveInt> = None;
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
                        Field::YearInterval => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#year_interval.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("yearInterval"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#year_interval.is_some() {
                                    return Err(serde::de::Error::duplicate_field("yearInterval"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#year_interval =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::YearIntervalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#year_interval.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_yearInterval"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("yearInterval");
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
                Ok(AppointmentRecurrenceTemplateYearlyTemplate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#year_interval: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#year_interval.unwrap_or(Default::default())
                    } else {
                        tri!(r#year_interval.ok_or(serde::de::Error::missing_field("yearInterval")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AppointmentRecurrenceTemplateYearlyTemplate>>
{
    type Value = Box<AppointmentRecurrenceTemplateYearlyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AppointmentRecurrenceTemplateYearlyTemplate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AppointmentRecurrenceTemplateYearlyTemplate>>
{
    type Value = Vec<AppointmentRecurrenceTemplateYearlyTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<AppointmentRecurrenceTemplateYearlyTemplate>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AppointmentRecurrenceTemplateYearlyTemplate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    AppointmentRecurrenceTemplateYearlyTemplate,
                > = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::AppointmentRecurrenceTemplate;
impl serde::ser::Serialize for SerializationContext<&AppointmentRecurrenceTemplate> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment.recurrenceTemplate", field
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
        if let Some(some) = self.value.r#timezone.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("timezone", ctx)));
        }
        if self.value.r#recurrence_type.id.as_deref() == Some("$invalid") {
            return missing_field_error("recurrenceType");
        } else {
            tri!(self.with_context(&self.value.r#recurrence_type, |ctx| state
                .serialize_entry("recurrenceType", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#last_occurrence_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("lastOccurrenceDate", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_lastOccurrenceDate", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#last_occurrence_date.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("lastOccurrenceDate", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#occurrence_count.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("occurrenceCount", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_occurrenceCount", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#occurrence_count.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("occurrenceCount", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#occurrence_date.is_empty() {
                let values = tri!(self
                    .value
                    .r#occurrence_date
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("occurrenceDate", &values));
                }
                let requires_elements = self
                    .value
                    .r#occurrence_date
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#occurrence_date
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
                        .serialize_entry("_occurrenceDate", ctx)));
                }
            }
        } else if !self.value.r#occurrence_date.is_empty() {
            tri!(self.with_context(&self.value.r#occurrence_date, |ctx| state
                .serialize_entry("occurrenceDate", ctx)));
        }
        if let Some(some) = self.value.r#weekly_template.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("weeklyTemplate", ctx)));
        }
        if let Some(some) = self.value.r#monthly_template.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("monthlyTemplate", ctx)));
        }
        if let Some(some) = self.value.r#yearly_template.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("yearlyTemplate", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#excluding_date.is_empty() {
                let values = tri!(self
                    .value
                    .r#excluding_date
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("excludingDate", &values));
                }
                let requires_elements = self
                    .value
                    .r#excluding_date
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#excluding_date
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
                        .serialize_entry("_excludingDate", ctx)));
                }
            }
        } else if !self.value.r#excluding_date.is_empty() {
            tri!(self.with_context(&self.value.r#excluding_date, |ctx| state
                .serialize_entry("excludingDate", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#excluding_recurrence_id.is_empty() {
                let values = tri!(self
                    .value
                    .r#excluding_recurrence_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("excludingRecurrenceId", &values));
                }
                let requires_elements = self
                    .value
                    .r#excluding_recurrence_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#excluding_recurrence_id
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
                        .serialize_entry("_excludingRecurrenceId", ctx)));
                }
            }
        } else if !self.value.r#excluding_recurrence_id.is_empty() {
            tri!(
                self.with_context(&self.value.r#excluding_recurrence_id, |ctx| state
                    .serialize_entry("excludingRecurrenceId", ctx))
            );
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AppointmentRecurrenceTemplate>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AppointmentRecurrenceTemplate>> {
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
    for &mut DeserializationContext<AppointmentRecurrenceTemplate>
{
    type Value = AppointmentRecurrenceTemplate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AppointmentRecurrenceTemplate>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AppointmentRecurrenceTemplate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AppointmentRecurrenceTemplate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AppointmentRecurrenceTemplate, V::Error>
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
                    #[serde(rename = "timezone")]
                    Timezone,
                    #[serde(rename = "recurrenceType")]
                    RecurrenceType,
                    #[serde(rename = "lastOccurrenceDate")]
                    LastOccurrenceDate,
                    #[serde(rename = "_lastOccurrenceDate")]
                    LastOccurrenceDatePrimitiveElement,
                    #[serde(rename = "occurrenceCount")]
                    OccurrenceCount,
                    #[serde(rename = "_occurrenceCount")]
                    OccurrenceCountPrimitiveElement,
                    #[serde(rename = "occurrenceDate")]
                    OccurrenceDate,
                    #[serde(rename = "_occurrenceDate")]
                    OccurrenceDatePrimitiveElement,
                    #[serde(rename = "weeklyTemplate")]
                    WeeklyTemplate,
                    #[serde(rename = "monthlyTemplate")]
                    MonthlyTemplate,
                    #[serde(rename = "yearlyTemplate")]
                    YearlyTemplate,
                    #[serde(rename = "excludingDate")]
                    ExcludingDate,
                    #[serde(rename = "_excludingDate")]
                    ExcludingDatePrimitiveElement,
                    #[serde(rename = "excludingRecurrenceId")]
                    ExcludingRecurrenceId,
                    #[serde(rename = "_excludingRecurrenceId")]
                    ExcludingRecurrenceIdPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "timezone",
                            "recurrenceType",
                            "lastOccurrenceDate",
                            "occurrenceCount",
                            "occurrenceDate",
                            "weeklyTemplate",
                            "monthlyTemplate",
                            "yearlyTemplate",
                            "excludingDate",
                            "excludingRecurrenceId",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#timezone: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#recurrence_type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#last_occurrence_date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#occurrence_count: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#occurrence_date: Option<Vec<fhirbolt_model::r5::types::Date>> = None;
                let mut r#weekly_template: Option<
                    fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateWeeklyTemplate,
                > = None;
                let mut r#monthly_template: Option<
                    fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateMonthlyTemplate,
                > = None;
                let mut r#yearly_template: Option<
                    fhirbolt_model::r5::resources::AppointmentRecurrenceTemplateYearlyTemplate,
                > = None;
                let mut r#excluding_date: Option<Vec<fhirbolt_model::r5::types::Date>> = None;
                let mut r#excluding_recurrence_id: Option<
                    Vec<fhirbolt_model::r5::types::PositiveInt>,
                > = None;
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
                        Field::Timezone => {
                            if r#timezone.is_some() {
                                return Err(serde::de::Error::duplicate_field("timezone"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#timezone = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::RecurrenceType => {
                            if r#recurrence_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("recurrenceType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#recurrence_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::LastOccurrenceDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_occurrence_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastOccurrenceDate",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#last_occurrence_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastOccurrenceDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#last_occurrence_date =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LastOccurrenceDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_occurrence_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastOccurrenceDate",
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
                                return unknown_field_error("lastOccurrenceDate");
                            }
                        }
                        Field::OccurrenceCount => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#occurrence_count.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceCount",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#occurrence_count.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceCount",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#occurrence_count =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OccurrenceCountPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#occurrence_count.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceCount",
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
                                return unknown_field_error("occurrenceCount");
                            }
                        }
                        Field::OccurrenceDate => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#occurrence_date.get_or_insert(
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
                                        "occurrenceDate",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#occurrence_date.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OccurrenceDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#occurrence_date.get_or_insert(
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
                                        "_occurrenceDate",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("occurrenceDate");
                            }
                        }
                        Field::WeeklyTemplate => {
                            if r#weekly_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("weeklyTemplate"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: AppointmentRecurrenceTemplateWeeklyTemplate > = self . 0 . transmute () ;
                            r#weekly_template =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::MonthlyTemplate => {
                            if r#monthly_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("monthlyTemplate"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: AppointmentRecurrenceTemplateMonthlyTemplate > = self . 0 . transmute () ;
                            r#monthly_template =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::YearlyTemplate => {
                            if r#yearly_template.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearlyTemplate"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: AppointmentRecurrenceTemplateYearlyTemplate > = self . 0 . transmute () ;
                            r#yearly_template =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ExcludingDate => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#excluding_date.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("excludingDate"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#excluding_date.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ExcludingDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#excluding_date.get_or_insert(
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
                                        "_excludingDate",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("excludingDate");
                            }
                        }
                        Field::ExcludingRecurrenceId => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#excluding_recurrence_id.get_or_insert(
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
                                        "excludingRecurrenceId",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec =
                                    r#excluding_recurrence_id.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ExcludingRecurrenceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#excluding_recurrence_id.get_or_insert(
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
                                        "_excludingRecurrenceId",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("excludingRecurrenceId");
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
                Ok(AppointmentRecurrenceTemplate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#timezone,
                    r#recurrence_type: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#recurrence_type.unwrap_or(Default::default())
                    } else {
                        tri!(r#recurrence_type
                            .ok_or(serde::de::Error::missing_field("recurrenceType")))
                    },
                    r#last_occurrence_date,
                    r#occurrence_count,
                    r#occurrence_date: r#occurrence_date.unwrap_or(vec![]),
                    r#weekly_template,
                    r#monthly_template,
                    r#yearly_template,
                    r#excluding_date: r#excluding_date.unwrap_or(vec![]),
                    r#excluding_recurrence_id: r#excluding_recurrence_id.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AppointmentRecurrenceTemplate>>
{
    type Value = Box<AppointmentRecurrenceTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AppointmentRecurrenceTemplate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AppointmentRecurrenceTemplate>>
{
    type Value = Vec<AppointmentRecurrenceTemplate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AppointmentRecurrenceTemplate>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AppointmentRecurrenceTemplate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AppointmentRecurrenceTemplate> =
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
use fhirbolt_model::r5::resources::Appointment;
impl crate::Resource for Appointment {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&Appointment> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Appointment", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "Appointment"));
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
        if let Some(some) = self.value.r#cancellation_reason.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("cancellationReason", ctx)));
        }
        if !self.value.r#class.is_empty() {
            tri!(self.with_context(&self.value.r#class, |ctx| state
                .serialize_entry("class", ctx)));
        }
        if !self.value.r#service_category.is_empty() {
            tri!(
                self.with_context(&self.value.r#service_category, |ctx| state
                    .serialize_entry("serviceCategory", ctx))
            );
        }
        if !self.value.r#service_type.is_empty() {
            tri!(self.with_context(&self.value.r#service_type, |ctx| state
                .serialize_entry("serviceType", ctx)));
        }
        if !self.value.r#specialty.is_empty() {
            tri!(self.with_context(&self.value.r#specialty, |ctx| state
                .serialize_entry("specialty", ctx)));
        }
        if let Some(some) = self.value.r#appointment_type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("appointmentType", ctx)));
        }
        if !self.value.r#reason.is_empty() {
            tri!(self.with_context(&self.value.r#reason, |ctx| state
                .serialize_entry("reason", ctx)));
        }
        if let Some(some) = self.value.r#priority.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("priority", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("description", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_description", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("description", ctx)));
        }
        if !self.value.r#replaces.is_empty() {
            tri!(self.with_context(&self.value.r#replaces, |ctx| state
                .serialize_entry("replaces", ctx)));
        }
        if !self.value.r#virtual_service.is_empty() {
            tri!(self.with_context(&self.value.r#virtual_service, |ctx| state
                .serialize_entry("virtualService", ctx)));
        }
        if !self.value.r#supporting_information.is_empty() {
            tri!(
                self.with_context(&self.value.r#supporting_information, |ctx| state
                    .serialize_entry("supportingInformation", ctx))
            );
        }
        if let Some(some) = self.value.r#previous_appointment.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("previousAppointment", ctx)));
        }
        if let Some(some) = self.value.r#originating_appointment.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("originatingAppointment", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("start", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_start", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#start.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("start", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("end", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_end", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#end.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("end", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#minutes_duration.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("minutesDuration", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_minutesDuration", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#minutes_duration.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("minutesDuration", ctx)));
        }
        if !self.value.r#requested_period.is_empty() {
            tri!(
                self.with_context(&self.value.r#requested_period, |ctx| state
                    .serialize_entry("requestedPeriod", ctx))
            );
        }
        if !self.value.r#slot.is_empty() {
            tri!(self.with_context(&self.value.r#slot, |ctx| state.serialize_entry("slot", ctx)));
        }
        if !self.value.r#account.is_empty() {
            tri!(self.with_context(&self.value.r#account, |ctx| state
                .serialize_entry("account", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#created.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("created", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_created", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#created.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("created", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#cancellation_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("cancellationDate", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_cancellationDate", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#cancellation_date.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("cancellationDate", ctx)));
        }
        if !self.value.r#note.is_empty() {
            tri!(self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx)));
        }
        if !self.value.r#patient_instruction.is_empty() {
            tri!(
                self.with_context(&self.value.r#patient_instruction, |ctx| state
                    .serialize_entry("patientInstruction", ctx))
            );
        }
        if !self.value.r#based_on.is_empty() {
            tri!(self.with_context(&self.value.r#based_on, |ctx| state
                .serialize_entry("basedOn", ctx)));
        }
        if let Some(some) = self.value.r#subject.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("subject", ctx)));
        }
        if !self.value.r#participant.is_empty() {
            tri!(self.with_context(&self.value.r#participant, |ctx| state
                .serialize_entry("participant", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#recurrence_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("recurrenceId", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_recurrenceId", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#recurrence_id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("recurrenceId", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#occurrence_changed.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("occurrenceChanged", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_occurrenceChanged", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#occurrence_changed.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("occurrenceChanged", ctx)));
        }
        if !self.value.r#recurrence_template.is_empty() {
            tri!(
                self.with_context(&self.value.r#recurrence_template, |ctx| state
                    .serialize_entry("recurrenceTemplate", ctx))
            );
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Appointment>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Appointment>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Appointment> {
    type Value = Appointment;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Appointment> {
    type Value = Appointment;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Appointment>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Appointment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Appointment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Appointment, V::Error>
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
                    #[serde(rename = "cancellationReason")]
                    CancellationReason,
                    #[serde(rename = "class")]
                    Class,
                    #[serde(rename = "serviceCategory")]
                    ServiceCategory,
                    #[serde(rename = "serviceType")]
                    ServiceType,
                    #[serde(rename = "specialty")]
                    Specialty,
                    #[serde(rename = "appointmentType")]
                    AppointmentType,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "priority")]
                    Priority,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "replaces")]
                    Replaces,
                    #[serde(rename = "virtualService")]
                    VirtualService,
                    #[serde(rename = "supportingInformation")]
                    SupportingInformation,
                    #[serde(rename = "previousAppointment")]
                    PreviousAppointment,
                    #[serde(rename = "originatingAppointment")]
                    OriginatingAppointment,
                    #[serde(rename = "start")]
                    Start,
                    #[serde(rename = "_start")]
                    StartPrimitiveElement,
                    #[serde(rename = "end")]
                    End,
                    #[serde(rename = "_end")]
                    EndPrimitiveElement,
                    #[serde(rename = "minutesDuration")]
                    MinutesDuration,
                    #[serde(rename = "_minutesDuration")]
                    MinutesDurationPrimitiveElement,
                    #[serde(rename = "requestedPeriod")]
                    RequestedPeriod,
                    #[serde(rename = "slot")]
                    Slot,
                    #[serde(rename = "account")]
                    Account,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "cancellationDate")]
                    CancellationDate,
                    #[serde(rename = "_cancellationDate")]
                    CancellationDatePrimitiveElement,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "patientInstruction")]
                    PatientInstruction,
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "participant")]
                    Participant,
                    #[serde(rename = "recurrenceId")]
                    RecurrenceId,
                    #[serde(rename = "_recurrenceId")]
                    RecurrenceIdPrimitiveElement,
                    #[serde(rename = "occurrenceChanged")]
                    OccurrenceChanged,
                    #[serde(rename = "_occurrenceChanged")]
                    OccurrenceChangedPrimitiveElement,
                    #[serde(rename = "recurrenceTemplate")]
                    RecurrenceTemplate,
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
                            "cancellationReason",
                            "class",
                            "serviceCategory",
                            "serviceType",
                            "specialty",
                            "appointmentType",
                            "reason",
                            "priority",
                            "description",
                            "replaces",
                            "virtualService",
                            "supportingInformation",
                            "previousAppointment",
                            "originatingAppointment",
                            "start",
                            "end",
                            "minutesDuration",
                            "requestedPeriod",
                            "slot",
                            "account",
                            "created",
                            "cancellationDate",
                            "note",
                            "patientInstruction",
                            "basedOn",
                            "subject",
                            "participant",
                            "recurrenceId",
                            "occurrenceChanged",
                            "recurrenceTemplate",
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
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#cancellation_reason: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#class: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#service_category: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#service_type: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#specialty: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#appointment_type: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#priority: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#replaces: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#virtual_service: Option<
                    Vec<fhirbolt_model::r5::types::VirtualServiceDetail>,
                > = None;
                let mut r#supporting_information: Option<
                    Vec<fhirbolt_model::r5::types::Reference>,
                > = None;
                let mut r#previous_appointment: Option<Box<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#originating_appointment: Option<
                    Box<fhirbolt_model::r5::types::Reference>,
                > = None;
                let mut r#start: Option<fhirbolt_model::r5::types::Instant> = None;
                let mut r#end: Option<fhirbolt_model::r5::types::Instant> = None;
                let mut r#minutes_duration: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#requested_period: Option<Vec<fhirbolt_model::r5::types::Period>> = None;
                let mut r#slot: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#account: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#created: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#cancellation_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#patient_instruction: Option<
                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                > = None;
                let mut r#based_on: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#participant: Option<
                    Vec<fhirbolt_model::r5::resources::AppointmentParticipant>,
                > = None;
                let mut r#recurrence_id: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#occurrence_changed: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#recurrence_template: Option<
                    Vec<fhirbolt_model::r5::resources::AppointmentRecurrenceTemplate>,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "Appointment" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Appointment",
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
                        Field::CancellationReason => {
                            if r#cancellation_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cancellationReason",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#cancellation_reason =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Class => {
                            if self.0.from == crate::context::Format::Json {
                                if r#class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("class"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#class = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#class.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ServiceCategory => {
                            if self.0.from == crate::context::Format::Json {
                                if r#service_category.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "serviceCategory",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#service_category =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#service_category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                r#service_type =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#service_type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Specialty => {
                            if self.0.from == crate::context::Format::Json {
                                if r#specialty.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specialty"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#specialty =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#specialty.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AppointmentType => {
                            if r#appointment_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("appointmentType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#appointment_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                        Field::Priority => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#priority = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#description =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Replaces => {
                            if self.0.from == crate::context::Format::Json {
                                if r#replaces.is_some() {
                                    return Err(serde::de::Error::duplicate_field("replaces"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#replaces = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#replaces.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#virtual_service.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::VirtualServiceDetail,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SupportingInformation => {
                            if self.0.from == crate::context::Format::Json {
                                if r#supporting_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInformation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#supporting_information =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec =
                                    r#supporting_information.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PreviousAppointment => {
                            if r#previous_appointment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "previousAppointment",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#previous_appointment =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::OriginatingAppointment => {
                            if r#originating_appointment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "originatingAppointment",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#originating_appointment =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Start => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Instant,
                                > = self.0.transmute();
                                r#start = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::StartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_start"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("start");
                            }
                        }
                        Field::End => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Instant,
                                > = self.0.transmute();
                                r#end = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::EndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_end"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("end");
                            }
                        }
                        Field::MinutesDuration => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#minutes_duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minutesDuration",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#minutes_duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minutesDuration",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#minutes_duration =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MinutesDurationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#minutes_duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minutesDuration",
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
                                return unknown_field_error("minutesDuration");
                            }
                        }
                        Field::RequestedPeriod => {
                            if self.0.from == crate::context::Format::Json {
                                if r#requested_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "requestedPeriod",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Period>,
                                > = self.0.transmute();
                                r#requested_period =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#requested_period.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Period,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Slot => {
                            if self.0.from == crate::context::Format::Json {
                                if r#slot.is_some() {
                                    return Err(serde::de::Error::duplicate_field("slot"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#slot = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#slot.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                r#account = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#account.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Created => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#created.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#created = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("created");
                            }
                        }
                        Field::CancellationDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cancellation_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cancellationDate",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#cancellation_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cancellationDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#cancellation_date =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CancellationDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cancellation_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_cancellationDate",
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
                                return unknown_field_error("cancellationDate");
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
                        Field::PatientInstruction => {
                            if self.0.from == crate::context::Format::Json {
                                if r#patient_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#patient_instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#patient_instruction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                r#based_on = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Participant => {
                            if self.0.from == crate::context::Format::Json {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::AppointmentParticipant>,
                                > = self.0.transmute();
                                r#participant =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#participant.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::AppointmentParticipant,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RecurrenceId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recurrence_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recurrenceId"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#recurrence_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recurrenceId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#recurrence_id =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RecurrenceIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recurrence_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recurrenceId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recurrenceId");
                            }
                        }
                        Field::OccurrenceChanged => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#occurrence_changed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceChanged",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#occurrence_changed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceChanged",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#occurrence_changed =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OccurrenceChangedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#occurrence_changed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceChanged",
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
                                return unknown_field_error("occurrenceChanged");
                            }
                        }
                        Field::RecurrenceTemplate => {
                            if self.0.from == crate::context::Format::Json {
                                if r#recurrence_template.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "recurrenceTemplate",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AppointmentRecurrenceTemplate >> = self . 0 . transmute () ;
                                r#recurrence_template =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#recurrence_template.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::AppointmentRecurrenceTemplate,
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
                Ok(Appointment {
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
                        tri!(r#status.ok_or(serde::de::Error::missing_field("status")))
                    },
                    r#cancellation_reason,
                    r#class: r#class.unwrap_or(vec![]),
                    r#service_category: r#service_category.unwrap_or(vec![]),
                    r#service_type: r#service_type.unwrap_or(vec![]),
                    r#specialty: r#specialty.unwrap_or(vec![]),
                    r#appointment_type,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#priority,
                    r#description,
                    r#replaces: r#replaces.unwrap_or(vec![]),
                    r#virtual_service: r#virtual_service.unwrap_or(vec![]),
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#previous_appointment,
                    r#originating_appointment,
                    r#start,
                    r#end,
                    r#minutes_duration,
                    r#requested_period: r#requested_period.unwrap_or(vec![]),
                    r#slot: r#slot.unwrap_or(vec![]),
                    r#account: r#account.unwrap_or(vec![]),
                    r#created,
                    r#cancellation_date,
                    r#note: r#note.unwrap_or(vec![]),
                    r#patient_instruction: r#patient_instruction.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#subject,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#recurrence_id,
                    r#occurrence_changed,
                    r#recurrence_template: r#recurrence_template.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Appointment>> {
    type Value = Box<Appointment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Appointment>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Appointment>> {
    type Value = Vec<Appointment>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Appointment>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Appointment>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Appointment> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
