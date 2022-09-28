// Generated on 2022-09-28 by fhirbolt-codegen v0.1.0
#[doc = "The date and time(s) at which the media was collected."]
#[derive(Debug, Clone)]
pub enum MediaCreated {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl crate::model::ResourceOrElement for MediaCreated {}
impl Default for MediaCreated {
    fn default() -> MediaCreated {
        MediaCreated::Invalid
    }
}
#[doc = "A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference."]
#[derive(Default, Debug, Clone)]
pub struct Media {
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
    #[doc = "Identifiers associated with the image - these may include identifiers for the image itself, identifiers for the context of its collection (e.g. series ids) and context ids such as accession numbers or other workflow identifiers."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A procedure that is fulfilled in whole or in part by the creation of this media."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "The current state of the {{title}}."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies whether the media is an image, video or audio recording or some other media category."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of the type of the media - usually, how it was acquired (what type of device). If images sourced from a DICOM system, are wrapped in a Media resource, then this is the modality."]
    pub r#modality: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name of the imaging view e.g. Lateral or Antero-posterior (AP)."]
    pub r#view: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Who/What this Media is a record of."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter that establishes the context for this media."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and time(s) at which the media was collected."]
    pub r#created: Option<MediaCreated>,
    #[doc = "The date and time this version of the media was made available to providers, typically after having been reviewed."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "The person who administered the collection of the image."]
    pub r#operator: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes why the event occurred in coded or textual form."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the site on the subject's body where the observation was made (i.e. the target site)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name of the device / manufacturer of the device  that was used to make the recording."]
    pub r#device_name: Option<super::super::types::String>,
    #[doc = "The device used to collect the media."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Height of the image in pixels (photo/video)."]
    pub r#height: Option<super::super::types::PositiveInt>,
    #[doc = "Width of the image in pixels (photo/video)."]
    pub r#width: Option<super::super::types::PositiveInt>,
    #[doc = "The number of frames in a photo. This is used with a multi-page fax, or an imaging acquisition context that takes multiple slices in a single image, or an animated gif. If there is more than one frame, this SHALL have a value in order to alert interface software that a multi-frame capable rendering widget is required."]
    pub r#frames: Option<super::super::types::PositiveInt>,
    #[doc = "The duration of the recording in seconds - for audio and video."]
    pub r#duration: Option<super::super::types::Decimal>,
    #[doc = "The actual content of the media - inline or by direct reference to the media source file."]
    pub r#content: Box<super::super::types::Attachment>,
    #[doc = "Comments made about the media by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl crate::model::ResourceOrElement for Media {}
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
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("createdDateTime", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("issued", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("deviceName", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("height", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("width", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("frames", &some)?;
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
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("duration", &some)?;
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
            Unknown(String),
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                crate::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Media" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Media",
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
                                let r#enum = r#created
                                    .get_or_insert(MediaCreated::DateTime(Default::default()));
                                if let MediaCreated::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "createdDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("created[x]"));
                                }
                            }
                            Field::CreatedDateTimePrimitiveElement => {
                                let r#enum = r#created
                                    .get_or_insert(MediaCreated::DateTime(Default::default()));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
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
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "type",
                                            "modality",
                                            "view",
                                            "subject",
                                            "encounter",
                                            "createdDateTime",
                                            "createdPeriod",
                                            "issued",
                                            "operator",
                                            "reasonCode",
                                            "bodySite",
                                            "deviceName",
                                            "device",
                                            "height",
                                            "width",
                                            "frames",
                                            "duration",
                                            "content",
                                            "note",
                                        ],
                                    ));
                                }
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
                        r#status: if config.mode == crate::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
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
                        r#content: if config.mode == crate::DeserializationMode::Lax {
                            r#content.unwrap_or(Default::default())
                        } else {
                            r#content.ok_or(serde::de::Error::missing_field("content"))?
                        },
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
