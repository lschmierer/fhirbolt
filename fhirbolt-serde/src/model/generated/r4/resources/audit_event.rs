// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::AuditEventAgentNetwork;
impl serde::ser::Serialize for SerializationContext<&AuditEventAgentNetwork> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent.agent.network", field
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
            if let Some(some) = self.value.r#address.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("address", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_address", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#address.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("address", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("type", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_type", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEventAgentNetwork>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEventAgentNetwork>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEventAgentNetwork> {
    type Value = AuditEventAgentNetwork;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEventAgentNetwork>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEventAgentNetwork;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgentNetwork")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgentNetwork, V::Error>
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
                    #[serde(rename = "address")]
                    Address,
                    #[serde(rename = "_address")]
                    AddressPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "address", "type"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#address: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Address => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#address.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#address = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AddressPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#address.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_address"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("address");
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
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
                Ok(AuditEventAgentNetwork {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#address,
                    r#type,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AuditEventAgentNetwork>>
{
    type Value = Box<AuditEventAgentNetwork>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEventAgentNetwork>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AuditEventAgentNetwork>>
{
    type Value = Vec<AuditEventAgentNetwork>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEventAgentNetwork>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEventAgentNetwork>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEventAgentNetwork> =
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
use fhirbolt_model::r4::resources::AuditEventAgent;
impl serde::ser::Serialize for SerializationContext<&AuditEventAgent> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent.agent", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        if !self.value.r#role.is_empty() {
            tri!(self.with_context(&self.value.r#role, |ctx| state.serialize_entry("role", ctx)));
        }
        if let Some(some) = self.value.r#who.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("who", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#alt_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("altId", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_altId", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#alt_id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("altId", ctx)));
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
        if self.output == crate::context::Format::Json {
            if self.value.r#requestor.id.as_deref() == Some("$invalid") {
                return missing_field_error("requestor");
            }
            if let Some(some) = self.value.r#requestor.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("requestor", &some?));
            }
            if self.value.r#requestor.id.is_some() || !self.value.r#requestor.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#requestor.id.as_ref(),
                    extension: &self.value.r#requestor.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_requestor", ctx)));
            }
        } else if self.value.r#requestor.id.as_deref() == Some("$invalid") {
            return missing_field_error("requestor");
        } else {
            tri!(self.with_context(&self.value.r#requestor, |ctx| state
                .serialize_entry("requestor", ctx)));
        }
        if let Some(some) = self.value.r#location.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("location", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#policy.is_empty() {
                let values = tri!(self
                    .value
                    .r#policy
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("policy", &values));
                }
                let requires_elements = self
                    .value
                    .r#policy
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#policy
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
                        .serialize_entry("_policy", ctx)));
                }
            }
        } else if !self.value.r#policy.is_empty() {
            tri!(self.with_context(&self.value.r#policy, |ctx| state
                .serialize_entry("policy", ctx)));
        }
        if let Some(some) = self.value.r#media.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("media", ctx)));
        }
        if let Some(some) = self.value.r#network.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("network", ctx)));
        }
        if !self.value.r#purpose_of_use.is_empty() {
            tri!(self.with_context(&self.value.r#purpose_of_use, |ctx| state
                .serialize_entry("purposeOfUse", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEventAgent>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEventAgent>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEventAgent> {
    type Value = AuditEventAgent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEventAgent>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEventAgent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgent, V::Error>
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
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "who")]
                    Who,
                    #[serde(rename = "altId")]
                    AltId,
                    #[serde(rename = "_altId")]
                    AltIdPrimitiveElement,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "requestor")]
                    Requestor,
                    #[serde(rename = "_requestor")]
                    RequestorPrimitiveElement,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "policy")]
                    Policy,
                    #[serde(rename = "_policy")]
                    PolicyPrimitiveElement,
                    #[serde(rename = "media")]
                    Media,
                    #[serde(rename = "network")]
                    Network,
                    #[serde(rename = "purposeOfUse")]
                    PurposeOfUse,
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
                            "role",
                            "who",
                            "altId",
                            "name",
                            "requestor",
                            "location",
                            "policy",
                            "media",
                            "network",
                            "purposeOfUse",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#role: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#who: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#alt_id: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#requestor: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#location: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#policy: Option<Vec<fhirbolt_model::r4::types::Uri>> = None;
                let mut r#media: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#network: Option<fhirbolt_model::r4::resources::AuditEventAgentNetwork> =
                    None;
                let mut r#purpose_of_use: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Role => {
                            if self.0.from == crate::context::Format::Json {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#role = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#role.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Who => {
                            if r#who.is_some() {
                                return Err(serde::de::Error::duplicate_field("who"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#who = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AltId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#alt_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("altId"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#alt_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("altId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#alt_id = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AltIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#alt_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_altId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("altId");
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
                                    fhirbolt_model::r4::types::String,
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
                        Field::Requestor => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#requestor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestor"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#requestor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestor"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#requestor =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RequestorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#requestor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requestor"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requestor");
                            }
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#location = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Policy => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#policy.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("policy"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#policy.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PolicyPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#policy.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_policy"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("policy");
                            }
                        }
                        Field::Media => {
                            if r#media.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#media = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Network => {
                            if r#network.is_some() {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::AuditEventAgentNetwork,
                            > = self.0.transmute();
                            r#network = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::PurposeOfUse => {
                            if self.0.from == crate::context::Format::Json {
                                if r#purpose_of_use.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purposeOfUse"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#purpose_of_use =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#purpose_of_use.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
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
                Ok(AuditEventAgent {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#role: r#role.unwrap_or(vec![]),
                    r#who,
                    r#alt_id,
                    r#name,
                    r#requestor: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#requestor.unwrap_or(Default::default())
                    } else {
                        tri!(r#requestor.ok_or(serde::de::Error::missing_field("requestor")))
                    },
                    r#location,
                    r#policy: r#policy.unwrap_or(vec![]),
                    r#media,
                    r#network,
                    r#purpose_of_use: r#purpose_of_use.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<AuditEventAgent>> {
    type Value = Box<AuditEventAgent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEventAgent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<AuditEventAgent>> {
    type Value = Vec<AuditEventAgent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEventAgent>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEventAgent>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEventAgent> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::AuditEventSource;
impl serde::ser::Serialize for SerializationContext<&AuditEventSource> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent.source", field
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
            if let Some(some) = self.value.r#site.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("site", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_site", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#site.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("site", ctx)));
        }
        if self.value.r#observer.id.as_deref() == Some("$invalid") {
            return missing_field_error("observer");
        } else {
            tri!(self.with_context(&self.value.r#observer, |ctx| state
                .serialize_entry("observer", ctx)));
        }
        if !self.value.r#type.is_empty() {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEventSource>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEventSource>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEventSource> {
    type Value = AuditEventSource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEventSource>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEventSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventSource")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventSource, V::Error>
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
                    #[serde(rename = "_site")]
                    SitePrimitiveElement,
                    #[serde(rename = "observer")]
                    Observer,
                    #[serde(rename = "type")]
                    Type,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "site",
                            "observer",
                            "type",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#site: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#observer: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Site => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#site.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#site = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SitePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#site.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_site"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("site");
                            }
                        }
                        Field::Observer => {
                            if r#observer.is_some() {
                                return Err(serde::de::Error::duplicate_field("observer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#observer = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
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
                Ok(AuditEventSource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#site,
                    r#observer: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#observer.unwrap_or(Default::default())
                    } else {
                        tri!(r#observer.ok_or(serde::de::Error::missing_field("observer")))
                    },
                    r#type: r#type.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<AuditEventSource>> {
    type Value = Box<AuditEventSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEventSource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<AuditEventSource>> {
    type Value = Vec<AuditEventSource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEventSource>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEventSource>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEventSource> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::AuditEventEntityDetail;
impl serde::ser::Serialize for SerializationContext<&AuditEventEntityDetail> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent.entity.detail", field
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
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            if let Some(some) = self.value.r#type.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("type", &some?));
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_type", ctx)));
            }
        } else if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        {
            use fhirbolt_model::r4::resources::AuditEventEntityDetailValue as _Enum;
            match self.value.r#value {
                _Enum::String(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            tri!(state.serialize_entry("valueString", &some?));
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            tri!(self.with_context(&primitive_element, |ctx| state
                                .serialize_entry("_valueString", ctx)));
                        }
                    } else {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("valueString", ctx)));
                    }
                }
                _Enum::Base64Binary(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            tri!(state.serialize_entry("valueBase64Binary", &some?));
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            tri!(self.with_context(&primitive_element, |ctx| state
                                .serialize_entry("_valueBase64Binary", ctx)));
                        }
                    } else {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("valueBase64Binary", ctx)));
                    }
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEventEntityDetail>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEventEntityDetail>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEventEntityDetail> {
    type Value = AuditEventEntityDetail;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEventEntityDetail>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEventEntityDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntityDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntityDetail, V::Error>
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
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueBase64Binary")]
                    ValueBase64Binary,
                    #[serde(rename = "_valueBase64Binary")]
                    ValueBase64BinaryPrimitiveElement,
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
                            "valueString",
                            "valueBase64Binary",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#type: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#value: Option<
                    fhirbolt_model::r4::resources::AuditEventEntityDetailValue,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::ValueString => {
                            use fhirbolt_model::r4::resources::AuditEventEntityDetailValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#value = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            use fhirbolt_model::r4::resources::AuditEventEntityDetailValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
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
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueBase64Binary => {
                            use fhirbolt_model::r4::resources::AuditEventEntityDetailValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Base64Binary,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Base64Binary(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4::resources::AuditEventEntityDetailValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
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
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBase64Binary");
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
                Ok(AuditEventEntityDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        tri!(r#type.ok_or(serde::de::Error::missing_field("type")))
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        tri!(r#value.ok_or(serde::de::Error::missing_field("value[x]")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<AuditEventEntityDetail>>
{
    type Value = Box<AuditEventEntityDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEventEntityDetail>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<AuditEventEntityDetail>>
{
    type Value = Vec<AuditEventEntityDetail>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEventEntityDetail>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEventEntityDetail>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEventEntityDetail> =
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
use fhirbolt_model::r4::resources::AuditEventEntity;
impl serde::ser::Serialize for SerializationContext<&AuditEventEntity> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent.entity", field
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
        if let Some(some) = self.value.r#what.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("what", ctx)));
        }
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        if let Some(some) = self.value.r#role.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("role", ctx)));
        }
        if let Some(some) = self.value.r#lifecycle.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("lifecycle", ctx)));
        }
        if !self.value.r#security_label.is_empty() {
            tri!(self.with_context(&self.value.r#security_label, |ctx| state
                .serialize_entry("securityLabel", ctx)));
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#query.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("query", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_query", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#query.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("query", ctx)));
        }
        if !self.value.r#detail.is_empty() {
            tri!(self.with_context(&self.value.r#detail, |ctx| state
                .serialize_entry("detail", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEventEntity>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEventEntity>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEventEntity> {
    type Value = AuditEventEntity;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEventEntity>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEventEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntity, V::Error>
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
                    #[serde(rename = "what")]
                    What,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "lifecycle")]
                    Lifecycle,
                    #[serde(rename = "securityLabel")]
                    SecurityLabel,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "query")]
                    Query,
                    #[serde(rename = "_query")]
                    QueryPrimitiveElement,
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
                            "what",
                            "type",
                            "role",
                            "lifecycle",
                            "securityLabel",
                            "name",
                            "description",
                            "query",
                            "detail",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#what: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#lifecycle: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#security_label: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#query: Option<fhirbolt_model::r4::types::Base64Binary> = None;
                let mut r#detail: Option<
                    Vec<fhirbolt_model::r4::resources::AuditEventEntityDetail>,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::What => {
                            if r#what.is_some() {
                                return Err(serde::de::Error::duplicate_field("what"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#what = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#role = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Lifecycle => {
                            if r#lifecycle.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycle"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#lifecycle = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::SecurityLabel => {
                            if self.0.from == crate::context::Format::Json {
                                if r#security_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("securityLabel"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#security_label =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#security_label.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
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
                                    fhirbolt_model::r4::types::String,
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
                                    fhirbolt_model::r4::types::String,
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
                        Field::Query => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("query"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#query.is_some() {
                                    return Err(serde::de::Error::duplicate_field("query"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Base64Binary,
                                > = self.0.transmute();
                                r#query = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::QueryPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_query"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("query");
                            }
                        }
                        Field::Detail => {
                            if self.0.from == crate::context::Format::Json {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::AuditEventEntityDetail>,
                                > = self.0.transmute();
                                r#detail = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#detail.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::AuditEventEntityDetail,
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
                Ok(AuditEventEntity {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#what,
                    r#type,
                    r#role,
                    r#lifecycle,
                    r#security_label: r#security_label.unwrap_or(vec![]),
                    r#name,
                    r#description,
                    r#query,
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<AuditEventEntity>> {
    type Value = Box<AuditEventEntity>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEventEntity>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<AuditEventEntity>> {
    type Value = Vec<AuditEventEntity>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEventEntity>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEventEntity>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEventEntity> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::AuditEvent;
impl crate::Resource for AuditEvent {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&AuditEvent> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AuditEvent", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "AuditEvent"));
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        if !self.value.r#subtype.is_empty() {
            tri!(self.with_context(&self.value.r#subtype, |ctx| state
                .serialize_entry("subtype", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#action.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("action", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_action", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#action.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("action", ctx)));
        }
        if let Some(some) = self.value.r#period.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("period", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#recorded.id.as_deref() == Some("$invalid") {
                return missing_field_error("recorded");
            }
            if let Some(some) = self.value.r#recorded.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("recorded", &some?));
            }
            if self.value.r#recorded.id.is_some() || !self.value.r#recorded.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#recorded.id.as_ref(),
                    extension: &self.value.r#recorded.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_recorded", ctx)));
            }
        } else if self.value.r#recorded.id.as_deref() == Some("$invalid") {
            return missing_field_error("recorded");
        } else {
            tri!(self.with_context(&self.value.r#recorded, |ctx| state
                .serialize_entry("recorded", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#outcome.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("outcome", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_outcome", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#outcome.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("outcome", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#outcome_desc.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("outcomeDesc", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_outcomeDesc", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#outcome_desc.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("outcomeDesc", ctx)));
        }
        if !self.value.r#purpose_of_event.is_empty() {
            tri!(
                self.with_context(&self.value.r#purpose_of_event, |ctx| state
                    .serialize_entry("purposeOfEvent", ctx))
            );
        }
        if !self.value.r#agent.is_empty() {
            tri!(self.with_context(&self.value.r#agent, |ctx| state
                .serialize_entry("agent", ctx)));
        }
        if self.value.r#source.id.as_deref() == Some("$invalid") {
            return missing_field_error("source");
        } else {
            tri!(self.with_context(&self.value.r#source, |ctx| state
                .serialize_entry("source", ctx)));
        }
        if !self.value.r#entity.is_empty() {
            tri!(self.with_context(&self.value.r#entity, |ctx| state
                .serialize_entry("entity", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<AuditEvent>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<AuditEvent>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<AuditEvent> {
    type Value = AuditEvent;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<AuditEvent> {
    type Value = AuditEvent;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<AuditEvent>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = AuditEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEvent, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subtype")]
                    Subtype,
                    #[serde(rename = "action")]
                    Action,
                    #[serde(rename = "_action")]
                    ActionPrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "recorded")]
                    Recorded,
                    #[serde(rename = "_recorded")]
                    RecordedPrimitiveElement,
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "_outcome")]
                    OutcomePrimitiveElement,
                    #[serde(rename = "outcomeDesc")]
                    OutcomeDesc,
                    #[serde(rename = "_outcomeDesc")]
                    OutcomeDescPrimitiveElement,
                    #[serde(rename = "purposeOfEvent")]
                    PurposeOfEvent,
                    #[serde(rename = "agent")]
                    Agent,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "entity")]
                    Entity,
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
                            "type",
                            "subtype",
                            "action",
                            "period",
                            "recorded",
                            "outcome",
                            "outcomeDesc",
                            "purposeOfEvent",
                            "agent",
                            "source",
                            "entity",
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
                let mut r#type: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#subtype: Option<Vec<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#action: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4::types::Period>> = None;
                let mut r#recorded: Option<fhirbolt_model::r4::types::Instant> = None;
                let mut r#outcome: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#outcome_desc: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#purpose_of_event: Option<
                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#agent: Option<Vec<fhirbolt_model::r4::resources::AuditEventAgent>> = None;
                let mut r#source: Option<fhirbolt_model::r4::resources::AuditEventSource> = None;
                let mut r#entity: Option<Vec<fhirbolt_model::r4::resources::AuditEventEntity>> =
                    None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "AuditEvent" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"AuditEvent",
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
                                    fhirbolt_model::r4::types::Id,
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
                                Box<fhirbolt_model::r4::types::Meta>,
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
                                    fhirbolt_model::r4::types::Uri,
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
                                    fhirbolt_model::r4::types::Code,
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Subtype => {
                            if self.0.from == crate::context::Format::Json {
                                if r#subtype.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtype"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Coding>,
                                > = self.0.transmute();
                                r#subtype = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#subtype.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Coding,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Action => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#action.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#action = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ActionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#action.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_action"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("action");
                            }
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#period = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Recorded => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#recorded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Instant,
                                > = self.0.transmute();
                                r#recorded = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RecordedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recorded"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recorded");
                            }
                        }
                        Field::Outcome => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#outcome = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("outcome");
                            }
                        }
                        Field::OutcomeDesc => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#outcome_desc.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeDesc"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#outcome_desc.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeDesc"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#outcome_desc =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::OutcomeDescPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#outcome_desc.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcomeDesc"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("outcomeDesc");
                            }
                        }
                        Field::PurposeOfEvent => {
                            if self.0.from == crate::context::Format::Json {
                                if r#purpose_of_event.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "purposeOfEvent",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#purpose_of_event =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#purpose_of_event.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Agent => {
                            if self.0.from == crate::context::Format::Json {
                                if r#agent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("agent"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::AuditEventAgent>,
                                > = self.0.transmute();
                                r#agent = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#agent.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::AuditEventAgent,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::AuditEventSource,
                            > = self.0.transmute();
                            r#source = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Entity => {
                            if self.0.from == crate::context::Format::Json {
                                if r#entity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entity"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::AuditEventEntity>,
                                > = self.0.transmute();
                                r#entity = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#entity.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::AuditEventEntity,
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
                Ok(AuditEvent {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        tri!(r#type.ok_or(serde::de::Error::missing_field("type")))
                    },
                    r#subtype: r#subtype.unwrap_or(vec![]),
                    r#action,
                    r#period,
                    r#recorded: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#recorded.unwrap_or(Default::default())
                    } else {
                        tri!(r#recorded.ok_or(serde::de::Error::missing_field("recorded")))
                    },
                    r#outcome,
                    r#outcome_desc,
                    r#purpose_of_event: r#purpose_of_event.unwrap_or(vec![]),
                    r#agent: r#agent.unwrap_or(vec![]),
                    r#source: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#source.unwrap_or(Default::default())
                    } else {
                        tri!(r#source.ok_or(serde::de::Error::missing_field("source")))
                    },
                    r#entity: r#entity.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<AuditEvent>> {
    type Value = Box<AuditEvent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<AuditEvent>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<AuditEvent>> {
    type Value = Vec<AuditEvent>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<AuditEvent>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<AuditEvent>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<AuditEvent> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
