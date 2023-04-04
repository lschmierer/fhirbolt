// Generated on 2023-04-04 by fhirbolt-codegen v0.1.0
#[doc = "Indicates who or what performed the series and how they were involved."]
#[derive(Default, Debug, Clone)]
pub struct ImagingStudySeriesPerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Distinguishes the type of involvement of the performer in the series."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed the series."]
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ImagingStudySeriesPerformer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if let Some(some) = self.r#function.as_ref() {
                state.serialize_entry("function", some)?;
            }
            state.serialize_entry("actor", &self.r#actor)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeriesPerformer {
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
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "actor")]
            Actor,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeriesPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeriesPerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ImagingStudySeriesPerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Function => {
                                if r#function.is_some() {
                                    return Err(serde::de::Error::duplicate_field("function"));
                                }
                                r#function = Some(map_access.next_value()?);
                            }
                            Field::Actor => {
                                if r#actor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actor"));
                                }
                                r#actor = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "function", "actor"],
                                ));
                            },
                        }
                    }
                    Ok(ImagingStudySeriesPerformer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#function,
                        r#actor: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#actor.unwrap_or(Default::default())
                        } else {
                            r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
#[derive(Default, Debug, Clone)]
pub struct ImagingStudySeriesInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The DICOM SOP Instance UID for this image or other DICOM content."]
    pub r#uid: super::super::types::Id,
    #[doc = "DICOM instance  type."]
    pub r#sop_class: Box<super::super::types::Coding>,
    #[doc = "The number of instance in the series."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The description of the instance."]
    pub r#title: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ImagingStudySeriesInstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#uid.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("uid", &some)?;
                }
                if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#uid.id.as_ref(),
                        extension: &self.r#uid.extension,
                    };
                    state.serialize_entry("_uid", &primitive_element)?;
                }
            } else {
                state.serialize_entry("uid", &self.r#uid)?;
            }
            state.serialize_entry("sopClass", &self.r#sop_class)?;
            if _ctx.output_json {
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeriesInstance {
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
            #[serde(rename = "uid")]
            Uid,
            #[serde(rename = "_uid")]
            UidPrimitiveElement,
            #[serde(rename = "sopClass")]
            SopClass,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "_number")]
            NumberPrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeriesInstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeriesInstance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudySeriesInstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#uid: Option<super::super::types::Id> = None;
                let mut r#sop_class: Option<Box<super::super::types::Coding>> = None;
                let mut r#number: Option<super::super::types::UnsignedInt> = None;
                let mut r#title: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Uid => {
                                if _ctx.from_json {
                                    let some = r#uid.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#uid.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uid"));
                                    }
                                    r#uid = Some(map_access.next_value()?);
                                }
                            }
                            Field::UidPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#uid.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_uid"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "uid",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "sopClass",
                                            "number",
                                            "title",
                                        ],
                                    ));
                                }
                            }
                            Field::SopClass => {
                                if r#sop_class.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sopClass"));
                                }
                                r#sop_class = Some(map_access.next_value()?);
                            }
                            Field::Number => {
                                if _ctx.from_json {
                                    let some = r#number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    r#number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_number"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "number",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "sopClass",
                                            "number",
                                            "title",
                                        ],
                                    ));
                                }
                            }
                            Field::Title => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#title.is_some() {
                                        return Err(serde::de::Error::duplicate_field("title"));
                                    }
                                    r#title = Some(map_access.next_value()?);
                                }
                            }
                            Field::TitlePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#title.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_title"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "title",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "sopClass",
                                            "number",
                                            "title",
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
                                        "extension",
                                        "modifierExtension",
                                        "uid",
                                        "sopClass",
                                        "number",
                                        "title",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ImagingStudySeriesInstance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#uid: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#uid.unwrap_or(Default::default())
                        } else {
                            r#uid.ok_or(serde::de::Error::missing_field("uid"))?
                        },
                        r#sop_class: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#sop_class.unwrap_or(Default::default())
                        } else {
                            r#sop_class.ok_or(serde::de::Error::missing_field("sopClass"))?
                        },
                        r#number,
                        r#title,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Each study has one or more series of images or other content."]
