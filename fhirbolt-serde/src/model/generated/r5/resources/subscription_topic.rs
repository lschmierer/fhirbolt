// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::SubscriptionTopicResourceTriggerQueryCriteria;
impl serde::ser::Serialize
    for SerializationContext<&SubscriptionTopicResourceTriggerQueryCriteria>
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
                "SubscriptionTopic.resourceTrigger.queryCriteria", field
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
            if let Some(some) = self.value.r#previous.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("previous", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_previous", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#previous.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("previous", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#result_for_create.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("resultForCreate", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_resultForCreate", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#result_for_create.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("resultForCreate", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#current.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("current", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_current", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#current.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("current", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#result_for_delete.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("resultForDelete", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_resultForDelete", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#result_for_delete.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("resultForDelete", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#require_both.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("requireBoth", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_requireBoth", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#require_both.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("requireBoth", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<SubscriptionTopicResourceTriggerQueryCriteria>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<SubscriptionTopicResourceTriggerQueryCriteria>>
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
    for &mut DeserializationContext<SubscriptionTopicResourceTriggerQueryCriteria>
{
    type Value = SubscriptionTopicResourceTriggerQueryCriteria;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<SubscriptionTopicResourceTriggerQueryCriteria>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopicResourceTriggerQueryCriteria;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopicResourceTriggerQueryCriteria")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionTopicResourceTriggerQueryCriteria, V::Error>
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
                    #[serde(rename = "previous")]
                    Previous,
                    #[serde(rename = "_previous")]
                    PreviousPrimitiveElement,
                    #[serde(rename = "resultForCreate")]
                    ResultForCreate,
                    #[serde(rename = "_resultForCreate")]
                    ResultForCreatePrimitiveElement,
                    #[serde(rename = "current")]
                    Current,
                    #[serde(rename = "_current")]
                    CurrentPrimitiveElement,
                    #[serde(rename = "resultForDelete")]
                    ResultForDelete,
                    #[serde(rename = "_resultForDelete")]
                    ResultForDeletePrimitiveElement,
                    #[serde(rename = "requireBoth")]
                    RequireBoth,
                    #[serde(rename = "_requireBoth")]
                    RequireBothPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "previous",
                            "resultForCreate",
                            "current",
                            "resultForDelete",
                            "requireBoth",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#previous: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#result_for_create: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#current: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#result_for_delete: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#require_both: Option<fhirbolt_model::r5::types::Boolean> = None;
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
                        Field::Previous => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#previous.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("previous"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#previous.is_some() {
                                    return Err(serde::de::Error::duplicate_field("previous"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#previous = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PreviousPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#previous.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_previous"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("previous");
                            }
                        }
                        Field::ResultForCreate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#result_for_create.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultForCreate",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#result_for_create.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultForCreate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#result_for_create =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResultForCreatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#result_for_create.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_resultForCreate",
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
                                return unknown_field_error("resultForCreate");
                            }
                        }
                        Field::Current => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#current.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("current"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#current.is_some() {
                                    return Err(serde::de::Error::duplicate_field("current"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#current = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CurrentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#current.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_current"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("current");
                            }
                        }
                        Field::ResultForDelete => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#result_for_delete.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultForDelete",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#result_for_delete.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resultForDelete",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#result_for_delete =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResultForDeletePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#result_for_delete.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_resultForDelete",
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
                                return unknown_field_error("resultForDelete");
                            }
                        }
                        Field::RequireBoth => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#require_both.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requireBoth"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#require_both.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requireBoth"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#require_both =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RequireBothPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#require_both.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requireBoth"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requireBoth");
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
                Ok(SubscriptionTopicResourceTriggerQueryCriteria {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#previous,
                    r#result_for_create,
                    r#current,
                    r#result_for_delete,
                    r#require_both,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubscriptionTopicResourceTriggerQueryCriteria>>
{
    type Value = Box<SubscriptionTopicResourceTriggerQueryCriteria>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopicResourceTriggerQueryCriteria>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubscriptionTopicResourceTriggerQueryCriteria>>
{
    type Value = Vec<SubscriptionTopicResourceTriggerQueryCriteria>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<SubscriptionTopicResourceTriggerQueryCriteria>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopicResourceTriggerQueryCriteria>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    SubscriptionTopicResourceTriggerQueryCriteria,
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
use fhirbolt_model::r5::resources::SubscriptionTopicResourceTrigger;
impl serde::ser::Serialize for SerializationContext<&SubscriptionTopicResourceTrigger> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubscriptionTopic.resourceTrigger", field
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
            if self.value.r#resource.id.as_deref() == Some("$invalid") {
                return missing_field_error("resource");
            }
            if let Some(some) = self.value.r#resource.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("resource", &some?));
            }
            if self.value.r#resource.id.is_some() || !self.value.r#resource.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#resource.id.as_ref(),
                    extension: &self.value.r#resource.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_resource", ctx)));
            }
        } else if self.value.r#resource.id.as_deref() == Some("$invalid") {
            return missing_field_error("resource");
        } else {
            tri!(self.with_context(&self.value.r#resource, |ctx| state
                .serialize_entry("resource", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#supported_interaction.is_empty() {
                let values = tri!(self
                    .value
                    .r#supported_interaction
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("supportedInteraction", &values));
                }
                let requires_elements = self
                    .value
                    .r#supported_interaction
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#supported_interaction
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
                        .serialize_entry("_supportedInteraction", ctx)));
                }
            }
        } else if !self.value.r#supported_interaction.is_empty() {
            tri!(
                self.with_context(&self.value.r#supported_interaction, |ctx| state
                    .serialize_entry("supportedInteraction", ctx))
            );
        }
        if let Some(some) = self.value.r#query_criteria.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("queryCriteria", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#fhir_path_criteria.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("fhirPathCriteria", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_fhirPathCriteria", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#fhir_path_criteria.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("fhirPathCriteria", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubscriptionTopicResourceTrigger>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubscriptionTopicResourceTrigger>> {
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
    for &mut DeserializationContext<SubscriptionTopicResourceTrigger>
{
    type Value = SubscriptionTopicResourceTrigger;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubscriptionTopicResourceTrigger>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopicResourceTrigger;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopicResourceTrigger")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionTopicResourceTrigger, V::Error>
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
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    #[serde(rename = "supportedInteraction")]
                    SupportedInteraction,
                    #[serde(rename = "_supportedInteraction")]
                    SupportedInteractionPrimitiveElement,
                    #[serde(rename = "queryCriteria")]
                    QueryCriteria,
                    #[serde(rename = "fhirPathCriteria")]
                    FhirPathCriteria,
                    #[serde(rename = "_fhirPathCriteria")]
                    FhirPathCriteriaPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "description",
                            "resource",
                            "supportedInteraction",
                            "queryCriteria",
                            "fhirPathCriteria",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#supported_interaction: Option<Vec<fhirbolt_model::r5::types::Code>> =
                    None;
                let mut r#query_criteria: Option<
                    fhirbolt_model::r5::resources::SubscriptionTopicResourceTriggerQueryCriteria,
                > = None;
                let mut r#fhir_path_criteria: Option<fhirbolt_model::r5::types::String> = None;
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
                                    fhirbolt_model::r5::types::Markdown,
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
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
                            }
                        }
                        Field::SupportedInteraction => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#supported_interaction.get_or_insert(
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
                                        "supportedInteraction",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#supported_interaction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SupportedInteractionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#supported_interaction.get_or_insert(
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
                                        "_supportedInteraction",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("supportedInteraction");
                            }
                        }
                        Field::QueryCriteria => {
                            if r#query_criteria.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryCriteria"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: SubscriptionTopicResourceTriggerQueryCriteria > = self . 0 . transmute () ;
                            r#query_criteria =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::FhirPathCriteria => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#fhir_path_criteria.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fhirPathCriteria",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#fhir_path_criteria.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fhirPathCriteria",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#fhir_path_criteria =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FhirPathCriteriaPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#fhir_path_criteria.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_fhirPathCriteria",
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
                                return unknown_field_error("fhirPathCriteria");
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
                Ok(SubscriptionTopicResourceTrigger {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description,
                    r#resource: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#resource.unwrap_or(Default::default())
                    } else {
                        tri!(r#resource.ok_or(serde::de::Error::missing_field("resource")))
                    },
                    r#supported_interaction: r#supported_interaction.unwrap_or(vec![]),
                    r#query_criteria,
                    r#fhir_path_criteria,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubscriptionTopicResourceTrigger>>
{
    type Value = Box<SubscriptionTopicResourceTrigger>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopicResourceTrigger>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubscriptionTopicResourceTrigger>>
{
    type Value = Vec<SubscriptionTopicResourceTrigger>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubscriptionTopicResourceTrigger>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopicResourceTrigger>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubscriptionTopicResourceTrigger> =
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
use fhirbolt_model::r5::resources::SubscriptionTopicEventTrigger;
impl serde::ser::Serialize for SerializationContext<&SubscriptionTopicEventTrigger> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubscriptionTopic.eventTrigger", field
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
        if self.value.r#event.id.as_deref() == Some("$invalid") {
            return missing_field_error("event");
        } else {
            tri!(self.with_context(&self.value.r#event, |ctx| state
                .serialize_entry("event", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#resource.id.as_deref() == Some("$invalid") {
                return missing_field_error("resource");
            }
            if let Some(some) = self.value.r#resource.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("resource", &some?));
            }
            if self.value.r#resource.id.is_some() || !self.value.r#resource.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#resource.id.as_ref(),
                    extension: &self.value.r#resource.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_resource", ctx)));
            }
        } else if self.value.r#resource.id.as_deref() == Some("$invalid") {
            return missing_field_error("resource");
        } else {
            tri!(self.with_context(&self.value.r#resource, |ctx| state
                .serialize_entry("resource", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubscriptionTopicEventTrigger>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubscriptionTopicEventTrigger>> {
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
    for &mut DeserializationContext<SubscriptionTopicEventTrigger>
{
    type Value = SubscriptionTopicEventTrigger;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubscriptionTopicEventTrigger>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopicEventTrigger;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopicEventTrigger")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionTopicEventTrigger, V::Error>
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
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "event")]
                    Event,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "description",
                            "event",
                            "resource",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#event: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
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
                                    fhirbolt_model::r5::types::Markdown,
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
                        Field::Event => {
                            if r#event.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#event = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
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
                Ok(SubscriptionTopicEventTrigger {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description,
                    r#event: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#event.unwrap_or(Default::default())
                    } else {
                        tri!(r#event.ok_or(serde::de::Error::missing_field("event")))
                    },
                    r#resource: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#resource.unwrap_or(Default::default())
                    } else {
                        tri!(r#resource.ok_or(serde::de::Error::missing_field("resource")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubscriptionTopicEventTrigger>>
{
    type Value = Box<SubscriptionTopicEventTrigger>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopicEventTrigger>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubscriptionTopicEventTrigger>>
{
    type Value = Vec<SubscriptionTopicEventTrigger>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubscriptionTopicEventTrigger>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopicEventTrigger>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubscriptionTopicEventTrigger> =
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
use fhirbolt_model::r5::resources::SubscriptionTopicCanFilterBy;
impl serde::ser::Serialize for SerializationContext<&SubscriptionTopicCanFilterBy> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubscriptionTopic.canFilterBy", field
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
            if let Some(some) = self.value.r#resource.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("resource", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_resource", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#resource.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("resource", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#filter_parameter.id.as_deref() == Some("$invalid") {
                return missing_field_error("filterParameter");
            }
            if let Some(some) = self.value.r#filter_parameter.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("filterParameter", &some?));
            }
            if self.value.r#filter_parameter.id.is_some()
                || !self.value.r#filter_parameter.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#filter_parameter.id.as_ref(),
                    extension: &self.value.r#filter_parameter.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_filterParameter", ctx)));
            }
        } else if self.value.r#filter_parameter.id.as_deref() == Some("$invalid") {
            return missing_field_error("filterParameter");
        } else {
            tri!(
                self.with_context(&self.value.r#filter_parameter, |ctx| state
                    .serialize_entry("filterParameter", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#filter_definition.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("filterDefinition", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_filterDefinition", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#filter_definition.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("filterDefinition", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#comparator.is_empty() {
                let values = tri!(self
                    .value
                    .r#comparator
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("comparator", &values));
                }
                let requires_elements = self
                    .value
                    .r#comparator
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#comparator
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
                        .serialize_entry("_comparator", ctx)));
                }
            }
        } else if !self.value.r#comparator.is_empty() {
            tri!(self.with_context(&self.value.r#comparator, |ctx| state
                .serialize_entry("comparator", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#modifier.is_empty() {
                let values = tri!(self
                    .value
                    .r#modifier
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("modifier", &values));
                }
                let requires_elements = self
                    .value
                    .r#modifier
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#modifier
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
                        .serialize_entry("_modifier", ctx)));
                }
            }
        } else if !self.value.r#modifier.is_empty() {
            tri!(self.with_context(&self.value.r#modifier, |ctx| state
                .serialize_entry("modifier", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubscriptionTopicCanFilterBy>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubscriptionTopicCanFilterBy>> {
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
    for &mut DeserializationContext<SubscriptionTopicCanFilterBy>
{
    type Value = SubscriptionTopicCanFilterBy;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubscriptionTopicCanFilterBy>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopicCanFilterBy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopicCanFilterBy")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionTopicCanFilterBy, V::Error>
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
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    #[serde(rename = "filterParameter")]
                    FilterParameter,
                    #[serde(rename = "_filterParameter")]
                    FilterParameterPrimitiveElement,
                    #[serde(rename = "filterDefinition")]
                    FilterDefinition,
                    #[serde(rename = "_filterDefinition")]
                    FilterDefinitionPrimitiveElement,
                    #[serde(rename = "comparator")]
                    Comparator,
                    #[serde(rename = "_comparator")]
                    ComparatorPrimitiveElement,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "_modifier")]
                    ModifierPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "description",
                            "resource",
                            "filterParameter",
                            "filterDefinition",
                            "comparator",
                            "modifier",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#filter_parameter: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#filter_definition: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#comparator: Option<Vec<fhirbolt_model::r5::types::Code>> = None;
                let mut r#modifier: Option<Vec<fhirbolt_model::r5::types::Code>> = None;
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
                                    fhirbolt_model::r5::types::Markdown,
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
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
                            }
                        }
                        Field::FilterParameter => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#filter_parameter.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "filterParameter",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#filter_parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "filterParameter",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#filter_parameter =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FilterParameterPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#filter_parameter.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_filterParameter",
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
                                return unknown_field_error("filterParameter");
                            }
                        }
                        Field::FilterDefinition => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#filter_definition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "filterDefinition",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#filter_definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "filterDefinition",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#filter_definition =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::FilterDefinitionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#filter_definition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_filterDefinition",
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
                                return unknown_field_error("filterDefinition");
                            }
                        }
                        Field::Comparator => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#comparator.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("comparator"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#comparator.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ComparatorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#comparator.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_comparator"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("comparator");
                            }
                        }
                        Field::Modifier => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#modifier.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#modifier.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_modifier"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("modifier");
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
                Ok(SubscriptionTopicCanFilterBy {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#description,
                    r#resource,
                    r#filter_parameter: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#filter_parameter.unwrap_or(Default::default())
                    } else {
                        tri!(r#filter_parameter
                            .ok_or(serde::de::Error::missing_field("filterParameter")))
                    },
                    r#filter_definition,
                    r#comparator: r#comparator.unwrap_or(vec![]),
                    r#modifier: r#modifier.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubscriptionTopicCanFilterBy>>
{
    type Value = Box<SubscriptionTopicCanFilterBy>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopicCanFilterBy>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubscriptionTopicCanFilterBy>>
{
    type Value = Vec<SubscriptionTopicCanFilterBy>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubscriptionTopicCanFilterBy>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopicCanFilterBy>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubscriptionTopicCanFilterBy> =
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
use fhirbolt_model::r5::resources::SubscriptionTopicNotificationShape;
impl serde::ser::Serialize for SerializationContext<&SubscriptionTopicNotificationShape> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubscriptionTopic.notificationShape", field
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
            if self.value.r#resource.id.as_deref() == Some("$invalid") {
                return missing_field_error("resource");
            }
            if let Some(some) = self.value.r#resource.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("resource", &some?));
            }
            if self.value.r#resource.id.is_some() || !self.value.r#resource.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#resource.id.as_ref(),
                    extension: &self.value.r#resource.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_resource", ctx)));
            }
        } else if self.value.r#resource.id.as_deref() == Some("$invalid") {
            return missing_field_error("resource");
        } else {
            tri!(self.with_context(&self.value.r#resource, |ctx| state
                .serialize_entry("resource", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#include.is_empty() {
                let values = tri!(self
                    .value
                    .r#include
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("include", &values));
                }
                let requires_elements = self
                    .value
                    .r#include
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#include
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
                        .serialize_entry("_include", ctx)));
                }
            }
        } else if !self.value.r#include.is_empty() {
            tri!(self.with_context(&self.value.r#include, |ctx| state
                .serialize_entry("include", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#rev_include.is_empty() {
                let values = tri!(self
                    .value
                    .r#rev_include
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("revInclude", &values));
                }
                let requires_elements = self
                    .value
                    .r#rev_include
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#rev_include
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
                        .serialize_entry("_revInclude", ctx)));
                }
            }
        } else if !self.value.r#rev_include.is_empty() {
            tri!(self.with_context(&self.value.r#rev_include, |ctx| state
                .serialize_entry("revInclude", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubscriptionTopicNotificationShape>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubscriptionTopicNotificationShape>> {
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
    for &mut DeserializationContext<SubscriptionTopicNotificationShape>
{
    type Value = SubscriptionTopicNotificationShape;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubscriptionTopicNotificationShape>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopicNotificationShape;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopicNotificationShape")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionTopicNotificationShape, V::Error>
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
                    #[serde(rename = "resource")]
                    Resource,
                    #[serde(rename = "_resource")]
                    ResourcePrimitiveElement,
                    #[serde(rename = "include")]
                    Include,
                    #[serde(rename = "_include")]
                    IncludePrimitiveElement,
                    #[serde(rename = "revInclude")]
                    RevInclude,
                    #[serde(rename = "_revInclude")]
                    RevIncludePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "resource",
                            "include",
                            "revInclude",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#resource: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#include: Option<Vec<fhirbolt_model::r5::types::String>> = None;
                let mut r#rev_include: Option<Vec<fhirbolt_model::r5::types::String>> = None;
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
                        Field::Resource => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#resource = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResourcePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#resource.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_resource"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("resource");
                            }
                        }
                        Field::Include => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#include.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("include"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#include.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::IncludePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#include.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_include"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("include");
                            }
                        }
                        Field::RevInclude => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#rev_include.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("revInclude"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#rev_include.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::RevIncludePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#rev_include.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_revInclude"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("revInclude");
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
                Ok(SubscriptionTopicNotificationShape {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#resource: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#resource.unwrap_or(Default::default())
                    } else {
                        tri!(r#resource.ok_or(serde::de::Error::missing_field("resource")))
                    },
                    r#include: r#include.unwrap_or(vec![]),
                    r#rev_include: r#rev_include.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubscriptionTopicNotificationShape>>
{
    type Value = Box<SubscriptionTopicNotificationShape>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopicNotificationShape>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubscriptionTopicNotificationShape>>
{
    type Value = Vec<SubscriptionTopicNotificationShape>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubscriptionTopicNotificationShape>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopicNotificationShape>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubscriptionTopicNotificationShape> =
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
use fhirbolt_model::r5::resources::SubscriptionTopic;
impl crate::Resource for SubscriptionTopic {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&SubscriptionTopic> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubscriptionTopic", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "SubscriptionTopic"));
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
        if self.output == crate::context::Format::Json {
            if self.value.r#url.id.as_deref() == Some("$invalid") {
                return missing_field_error("url");
            }
            if let Some(some) = self.value.r#url.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("url", &some?));
            }
            if self.value.r#url.id.is_some() || !self.value.r#url.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#url.id.as_ref(),
                    extension: &self.value.r#url.extension,
                };
                tri!(
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_url", ctx))
                );
            }
        } else if self.value.r#url.id.as_deref() == Some("$invalid") {
            return missing_field_error("url");
        } else {
            tri!(self.with_context(&self.value.r#url, |ctx| state.serialize_entry("url", ctx)));
        }
        if !self.value.r#identifier.is_empty() {
            tri!(self.with_context(&self.value.r#identifier, |ctx| state
                .serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("version", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_version", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#version.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("version", ctx)));
        }
        {
            use fhirbolt_model::r5::resources::SubscriptionTopicVersionAlgorithm as _Enum;
            if let Some(some) = self.value.r#version_algorithm.as_ref() {
                match some {
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("versionAlgorithmString", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_versionAlgorithmString", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("versionAlgorithmString", ctx)));
                        }
                    }
                    _Enum::Coding(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("versionAlgorithmCoding", ctx)));
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("version_algorithm is invalid"))
                    }
                }
            }
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
            if let Some(some) = self.value.r#title.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("title", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_title", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#title.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("title", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#derived_from.is_empty() {
                let values = tri!(self
                    .value
                    .r#derived_from
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("derivedFrom", &values));
                }
                let requires_elements = self
                    .value
                    .r#derived_from
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#derived_from
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
                        .serialize_entry("_derivedFrom", ctx)));
                }
            }
        } else if !self.value.r#derived_from.is_empty() {
            tri!(self.with_context(&self.value.r#derived_from, |ctx| state
                .serialize_entry("derivedFrom", ctx)));
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#experimental.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("experimental", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_experimental", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#experimental.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("experimental", ctx)));
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#publisher.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("publisher", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_publisher", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#publisher.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("publisher", ctx)));
        }
        if !self.value.r#contact.is_empty() {
            tri!(self.with_context(&self.value.r#contact, |ctx| state
                .serialize_entry("contact", ctx)));
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
        if !self.value.r#use_context.is_empty() {
            tri!(self.with_context(&self.value.r#use_context, |ctx| state
                .serialize_entry("useContext", ctx)));
        }
        if !self.value.r#jurisdiction.is_empty() {
            tri!(self.with_context(&self.value.r#jurisdiction, |ctx| state
                .serialize_entry("jurisdiction", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#purpose.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("purpose", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_purpose", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#purpose.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("purpose", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#copyright.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("copyright", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_copyright", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#copyright.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("copyright", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#copyright_label.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("copyrightLabel", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_copyrightLabel", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#copyright_label.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("copyrightLabel", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#approval_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("approvalDate", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_approvalDate", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#approval_date.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("approvalDate", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#last_review_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("lastReviewDate", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_lastReviewDate", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#last_review_date.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("lastReviewDate", ctx)));
        }
        if let Some(some) = self.value.r#effective_period.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("effectivePeriod", ctx)));
        }
        if !self.value.r#resource_trigger.is_empty() {
            tri!(
                self.with_context(&self.value.r#resource_trigger, |ctx| state
                    .serialize_entry("resourceTrigger", ctx))
            );
        }
        if !self.value.r#event_trigger.is_empty() {
            tri!(self.with_context(&self.value.r#event_trigger, |ctx| state
                .serialize_entry("eventTrigger", ctx)));
        }
        if !self.value.r#can_filter_by.is_empty() {
            tri!(self.with_context(&self.value.r#can_filter_by, |ctx| state
                .serialize_entry("canFilterBy", ctx)));
        }
        if !self.value.r#notification_shape.is_empty() {
            tri!(
                self.with_context(&self.value.r#notification_shape, |ctx| state
                    .serialize_entry("notificationShape", ctx))
            );
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubscriptionTopic>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubscriptionTopic>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<SubscriptionTopic> {
    type Value = SubscriptionTopic;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SubscriptionTopic> {
    type Value = SubscriptionTopic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubscriptionTopic>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubscriptionTopic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionTopic")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubscriptionTopic, V::Error>
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
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "_version")]
                    VersionPrimitiveElement,
                    #[serde(rename = "versionAlgorithmString")]
                    VersionAlgorithmString,
                    #[serde(rename = "_versionAlgorithmString")]
                    VersionAlgorithmStringPrimitiveElement,
                    #[serde(rename = "versionAlgorithmCoding")]
                    VersionAlgorithmCoding,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "derivedFrom")]
                    DerivedFrom,
                    #[serde(rename = "_derivedFrom")]
                    DerivedFromPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "experimental")]
                    Experimental,
                    #[serde(rename = "_experimental")]
                    ExperimentalPrimitiveElement,
                    #[serde(rename = "date")]
                    Date,
                    #[serde(rename = "_date")]
                    DatePrimitiveElement,
                    #[serde(rename = "publisher")]
                    Publisher,
                    #[serde(rename = "_publisher")]
                    PublisherPrimitiveElement,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "useContext")]
                    UseContext,
                    #[serde(rename = "jurisdiction")]
                    Jurisdiction,
                    #[serde(rename = "purpose")]
                    Purpose,
                    #[serde(rename = "_purpose")]
                    PurposePrimitiveElement,
                    #[serde(rename = "copyright")]
                    Copyright,
                    #[serde(rename = "_copyright")]
                    CopyrightPrimitiveElement,
                    #[serde(rename = "copyrightLabel")]
                    CopyrightLabel,
                    #[serde(rename = "_copyrightLabel")]
                    CopyrightLabelPrimitiveElement,
                    #[serde(rename = "approvalDate")]
                    ApprovalDate,
                    #[serde(rename = "_approvalDate")]
                    ApprovalDatePrimitiveElement,
                    #[serde(rename = "lastReviewDate")]
                    LastReviewDate,
                    #[serde(rename = "_lastReviewDate")]
                    LastReviewDatePrimitiveElement,
                    #[serde(rename = "effectivePeriod")]
                    EffectivePeriod,
                    #[serde(rename = "resourceTrigger")]
                    ResourceTrigger,
                    #[serde(rename = "eventTrigger")]
                    EventTrigger,
                    #[serde(rename = "canFilterBy")]
                    CanFilterBy,
                    #[serde(rename = "notificationShape")]
                    NotificationShape,
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
                            "url",
                            "identifier",
                            "version",
                            "versionAlgorithmString",
                            "versionAlgorithmCoding",
                            "name",
                            "title",
                            "derivedFrom",
                            "status",
                            "experimental",
                            "date",
                            "publisher",
                            "contact",
                            "description",
                            "useContext",
                            "jurisdiction",
                            "purpose",
                            "copyright",
                            "copyrightLabel",
                            "approvalDate",
                            "lastReviewDate",
                            "effectivePeriod",
                            "resourceTrigger",
                            "eventTrigger",
                            "canFilterBy",
                            "notificationShape",
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
                let mut r#url: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#version: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#version_algorithm: Option<
                    fhirbolt_model::r5::resources::SubscriptionTopicVersionAlgorithm,
                > = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#derived_from: Option<Vec<fhirbolt_model::r5::types::Canonical>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#experimental: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#publisher: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r5::types::ContactDetail>> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#use_context: Option<Vec<fhirbolt_model::r5::types::UsageContext>> = None;
                let mut r#jurisdiction: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#purpose: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#copyright: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#copyright_label: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#approval_date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#last_review_date: Option<fhirbolt_model::r5::types::Date> = None;
                let mut r#effective_period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#resource_trigger: Option<
                    Vec<fhirbolt_model::r5::resources::SubscriptionTopicResourceTrigger>,
                > = None;
                let mut r#event_trigger: Option<
                    Vec<fhirbolt_model::r5::resources::SubscriptionTopicEventTrigger>,
                > = None;
                let mut r#can_filter_by: Option<
                    Vec<fhirbolt_model::r5::resources::SubscriptionTopicCanFilterBy>,
                > = None;
                let mut r#notification_shape: Option<
                    Vec<fhirbolt_model::r5::resources::SubscriptionTopicNotificationShape>,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "SubscriptionTopic" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SubscriptionTopic",
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
                        Field::Url => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#url = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
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
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#version = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::VersionAlgorithmString => {
                            use fhirbolt_model::r5::resources::SubscriptionTopicVersionAlgorithm as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#version_algorithm
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "versionAlgorithmString",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "versionAlgorithm[x]",
                                    ));
                                }
                            } else {
                                if r#version_algorithm.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "versionAlgorithmString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#version_algorithm = Some(_Enum::String(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::VersionAlgorithmStringPrimitiveElement => {
                            use fhirbolt_model::r5::resources::SubscriptionTopicVersionAlgorithm as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#version_algorithm
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_versionAlgorithmString",
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_versionAlgorithm[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("versionAlgorithmString");
                            }
                        }
                        Field::VersionAlgorithmCoding => {
                            use fhirbolt_model::r5::resources::SubscriptionTopicVersionAlgorithm as _Enum;
                            if r#version_algorithm.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "versionAlgorithmCoding",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Coding>,
                            > = self.0.transmute();
                            r#version_algorithm = Some(_Enum::Coding(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
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
                        Field::Title => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#title = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::TitlePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::DerivedFrom => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#derived_from.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("derivedFrom"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#derived_from.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DerivedFromPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#derived_from.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_derivedFrom"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("derivedFrom");
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
                        Field::Experimental => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#experimental.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#experimental =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ExperimentalPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_experimental"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("experimental");
                            }
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
                        Field::Publisher => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#publisher =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PublisherPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publisher");
                            }
                        }
                        Field::Contact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::ContactDetail>,
                                > = self.0.transmute();
                                r#contact = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    fhirbolt_model::r5::types::Markdown,
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
                        Field::UseContext => {
                            if self.0.from == crate::context::Format::Json {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::UsageContext>,
                                > = self.0.transmute();
                                r#use_context =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::UsageContext,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from == crate::context::Format::Json {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#jurisdiction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#jurisdiction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Purpose => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#purpose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#purpose = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PurposePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Copyright => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#copyright.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#copyright =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CopyrightPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyright");
                            }
                        }
                        Field::CopyrightLabel => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright_label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "copyrightLabel",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#copyright_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "copyrightLabel",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#copyright_label =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CopyrightLabelPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright_label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_copyrightLabel",
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
                                return unknown_field_error("copyrightLabel");
                            }
                        }
                        Field::ApprovalDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#approval_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#approval_date =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ApprovalDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_approvalDate"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("approvalDate");
                            }
                        }
                        Field::LastReviewDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#last_review_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Date,
                                > = self.0.transmute();
                                r#last_review_date =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LastReviewDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastReviewDate",
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
                                return unknown_field_error("lastReviewDate");
                            }
                        }
                        Field::EffectivePeriod => {
                            if r#effective_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#effective_period =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ResourceTrigger => {
                            if self.0.from == crate::context::Format::Json {
                                if r#resource_trigger.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "resourceTrigger",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: SubscriptionTopicResourceTrigger >> = self . 0 . transmute () ;
                                r#resource_trigger =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#resource_trigger.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::SubscriptionTopicResourceTrigger,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::EventTrigger => {
                            if self.0.from == crate::context::Format::Json {
                                if r#event_trigger.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventTrigger"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: SubscriptionTopicEventTrigger >> = self . 0 . transmute () ;
                                r#event_trigger =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#event_trigger.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::SubscriptionTopicEventTrigger,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::CanFilterBy => {
                            if self.0.from == crate::context::Format::Json {
                                if r#can_filter_by.is_some() {
                                    return Err(serde::de::Error::duplicate_field("canFilterBy"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::SubscriptionTopicCanFilterBy,
                                    >,
                                > = self.0.transmute();
                                r#can_filter_by =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#can_filter_by.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::SubscriptionTopicCanFilterBy,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::NotificationShape => {
                            if self.0.from == crate::context::Format::Json {
                                if r#notification_shape.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "notificationShape",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: SubscriptionTopicNotificationShape >> = self . 0 . transmute () ;
                                r#notification_shape =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#notification_shape.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: SubscriptionTopicNotificationShape > = self . 0 . transmute () ;
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
                Ok(SubscriptionTopic {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#url.unwrap_or(Default::default())
                    } else {
                        tri!(r#url.ok_or(serde::de::Error::missing_field("url")))
                    },
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#version,
                    r#version_algorithm,
                    r#name,
                    r#title,
                    r#derived_from: r#derived_from.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        tri!(r#status.ok_or(serde::de::Error::missing_field("status")))
                    },
                    r#experimental,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#copyright,
                    r#copyright_label,
                    r#approval_date,
                    r#last_review_date,
                    r#effective_period,
                    r#resource_trigger: r#resource_trigger.unwrap_or(vec![]),
                    r#event_trigger: r#event_trigger.unwrap_or(vec![]),
                    r#can_filter_by: r#can_filter_by.unwrap_or(vec![]),
                    r#notification_shape: r#notification_shape.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SubscriptionTopic>> {
    type Value = Box<SubscriptionTopic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubscriptionTopic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SubscriptionTopic>> {
    type Value = Vec<SubscriptionTopic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubscriptionTopic>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubscriptionTopic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubscriptionTopic> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
