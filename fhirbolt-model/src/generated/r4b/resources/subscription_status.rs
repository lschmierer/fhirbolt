// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Detailed information about events relevant to this subscription notification."]
#[derive(Default, Debug, Clone)]
pub struct SubscriptionStatusNotificationEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The sequential number of this event in this subscription context. Note that this value is a 64-bit integer value, encoded as a string."]
    pub r#event_number: super::super::types::String,
    #[doc = "The actual time this event occured on the server."]
    pub r#timestamp: Option<super::super::types::Instant>,
    #[doc = "The focus of this event. While this will usually be a reference to the focus resource of the event, it MAY contain a reference to a non-FHIR object."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional context information for this event. Generally, this will contain references to additional resources included with the event (e.g., the Patient relevant to an Encounter), however it MAY refer to non-FHIR objects."]
    pub r#additional_context: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for SubscriptionStatusNotificationEvent {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for SubscriptionStatusNotificationEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#event_number.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("eventNumber", &some)?;
        }
        if self.r#event_number.id.is_some() || !self.r#event_number.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#event_number.id,
                extension: &self.r#event_number.extension,
            };
            state.serialize_entry("_eventNumber", &primitive_element)?;
        }
        if let Some(some) = self.r#timestamp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("timestamp", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_timestamp", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#focus.as_ref() {
            state.serialize_entry("focus", some)?;
        }
        if !self.r#additional_context.is_empty() {
            state.serialize_entry("additionalContext", &self.r#additional_context)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubscriptionStatusNotificationEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
            #[serde(rename = "eventNumber")]
            EventNumber,
            #[serde(rename = "_eventNumber")]
            EventNumberPrimitiveElement,
            #[serde(rename = "timestamp")]
            Timestamp,
            #[serde(rename = "_timestamp")]
            TimestampPrimitiveElement,
            #[serde(rename = "focus")]
            Focus,
            #[serde(rename = "additionalContext")]
            AdditionalContext,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionStatusNotificationEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionStatusNotificationEvent")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubscriptionStatusNotificationEvent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#event_number: Option<super::super::types::String> = None;
                let mut r#timestamp: Option<super::super::types::Instant> = None;
                let mut r#focus: Option<Box<super::super::types::Reference>> = None;
                let mut r#additional_context: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::EventNumber => {
                                let some = r#event_number.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventNumber"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::EventNumberPrimitiveElement => {
                                let some = r#event_number.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_eventNumber"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Timestamp => {
                                let some = r#timestamp.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timestamp"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TimestampPrimitiveElement => {
                                let some = r#timestamp.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timestamp"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Focus => {
                                if r#focus.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focus"));
                                }
                                r#focus = Some(map_access.next_value()?);
                            }
                            Field::AdditionalContext => {
                                if r#additional_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalContext",
                                    ));
                                }
                                r#additional_context = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "eventNumber",
                                        "timestamp",
                                        "focus",
                                        "additionalContext",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubscriptionStatusNotificationEvent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#event_number: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#event_number.unwrap_or(Default::default())
                        } else {
                            r#event_number.ok_or(serde::de::Error::missing_field("eventNumber"))?
                        },
                        r#timestamp,
                        r#focus,
                        r#additional_context: r#additional_context.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The SubscriptionStatus resource describes the state of a Subscription during notifications."]
#[derive(Default, Debug, Clone)]
pub struct SubscriptionStatus {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The status of the subscription, which marks the server state for managing the subscription."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The type of event being conveyed with this notificaiton."]
    pub r#type: super::super::types::Code,
    #[doc = "The total number of actual events which have been generated since the Subscription was created (inclusive of this notification) - regardless of how many have been successfully communicated.  This number is NOT incremented for handshake and heartbeat notifications."]
    pub r#events_since_subscription_start: Option<super::super::types::String>,
    #[doc = "Detailed information about events relevant to this subscription notification."]
    pub r#notification_event: Vec<SubscriptionStatusNotificationEvent>,
    #[doc = "The reference to the Subscription which generated this notification."]
    pub r#subscription: Box<super::super::types::Reference>,
    #[doc = "The reference to the SubscriptionTopic for the Subscription which generated this notification."]
    pub r#topic: Option<super::super::types::Canonical>,
    #[doc = "A record of errors that occurred when the server processed a notification."]
    pub r#error: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for SubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubscriptionStatus")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("type", &some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#events_since_subscription_start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("eventsSinceSubscriptionStart", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_eventsSinceSubscriptionStart", &primitive_element)?;
            }
        }
        if !self.r#notification_event.is_empty() {
            state.serialize_entry("notificationEvent", &self.r#notification_event)?;
        }
        state.serialize_entry("subscription", &self.r#subscription)?;
        if let Some(some) = self.r#topic.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("topic", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_topic", &primitive_element)?;
            }
        }
        if !self.r#error.is_empty() {
            state.serialize_entry("error", &self.r#error)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "eventsSinceSubscriptionStart")]
            EventsSinceSubscriptionStart,
            #[serde(rename = "_eventsSinceSubscriptionStart")]
            EventsSinceSubscriptionStartPrimitiveElement,
            #[serde(rename = "notificationEvent")]
            NotificationEvent,
            #[serde(rename = "subscription")]
            Subscription,
            #[serde(rename = "topic")]
            Topic,
            #[serde(rename = "_topic")]
            TopicPrimitiveElement,
            #[serde(rename = "error")]
            Error,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubscriptionStatus;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubscriptionStatus")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubscriptionStatus, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#events_since_subscription_start: Option<super::super::types::String> =
                    None;
                let mut r#notification_event: Option<Vec<SubscriptionStatusNotificationEvent>> =
                    None;
                let mut r#subscription: Option<Box<super::super::types::Reference>> = None;
                let mut r#topic: Option<super::super::types::Canonical> = None;
                let mut r#error: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SubscriptionStatus" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SubscriptionStatus",
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
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LanguagePrimitiveElement => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TypePrimitiveElement => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::EventsSinceSubscriptionStart => {
                                let some = r#events_since_subscription_start
                                    .get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "eventsSinceSubscriptionStart",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::EventsSinceSubscriptionStartPrimitiveElement => {
                                let some = r#events_since_subscription_start
                                    .get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_eventsSinceSubscriptionStart",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::NotificationEvent => {
                                if r#notification_event.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "notificationEvent",
                                    ));
                                }
                                r#notification_event = Some(map_access.next_value()?);
                            }
                            Field::Subscription => {
                                if r#subscription.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subscription"));
                                }
                                r#subscription = Some(map_access.next_value()?);
                            }
                            Field::Topic => {
                                let some = r#topic.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("topic"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TopicPrimitiveElement => {
                                let some = r#topic.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_topic"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Error => {
                                if r#error.is_some() {
                                    return Err(serde::de::Error::duplicate_field("error"));
                                }
                                r#error = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "status",
                                        "type",
                                        "eventsSinceSubscriptionStart",
                                        "notificationEvent",
                                        "subscription",
                                        "topic",
                                        "error",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SubscriptionStatus {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#status,
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#events_since_subscription_start,
                        r#notification_event: r#notification_event.unwrap_or(vec![]),
                        r#subscription: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#subscription.unwrap_or(Default::default())
                        } else {
                            r#subscription.ok_or(serde::de::Error::missing_field("subscription"))?
                        },
                        r#topic,
                        r#error: r#error.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