#[derive(Default, Debug, Clone)]
pub struct ImagingStudySeries {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The DICOM Series Instance UID for the series."]
    pub r#uid: super::super::types::Id,
    #[doc = "The numeric identifier of this series in the study."]
    pub r#number: Option<super::super::types::UnsignedInt>,
    #[doc = "The modality of this series sequence."]
    pub r#modality: Box<super::super::types::Coding>,
    #[doc = "A description of the series."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Number of SOP Instances in the Study. The value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for this series. See implementation notes for information about using DICOM endpoints. A series-level endpoint, if present, has precedence over a study-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    #[doc = "The anatomic structures examined. See DICOM Part 16 Annex L (<http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html>) for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of body part imaged; if so, it shall be consistent with any content of ImagingStudy.series.laterality."]
    pub r#body_site: Option<Box<super::super::types::Coding>>,
    #[doc = "The laterality of the (possibly paired) anatomic structures examined. E.g., the left knee, both lungs, or unpaired abdomen. If present, shall be consistent with any laterality information indicated in ImagingStudy.series.bodySite."]
    pub r#laterality: Option<Box<super::super::types::Coding>>,
    #[doc = "The specimen imaged, e.g., for whole slide imaging of a biopsy."]
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    #[doc = "The date and time the series was started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "Indicates who or what performed the series and how they were involved."]
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    #[doc = "A single SOP instance within the series, e.g. an image, or presentation state."]
    pub r#instance: Vec<ImagingStudySeriesInstance>,
}
impl serde::ser::Serialize for ImagingStudySeries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            if _ctx.output_json {
                if let Some(some) = self.r#uid.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("uid", &some)?;
                }
                if self.r#uid.id.is_some() || !self.r#uid.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#uid.id.as_ref(),
                        extension: &self.r#uid.extension,
                    };
                    state.serialize_entry("_uid", &primitive_element)?;
                }
            } else {
                state.serialize_entry("uid", &self.r#uid)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("number", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_number", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number.as_ref() {
                    state.serialize_entry("number", some)?;
                }
            }
            state.serialize_entry("modality", &self.r#modality)?;
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfInstances", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfInstances", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    state.serialize_entry("numberOfInstances", some)?;
                }
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            if let Some(some) = self.r#body_site.as_ref() {
                state.serialize_entry("bodySite", some)?;
            }
            if let Some(some) = self.r#laterality.as_ref() {
                state.serialize_entry("laterality", some)?;
            }
            if !self.r#specimen.is_empty() {
                state.serialize_entry("specimen", &self.r#specimen)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#started.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("started", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_started", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#started.as_ref() {
                    state.serialize_entry("started", some)?;
                }
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if !self.r#instance.is_empty() {
                state.serialize_entry("instance", &self.r#instance)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudySeries {
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
            #[serde(rename = "uid")]
            Uid,
            #[serde(rename = "_uid")]
            UidPrimitiveElement,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "_number")]
            NumberPrimitiveElement,
            #[serde(rename = "modality")]
            Modality,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "numberOfInstances")]
            NumberOfInstances,
            #[serde(rename = "_numberOfInstances")]
            NumberOfInstancesPrimitiveElement,
            #[serde(rename = "endpoint")]
            Endpoint,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "laterality")]
            Laterality,
            #[serde(rename = "specimen")]
            Specimen,
            #[serde(rename = "started")]
            Started,
            #[serde(rename = "_started")]
            StartedPrimitiveElement,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "instance")]
            Instance,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudySeries;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudySeries")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudySeries, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#uid: Option<super::super::types::Id> = None;
                let mut r#number: Option<super::super::types::UnsignedInt> = None;
                let mut r#modality: Option<Box<super::super::types::Coding>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#number_of_instances: Option<super::super::types::UnsignedInt> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Box<super::super::types::Coding>> = None;
                let mut r#laterality: Option<Box<super::super::types::Coding>> = None;
                let mut r#specimen: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#started: Option<super::super::types::DateTime> = None;
                let mut r#performer: Option<Vec<ImagingStudySeriesPerformer>> = None;
                let mut r#instance: Option<Vec<ImagingStudySeriesInstance>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Uid => {
                                if _ctx.from_json {
                                    let some = r#uid.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#uid.is_some() {
                                        return Err(serde::de::Error::duplicate_field("uid"));
                                    }
                                    r#uid = Some(map_access.next_value()?);
                                }
                            }
                            Field::UidPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#uid.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_uid"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "uid",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "number",
                                            "modality",
                                            "description",
                                            "numberOfInstances",
                                            "endpoint",
                                            "bodySite",
                                            "laterality",
                                            "specimen",
                                            "started",
                                            "performer",
                                            "instance",
                                        ],
                                    ));
                                }
                            }
                            Field::Number => {
                                if _ctx.from_json {
                                    let some = r#number.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number.is_some() {
                                        return Err(serde::de::Error::duplicate_field("number"));
                                    }
                                    r#number = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#number.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_number"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "number",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "number",
                                            "modality",
                                            "description",
                                            "numberOfInstances",
                                            "endpoint",
                                            "bodySite",
                                            "laterality",
                                            "specimen",
                                            "started",
                                            "performer",
                                            "instance",
                                        ],
                                    ));
                                }
                            }
                            Field::Modality => {
                                if r#modality.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modality"));
                                }
                                r#modality = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
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
                                        "description",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "number",
                                            "modality",
                                            "description",
                                            "numberOfInstances",
                                            "endpoint",
                                            "bodySite",
                                            "laterality",
                                            "specimen",
                                            "started",
                                            "performer",
                                            "instance",
                                        ],
                                    ));
                                }
                            }
                            Field::NumberOfInstances => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_instances.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfInstances",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_instances.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfInstances",
                                        ));
                                    }
                                    r#number_of_instances = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfInstancesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_instances.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfInstances",
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
                                        "numberOfInstances",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "number",
                                            "modality",
                                            "description",
                                            "numberOfInstances",
                                            "endpoint",
                                            "bodySite",
                                            "laterality",
                                            "specimen",
                                            "started",
                                            "performer",
                                            "instance",
                                        ],
                                    ));
                                }
                            }
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#endpoint.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::Laterality => {
                                if r#laterality.is_some() {
                                    return Err(serde::de::Error::duplicate_field("laterality"));
                                }
                                r#laterality = Some(map_access.next_value()?);
                            }
                            Field::Specimen => {
                                if _ctx.from_json {
                                    if r#specimen.is_some() {
                                        return Err(serde::de::Error::duplicate_field("specimen"));
                                    }
                                    r#specimen = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#specimen.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Started => {
                                if _ctx.from_json {
                                    let some = r#started.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("started"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#started.is_some() {
                                        return Err(serde::de::Error::duplicate_field("started"));
                                    }
                                    r#started = Some(map_access.next_value()?);
                                }
                            }
                            Field::StartedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#started.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_started"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "started",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "uid",
                                            "number",
                                            "modality",
                                            "description",
                                            "numberOfInstances",
                                            "endpoint",
                                            "bodySite",
                                            "laterality",
                                            "specimen",
                                            "started",
                                            "performer",
                                            "instance",
                                        ],
                                    ));
                                }
                            }
                            Field::Performer => {
                                if _ctx.from_json {
                                    if r#performer.is_some() {
                                        return Err(serde::de::Error::duplicate_field("performer"));
                                    }
                                    r#performer = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#performer.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Instance => {
                                if _ctx.from_json {
                                    if r#instance.is_some() {
                                        return Err(serde::de::Error::duplicate_field("instance"));
                                    }
                                    r#instance = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#instance.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "uid",
                                        "number",
                                        "modality",
                                        "description",
                                        "numberOfInstances",
                                        "endpoint",
                                        "bodySite",
                                        "laterality",
                                        "specimen",
                                        "started",
                                        "performer",
                                        "instance",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ImagingStudySeries {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#uid: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#uid.unwrap_or(Default::default())
                        } else {
                            r#uid.ok_or(serde::de::Error::missing_field("uid"))?
                        },
                        r#number,
                        r#modality: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#modality.unwrap_or(Default::default())
                        } else {
                            r#modality.ok_or(serde::de::Error::missing_field("modality"))?
                        },
                        r#description,
                        r#number_of_instances,
                        r#endpoint: r#endpoint.unwrap_or(vec![]),
                        r#body_site,
                        r#laterality,
                        r#specimen: r#specimen.unwrap_or(vec![]),
                        r#started,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#instance: r#instance.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities."]
