// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MediaCreated {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for MediaCreated {
    fn default() -> MediaCreated {
        MediaCreated::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct Media {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modality: Option<Box<super::super::types::CodeableConcept>>,
    pub r#view: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#created: Option<MediaCreated>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#operator: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#device_name: Option<super::super::types::String>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#height: Option<super::super::types::PositiveInt>,
    pub r#width: Option<super::super::types::PositiveInt>,
    pub r#frames: Option<super::super::types::PositiveInt>,
    pub r#duration: Option<super::super::types::Decimal>,
    pub r#content: Box<super::super::types::Attachment>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for Media {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Media")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
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
                state.serialize_entry("language", some)?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if !self.r#part_of.is_empty() {
            state.serialize_entry("partOf", &self.r#part_of)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#modality.as_ref() {
            state.serialize_entry("modality", some)?;
        }
        if let Some(some) = self.r#view.as_ref() {
            state.serialize_entry("view", some)?;
        }
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#created.as_ref() {
            match some {
                MediaCreated::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("createdDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_createdDateTime", &primitive_element)?;
                    }
                }
                MediaCreated::Period(ref value) => {
                    state.serialize_entry("createdPeriod", value)?;
                }
                MediaCreated::Invalid => {
                    return Err(serde::ser::Error::custom("created is invalid"))
                }
            }
        }
        if let Some(some) = self.r#issued.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("issued", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_issued", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#operator.as_ref() {
            state.serialize_entry("operator", some)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if let Some(some) = self.r#device_name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("deviceName", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_deviceName", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#device.as_ref() {
            state.serialize_entry("device", some)?;
        }
        if let Some(some) = self.r#height.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("height", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_height", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#width.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("width", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_width", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#frames.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("frames", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_frames", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("duration", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_duration", &primitive_element)?;
            }
        }
        state.serialize_entry("content", &self.r#content)?;
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Media {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "modality")]
            Modality,
            #[serde(rename = "view")]
            View,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "createdDateTime")]
            CreatedDateTime,
            #[serde(rename = "_createdDateTime")]
            CreatedDateTimePrimitiveElement,
            #[serde(rename = "createdPeriod")]
            CreatedPeriod,
            #[serde(rename = "issued")]
            Issued,
            #[serde(rename = "_issued")]
            IssuedPrimitiveElement,
            #[serde(rename = "operator")]
            Operator,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "deviceName")]
            DeviceName,
            #[serde(rename = "_deviceName")]
            DeviceNamePrimitiveElement,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "height")]
            Height,
            #[serde(rename = "_height")]
            HeightPrimitiveElement,
            #[serde(rename = "width")]
            Width,
            #[serde(rename = "_width")]
            WidthPrimitiveElement,
            #[serde(rename = "frames")]
            Frames,
            #[serde(rename = "_frames")]
            FramesPrimitiveElement,
            #[serde(rename = "duration")]
            Duration,
            #[serde(rename = "_duration")]
            DurationPrimitiveElement,
            #[serde(rename = "content")]
            Content,
            #[serde(rename = "note")]
            Note,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Media;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Media")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Media, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#modality: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#view: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#created: Option<MediaCreated> = None;
                let mut r#issued: Option<super::super::types::Instant> = None;
                let mut r#operator: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#device_name: Option<super::super::types::String> = None;
                let mut r#device: Option<Box<super::super::types::Reference>> = None;
                let mut r#height: Option<super::super::types::PositiveInt> = None;
                let mut r#width: Option<super::super::types::PositiveInt> = None;
                let mut r#frames: Option<super::super::types::PositiveInt> = None;
                let mut r#duration: Option<super::super::types::Decimal> = None;
                let mut r#content: Option<Box<super::super::types::Attachment>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
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
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
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
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::BasedOn => {
                            if r#based_on.is_some() {
                                return Err(serde::de::Error::duplicate_field("basedOn"));
                            }
                            r#based_on = Some(map_access.next_value()?);
                        }
                        Field::PartOf => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Modality => {
                            if r#modality.is_some() {
                                return Err(serde::de::Error::duplicate_field("modality"));
                            }
                            r#modality = Some(map_access.next_value()?);
                        }
                        Field::View => {
                            if r#view.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            r#view = Some(map_access.next_value()?);
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        Field::CreatedDateTime => {
                            let r#enum =
                                r#created.get_or_insert(MediaCreated::DateTime(Default::default()));
                            if let MediaCreated::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "createdDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("created[x]"));
                            }
                        }
                        Field::CreatedDateTimePrimitiveElement => {
                            let r#enum =
                                r#created.get_or_insert(MediaCreated::DateTime(Default::default()));
                            if let MediaCreated::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_createdDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_created[x]"));
                            }
                        }
                        Field::CreatedPeriod => {
                            if r#created.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdPeriod"));
                            }
                            r#created = Some(MediaCreated::Period(map_access.next_value()?));
                        }
                        Field::Issued => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("issued"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IssuedPrimitiveElement => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_issued"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Operator => {
                            if r#operator.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            r#operator = Some(map_access.next_value()?);
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        Field::DeviceName => {
                            let some = r#device_name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceName"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DeviceNamePrimitiveElement => {
                            let some = r#device_name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_deviceName"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        Field::Height => {
                            let some = r#height.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::HeightPrimitiveElement => {
                            let some = r#height.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_height"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Width => {
                            let some = r#width.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::WidthPrimitiveElement => {
                            let some = r#width.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_width"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Frames => {
                            let some = r#frames.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("frames"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FramesPrimitiveElement => {
                            let some = r#frames.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_frames"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Duration => {
                            let some = r#duration.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DurationPrimitiveElement => {
                            let some = r#duration.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_duration"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Content => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            r#content = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Media {
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
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#type,
                    r#modality,
                    r#view,
                    r#subject,
                    r#encounter,
                    r#created,
                    r#issued,
                    r#operator,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#body_site,
                    r#device_name,
                    r#device,
                    r#height,
                    r#width,
                    r#frames,
                    r#duration,
                    r#content: r#content.ok_or(serde::de::Error::missing_field("content"))?,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
