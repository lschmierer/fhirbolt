// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
#[derive(Default, Debug, Clone)]
pub struct Meta {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The version specific identifier, as it appears in the version portion of the URL. This value changes when the resource is created, updated, or deleted."]
    pub r#version_id: Option<super::super::types::Id>,
    #[doc = "When the resource last changed - e.g. when the version changed."]
    pub r#last_updated: Option<super::super::types::Instant>,
    #[doc = "A uri that identifies the source system of the resource. This provides a minimal amount of [Provenance](provenance.html#) information that can be used to track or differentiate the source of information in the resource. The source may identify another FHIR server, document, message, database, etc."]
    pub r#source: Option<super::super::types::Uri>,
    #[doc = "A list of profiles (references to [StructureDefinition](structuredefinition.html#) resources) that this resource claims to conform to. The URL is a reference to [StructureDefinition.url](structuredefinition-definitions.html#StructureDefinition.url)."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "Security labels applied to this resource. These tags connect specific resources to the overall security policy and infrastructure."]
    pub r#security: Vec<Box<super::super::types::Coding>>,
    #[doc = "Tags applied to this resource. Tags are intended to be used to identify and relate resources to process and workflow, and applications are not required to consider the tags when interpreting the meaning of a resource."]
    pub r#tag: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for Meta {
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
            if _ctx.output_json {
                if let Some(some) = self.r#version_id.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("versionId", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_versionId", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#version_id.as_ref() {
                    state.serialize_entry("versionId", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_updated.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastUpdated", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastUpdated", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_updated.as_ref() {
                    state.serialize_entry("lastUpdated", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#source.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("source", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_source", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source.as_ref() {
                    state.serialize_entry("source", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#profile.is_empty() {
                    let values = self
                        .r#profile
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("profile", &values)?;
                    }
                    let requires_elements = self
                        .r#profile
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#profile
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_profile", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#profile.is_empty() {
                    state.serialize_entry("profile", &self.r#profile)?;
                }
            }
            if !self.r#security.is_empty() {
                state.serialize_entry("security", &self.r#security)?;
            }
            if !self.r#tag.is_empty() {
                state.serialize_entry("tag", &self.r#tag)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Meta {
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
            #[serde(rename = "versionId")]
            VersionId,
            #[serde(rename = "_versionId")]
            VersionIdPrimitiveElement,
            #[serde(rename = "lastUpdated")]
            LastUpdated,
            #[serde(rename = "_lastUpdated")]
            LastUpdatedPrimitiveElement,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "_source")]
            SourcePrimitiveElement,
            #[serde(rename = "profile")]
            Profile,
            #[serde(rename = "_profile")]
            ProfilePrimitiveElement,
            #[serde(rename = "security")]
            Security,
            #[serde(rename = "tag")]
            Tag,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Meta;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Meta")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Meta, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#version_id: Option<super::super::types::Id> = None;
                let mut r#last_updated: Option<super::super::types::Instant> = None;
                let mut r#source: Option<super::super::types::Uri> = None;
                let mut r#profile: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#security: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#tag: Option<Vec<Box<super::super::types::Coding>>> = None;
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
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::VersionId => {
                                if _ctx.from_json {
                                    let some = r#version_id.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("versionId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#version_id.is_some() {
                                        return Err(serde::de::Error::duplicate_field("versionId"));
                                    }
                                    r#version_id = Some(map_access.next_value()?);
                                }
                            }
                            Field::VersionIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#version_id.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_versionId",
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
                                        "versionId",
                                        &[
                                            "id",
                                            "extension",
                                            "versionId",
                                            "lastUpdated",
                                            "source",
                                            "profile",
                                            "security",
                                            "tag",
                                        ],
                                    ));
                                }
                            }
                            Field::LastUpdated => {
                                if _ctx.from_json {
                                    let some = r#last_updated.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastUpdated",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_updated.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastUpdated",
                                        ));
                                    }
                                    r#last_updated = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastUpdatedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#last_updated.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lastUpdated",
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
                                        "lastUpdated",
                                        &[
                                            "id",
                                            "extension",
                                            "versionId",
                                            "lastUpdated",
                                            "source",
                                            "profile",
                                            "security",
                                            "tag",
                                        ],
                                    ));
                                }
                            }
                            Field::Source => {
                                if _ctx.from_json {
                                    let some = r#source.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("source"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#source.is_some() {
                                        return Err(serde::de::Error::duplicate_field("source"));
                                    }
                                    r#source = Some(map_access.next_value()?);
                                }
                            }
                            Field::SourcePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#source.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_source"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "source",
                                        &[
                                            "id",
                                            "extension",
                                            "versionId",
                                            "lastUpdated",
                                            "source",
                                            "profile",
                                            "security",
                                            "tag",
                                        ],
                                    ));
                                }
                            }
                            Field::Profile => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#profile.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    if r#profile.is_some() {
                                        return Err(serde::de::Error::duplicate_field("profile"));
                                    }
                                    r#profile = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProfilePrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#profile.get_or_insert(
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
                                        return Err(serde::de::Error::duplicate_field("_profile"));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "profile",
                                        &[
                                            "id",
                                            "extension",
                                            "versionId",
                                            "lastUpdated",
                                            "source",
                                            "profile",
                                            "security",
                                            "tag",
                                        ],
                                    ));
                                }
                            }
                            Field::Security => {
                                if r#security.is_some() {
                                    return Err(serde::de::Error::duplicate_field("security"));
                                }
                                r#security = Some(map_access.next_value()?);
                            }
                            Field::Tag => {
                                if r#tag.is_some() {
                                    return Err(serde::de::Error::duplicate_field("tag"));
                                }
                                r#tag = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "versionId",
                                        "lastUpdated",
                                        "source",
                                        "profile",
                                        "security",
                                        "tag",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Meta {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#version_id,
                        r#last_updated,
                        r#source,
                        r#profile: r#profile.unwrap_or(vec![]),
                        r#security: r#security.unwrap_or(vec![]),
                        r#tag: r#tag.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