#[derive(Default, Debug, Clone)]
pub struct ImagingStudy {
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
    #[doc = "Identifiers for the ImagingStudy such as DICOM Study Instance UID, and Accession Number."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the ImagingStudy."]
    pub r#status: super::super::types::Code,
    #[doc = "A list of all the series.modality values that are actual acquisition modalities, i.e. those in the DICOM Context Group 29 (value set OID 1.2.840.10008.6.1.19)."]
    pub r#modality: Vec<Box<super::super::types::Coding>>,
    #[doc = "The subject, typically a patient, of the imaging study."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The healthcare event (e.g. a patient and healthcare provider interaction) during which this ImagingStudy is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Date and time the study started."]
    pub r#started: Option<super::super::types::DateTime>,
    #[doc = "A list of the diagnostic requests that resulted in this imaging study being performed."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "The requesting/referring physician."]
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    #[doc = "Who read the study and interpreted the images or other content."]
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    #[doc = "The network service providing access (e.g., query, view, or retrieval) for the study. See implementation notes for information about using DICOM endpoints. A study-level endpoint applies to each series in the study, unless overridden by a series-level endpoint with the same Endpoint.connectionType."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    #[doc = "Number of Series in the Study. This value given may be larger than the number of series elements this Resource contains due to resource availability, security, or other factors. This element should be present if any series elements are present."]
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    #[doc = "Number of SOP Instances in Study. This value given may be larger than the number of instance elements this resource contains due to resource availability, security, or other factors. This element should be present if any instance elements are present."]
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    #[doc = "The procedure which this ImagingStudy was part of."]
    pub r#procedure_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "The code for the performed procedure type."]
    pub r#procedure_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The principal physical location where the ImagingStudy was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Description of clinical condition indicating why the ImagingStudy was requested."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource whose existence justifies this Study."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Per the recommended DICOM mapping, this element is derived from the Study Description attribute (0008,1030). Observations or findings about the imaging study should be recorded in another resource, e.g. Observation, and not in this element."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The Imaging Manager description of the study. Institution-generated description or classification of the Study (component) performed."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Each study has one or more series of images or other content."]
    pub r#series: Vec<ImagingStudySeries>,
}
impl crate::AnyResource for ImagingStudy {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4B;
}
impl serde::ser::Serialize for ImagingStudy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "ImagingStudy")?;
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
            if !self.r#modality.is_empty() {
                state.serialize_entry("modality", &self.r#modality)?;
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#started.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("started", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_started", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#started.as_ref() {
                    state.serialize_entry("started", some)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if let Some(some) = self.r#referrer.as_ref() {
                state.serialize_entry("referrer", some)?;
            }
            if !self.r#interpreter.is_empty() {
                state.serialize_entry("interpreter", &self.r#interpreter)?;
            }
            if !self.r#endpoint.is_empty() {
                state.serialize_entry("endpoint", &self.r#endpoint)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_series.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfSeries", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfSeries", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_series.as_ref() {
                    state.serialize_entry("numberOfSeries", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("numberOfInstances", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_numberOfInstances", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#number_of_instances.as_ref() {
                    state.serialize_entry("numberOfInstances", some)?;
                }
            }
            if let Some(some) = self.r#procedure_reference.as_ref() {
                state.serialize_entry("procedureReference", some)?;
            }
            if !self.r#procedure_code.is_empty() {
                state.serialize_entry("procedureCode", &self.r#procedure_code)?;
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if !self.r#series.is_empty() {
                state.serialize_entry("series", &self.r#series)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ImagingStudy {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "modality")]
            Modality,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "started")]
            Started,
            #[serde(rename = "_started")]
            StartedPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "referrer")]
            Referrer,
            #[serde(rename = "interpreter")]
            Interpreter,
            #[serde(rename = "endpoint")]
            Endpoint,
            #[serde(rename = "numberOfSeries")]
            NumberOfSeries,
            #[serde(rename = "_numberOfSeries")]
            NumberOfSeriesPrimitiveElement,
            #[serde(rename = "numberOfInstances")]
            NumberOfInstances,
            #[serde(rename = "_numberOfInstances")]
            NumberOfInstancesPrimitiveElement,
            #[serde(rename = "procedureReference")]
            ProcedureReference,
            #[serde(rename = "procedureCode")]
            ProcedureCode,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "series")]
            Series,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ImagingStudy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ImagingStudy")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ImagingStudy, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#modality: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#started: Option<super::super::types::DateTime> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#referrer: Option<Box<super::super::types::Reference>> = None;
                let mut r#interpreter: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#number_of_series: Option<super::super::types::UnsignedInt> = None;
                let mut r#number_of_instances: Option<super::super::types::UnsignedInt> = None;
                let mut r#procedure_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#procedure_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#series: Option<Vec<ImagingStudySeries>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "ImagingStudy" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"ImagingStudy",
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
                                            "status",
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
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
                                            "status",
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
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
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Identifier => {
                                if _ctx.from_json {
                                    if r#identifier.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identifier",
                                        ));
                                    }
                                    r#identifier = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#identifier.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                            "status",
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
                                        ],
                                    ));
                                }
                            }
                            Field::Modality => {
                                if _ctx.from_json {
                                    if r#modality.is_some() {
                                        return Err(serde::de::Error::duplicate_field("modality"));
                                    }
                                    r#modality = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#modality.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                            Field::Started => {
                                if _ctx.from_json {
                                    let some = r#started.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("started"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#started.is_some() {
                                        return Err(serde::de::Error::duplicate_field("started"));
                                    }
                                    r#started = Some(map_access.next_value()?);
                                }
                            }
                            Field::StartedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#started.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_started"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "started",
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
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
                                        ],
                                    ));
                                }
                            }
                            Field::BasedOn => {
                                if _ctx.from_json {
                                    if r#based_on.is_some() {
                                        return Err(serde::de::Error::duplicate_field("basedOn"));
                                    }
                                    r#based_on = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#based_on.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Referrer => {
                                if r#referrer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("referrer"));
                                }
                                r#referrer = Some(map_access.next_value()?);
                            }
                            Field::Interpreter => {
                                if _ctx.from_json {
                                    if r#interpreter.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "interpreter",
                                        ));
                                    }
                                    r#interpreter = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#interpreter.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Endpoint => {
                                if _ctx.from_json {
                                    if r#endpoint.is_some() {
                                        return Err(serde::de::Error::duplicate_field("endpoint"));
                                    }
                                    r#endpoint = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#endpoint.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfSeries => {
                                if _ctx.from_json {
                                    let some = r#number_of_series.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfSeries",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_series.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfSeries",
                                        ));
                                    }
                                    r#number_of_series = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfSeriesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#number_of_series.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfSeries",
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
                                        "numberOfSeries",
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
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
                                        ],
                                    ));
                                }
                            }
                            Field::NumberOfInstances => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_instances.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfInstances",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#number_of_instances.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "numberOfInstances",
                                        ));
                                    }
                                    r#number_of_instances = Some(map_access.next_value()?);
                                }
                            }
                            Field::NumberOfInstancesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#number_of_instances.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_numberOfInstances",
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
                                        "numberOfInstances",
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
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
                                        ],
                                    ));
                                }
                            }
                            Field::ProcedureReference => {
                                if r#procedure_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "procedureReference",
                                    ));
                                }
                                r#procedure_reference = Some(map_access.next_value()?);
                            }
                            Field::ProcedureCode => {
                                if _ctx.from_json {
                                    if r#procedure_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "procedureCode",
                                        ));
                                    }
                                    r#procedure_code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#procedure_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if _ctx.from_json {
                                    if r#reason_code.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonCode",
                                        ));
                                    }
                                    r#reason_code = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_code.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ReasonReference => {
                                if _ctx.from_json {
                                    if r#reason_reference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "reasonReference",
                                        ));
                                    }
                                    r#reason_reference = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#reason_reference.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Note => {
                                if _ctx.from_json {
                                    if r#note.is_some() {
                                        return Err(serde::de::Error::duplicate_field("note"));
                                    }
                                    r#note = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#note.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Description => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#description.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "description",
                                        ));
                                    }
                                    r#description = Some(map_access.next_value()?);
                                }
                            }
                            Field::DescriptionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#description.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_description",
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
                                        "description",
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
                                            "modality",
                                            "subject",
                                            "encounter",
                                            "started",
                                            "basedOn",
                                            "referrer",
                                            "interpreter",
                                            "endpoint",
                                            "numberOfSeries",
                                            "numberOfInstances",
                                            "procedureReference",
                                            "procedureCode",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "note",
                                            "description",
                                            "series",
                                        ],
                                    ));
                                }
                            }
                            Field::Series => {
                                if _ctx.from_json {
                                    if r#series.is_some() {
                                        return Err(serde::de::Error::duplicate_field("series"));
                                    }
                                    r#series = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#series.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
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
                                        "status",
                                        "modality",
                                        "subject",
                                        "encounter",
                                        "started",
                                        "basedOn",
                                        "referrer",
                                        "interpreter",
                                        "endpoint",
                                        "numberOfSeries",
                                        "numberOfInstances",
                                        "procedureReference",
                                        "procedureCode",
                                        "location",
                                        "reasonCode",
                                        "reasonReference",
                                        "note",
                                        "description",
                                        "series",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ImagingStudy {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#modality: r#modality.unwrap_or(vec![]),
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#started,
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#referrer,
                        r#interpreter: r#interpreter.unwrap_or(vec![]),
                        r#endpoint: r#endpoint.unwrap_or(vec![]),
                        r#number_of_series,
                        r#number_of_instances,
                        r#procedure_reference,
                        r#procedure_code: r#procedure_code.unwrap_or(vec![]),
                        r#location,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#description,
                        r#series: r#series.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
