// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "A slot of time on a schedule that may be available for booking appointments."]
#[derive(Default, Debug, Clone)]
pub struct Slot {
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
    #[doc = "External Ids for this item."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A broad categorization of the service that is to be performed during this appointment."]
    pub r#service_category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides the value provided on the availability resource."]
    pub r#service_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specialty of a practitioner that would be required to perform the service requested in this appointment."]
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The style of appointment or patient that may be booked in the slot (not service type)."]
    pub r#appointment_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The schedule resource that this slot defines an interval of status information."]
    pub r#schedule: Box<super::super::types::Reference>,
    #[doc = "busy | free | busy-unavailable | busy-tentative | entered-in-error."]
    pub r#status: super::super::types::Code,
    #[doc = "Date/Time that the slot is to begin."]
    pub r#start: super::super::types::Instant,
    #[doc = "Date/Time that the slot is to conclude."]
    pub r#end: super::super::types::Instant,
    #[doc = "This slot has already been overbooked, appointments are unlikely to be accepted for this time."]
    pub r#overbooked: Option<super::super::types::Boolean>,
    #[doc = "Comments on the slot to describe any extended information. Such as custom constraints on the slot."]
    pub r#comment: Option<super::super::types::String>,
}
impl crate::AnyResource for Slot {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Slot")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if !self.r#service_category.is_empty() {
                state.serialize_entry("serviceCategory", &self.r#service_category)?;
            }
            if !self.r#service_type.is_empty() {
                state.serialize_entry("serviceType", &self.r#service_type)?;
            }
            if !self.r#specialty.is_empty() {
                state.serialize_entry("specialty", &self.r#specialty)?;
            }
            if let Some(some) = self.r#appointment_type.as_ref() {
                state.serialize_entry("appointmentType", some)?;
            }
            state.serialize_entry("schedule", &self.r#schedule)?;
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#start.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("start", &some)?;
                }
                if self.r#start.id.is_some() || !self.r#start.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#start.id.as_ref(),
                        extension: &self.r#start.extension,
                    };
                    state.serialize_entry("_start", &primitive_element)?;
                }
            } else {
                state.serialize_entry("start", &self.r#start)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#end.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("end", &some)?;
                }
                if self.r#end.id.is_some() || !self.r#end.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#end.id.as_ref(),
                        extension: &self.r#end.extension,
                    };
                    state.serialize_entry("_end", &primitive_element)?;
                }
            } else {
                state.serialize_entry("end", &self.r#end)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#overbooked.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("overbooked", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_overbooked", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#overbooked.as_ref() {
                    state.serialize_entry("overbooked", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#comment.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("comment", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_comment", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#comment.as_ref() {
                    state.serialize_entry("comment", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Slot {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "serviceCategory")]
            ServiceCategory,
            #[serde(rename = "serviceType")]
            ServiceType,
            #[serde(rename = "specialty")]
            Specialty,
            #[serde(rename = "appointmentType")]
            AppointmentType,
            #[serde(rename = "schedule")]
            Schedule,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "start")]
            Start,
            #[serde(rename = "_start")]
            StartPrimitiveElement,
            #[serde(rename = "end")]
            End,
            #[serde(rename = "_end")]
            EndPrimitiveElement,
            #[serde(rename = "overbooked")]
            Overbooked,
            #[serde(rename = "_overbooked")]
            OverbookedPrimitiveElement,
            #[serde(rename = "comment")]
            Comment,
            #[serde(rename = "_comment")]
            CommentPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Slot;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Slot")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Slot, V::Error>
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
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#service_category: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#service_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#specialty: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#appointment_type: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#schedule: Option<Box<super::super::types::Reference>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#start: Option<super::super::types::Instant> = None;
                let mut r#end: Option<super::super::types::Instant> = None;
                let mut r#overbooked: Option<super::super::types::Boolean> = None;
                let mut r#comment: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Slot" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Slot",
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
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
                                    let some = r#language.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#language.is_some() {
                                        return Err(serde::de::Error::duplicate_field("language"));
                                    }
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::ServiceCategory => {
                                if r#service_category.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "serviceCategory",
                                    ));
                                }
                                r#service_category = Some(map_access.next_value()?);
                            }
                            Field::ServiceType => {
                                if r#service_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("serviceType"));
                                }
                                r#service_type = Some(map_access.next_value()?);
                            }
                            Field::Specialty => {
                                if r#specialty.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specialty"));
                                }
                                r#specialty = Some(map_access.next_value()?);
                            }
                            Field::AppointmentType => {
                                if r#appointment_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "appointmentType",
                                    ));
                                }
                                r#appointment_type = Some(map_access.next_value()?);
                            }
                            Field::Schedule => {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::Start => {
                                if _ctx.from_json {
                                    let some = r#start.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("start"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#start.is_some() {
                                        return Err(serde::de::Error::duplicate_field("start"));
                                    }
                                    r#start = Some(map_access.next_value()?);
                                }
                            }
                            Field::StartPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#start.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_start"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "start",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::End => {
                                if _ctx.from_json {
                                    let some = r#end.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("end"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#end.is_some() {
                                        return Err(serde::de::Error::duplicate_field("end"));
                                    }
                                    r#end = Some(map_access.next_value()?);
                                }
                            }
                            Field::EndPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#end.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_end"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "end",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::Overbooked => {
                                if _ctx.from_json {
                                    let some = r#overbooked.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "overbooked",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#overbooked.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "overbooked",
                                        ));
                                    }
                                    r#overbooked = Some(map_access.next_value()?);
                                }
                            }
                            Field::OverbookedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#overbooked.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_overbooked",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "overbooked",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::Comment => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#comment.is_some() {
                                        return Err(serde::de::Error::duplicate_field("comment"));
                                    }
                                    r#comment = Some(map_access.next_value()?);
                                }
                            }
                            Field::CommentPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#comment.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_comment"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "comment",
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
                                            "serviceCategory",
                                            "serviceType",
                                            "specialty",
                                            "appointmentType",
                                            "schedule",
                                            "status",
                                            "start",
                                            "end",
                                            "overbooked",
                                            "comment",
                                        ],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
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
                                        "identifier",
                                        "serviceCategory",
                                        "serviceType",
                                        "specialty",
                                        "appointmentType",
                                        "schedule",
                                        "status",
                                        "start",
                                        "end",
                                        "overbooked",
                                        "comment",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Slot {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#service_category: r#service_category.unwrap_or(vec![]),
                        r#service_type: r#service_type.unwrap_or(vec![]),
                        r#specialty: r#specialty.unwrap_or(vec![]),
                        r#appointment_type,
                        r#schedule: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#schedule.unwrap_or(Default::default())
                        } else {
                            r#schedule.ok_or(serde::de::Error::missing_field("schedule"))?
                        },
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#start: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#start.unwrap_or(Default::default())
                        } else {
                            r#start.ok_or(serde::de::Error::missing_field("start"))?
                        },
                        r#end: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#end.unwrap_or(Default::default())
                        } else {
                            r#end.ok_or(serde::de::Error::missing_field("end"))?
                        },
                        r#overbooked,
                        r#comment,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
