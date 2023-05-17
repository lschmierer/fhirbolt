// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::resources::Library;
impl crate::Resource for Library {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for SerializationContext<&Library> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Library", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Library")?;
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("url", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("version", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#version.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("name", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#title.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("title", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_title", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#title.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("title", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#subtitle.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("subtitle", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_subtitle", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#subtitle.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subtitle", ctx))?;
        }
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#experimental.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("experimental", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_experimental", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#experimental.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("experimental", ctx))?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        {
            use fhirbolt_model::r4b::resources::LibrarySubject as _Enum;
            if let Some(some) = self.value.r#subject.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("subjectCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("subjectReference", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("subject is invalid")),
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("date", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_date", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("date", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#publisher.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("publisher", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_publisher", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#publisher.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if !self.value.r#use_context.is_empty() {
            self.with_context(&self.value.r#use_context, |ctx| {
                state.serialize_entry("useContext", ctx)
            })?;
        }
        if !self.value.r#jurisdiction.is_empty() {
            self.with_context(&self.value.r#jurisdiction, |ctx| {
                state.serialize_entry("jurisdiction", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#purpose.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("purpose", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_purpose", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#purpose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("purpose", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#usage.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("usage", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_usage", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#usage.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("usage", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#copyright.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("copyright", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_copyright", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#copyright.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("copyright", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#approval_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("approvalDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_approvalDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#approval_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("approvalDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#last_review_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("lastReviewDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastReviewDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#last_review_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("lastReviewDate", ctx))?;
        }
        if let Some(some) = self.value.r#effective_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("effectivePeriod", ctx))?;
        }
        if !self.value.r#topic.is_empty() {
            self.with_context(&self.value.r#topic, |ctx| {
                state.serialize_entry("topic", ctx)
            })?;
        }
        if !self.value.r#author.is_empty() {
            self.with_context(&self.value.r#author, |ctx| {
                state.serialize_entry("author", ctx)
            })?;
        }
        if !self.value.r#editor.is_empty() {
            self.with_context(&self.value.r#editor, |ctx| {
                state.serialize_entry("editor", ctx)
            })?;
        }
        if !self.value.r#reviewer.is_empty() {
            self.with_context(&self.value.r#reviewer, |ctx| {
                state.serialize_entry("reviewer", ctx)
            })?;
        }
        if !self.value.r#endorser.is_empty() {
            self.with_context(&self.value.r#endorser, |ctx| {
                state.serialize_entry("endorser", ctx)
            })?;
        }
        if !self.value.r#related_artifact.is_empty() {
            self.with_context(&self.value.r#related_artifact, |ctx| {
                state.serialize_entry("relatedArtifact", ctx)
            })?;
        }
        if !self.value.r#parameter.is_empty() {
            self.with_context(&self.value.r#parameter, |ctx| {
                state.serialize_entry("parameter", ctx)
            })?;
        }
        if !self.value.r#data_requirement.is_empty() {
            self.with_context(&self.value.r#data_requirement, |ctx| {
                state.serialize_entry("dataRequirement", ctx)
            })?;
        }
        if !self.value.r#content.is_empty() {
            self.with_context(&self.value.r#content, |ctx| {
                state.serialize_entry("content", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Library>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Library>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Library> {
    type Value = Library;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Library> {
    type Value = Library;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Library>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Library;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Library")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Library, V::Error>
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "subtitle")]
                    Subtitle,
                    #[serde(rename = "_subtitle")]
                    SubtitlePrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "experimental")]
                    Experimental,
                    #[serde(rename = "_experimental")]
                    ExperimentalPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subjectCodeableConcept")]
                    SubjectCodeableConcept,
                    #[serde(rename = "subjectReference")]
                    SubjectReference,
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
                    #[serde(rename = "usage")]
                    Usage,
                    #[serde(rename = "_usage")]
                    UsagePrimitiveElement,
                    #[serde(rename = "copyright")]
                    Copyright,
                    #[serde(rename = "_copyright")]
                    CopyrightPrimitiveElement,
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
                    #[serde(rename = "topic")]
                    Topic,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "editor")]
                    Editor,
                    #[serde(rename = "reviewer")]
                    Reviewer,
                    #[serde(rename = "endorser")]
                    Endorser,
                    #[serde(rename = "relatedArtifact")]
                    RelatedArtifact,
                    #[serde(rename = "parameter")]
                    Parameter,
                    #[serde(rename = "dataRequirement")]
                    DataRequirement,
                    #[serde(rename = "content")]
                    Content,
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
                            "name",
                            "title",
                            "subtitle",
                            "status",
                            "experimental",
                            "type",
                            "subjectCodeableConcept",
                            "subjectReference",
                            "date",
                            "publisher",
                            "contact",
                            "description",
                            "useContext",
                            "jurisdiction",
                            "purpose",
                            "usage",
                            "copyright",
                            "approvalDate",
                            "lastReviewDate",
                            "effectivePeriod",
                            "topic",
                            "author",
                            "editor",
                            "reviewer",
                            "endorser",
                            "relatedArtifact",
                            "parameter",
                            "dataRequirement",
                            "content",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r4b::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4b::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#url: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4b::types::Identifier>> = None;
                let mut r#version: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#subtitle: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#experimental: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#subject: Option<fhirbolt_model::r4b::resources::LibrarySubject> = None;
                let mut r#date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#publisher: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r4b::types::ContactDetail>> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#use_context: Option<Vec<fhirbolt_model::r4b::types::UsageContext>> = None;
                let mut r#jurisdiction: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#purpose: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#usage: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#copyright: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#approval_date: Option<fhirbolt_model::r4b::types::Date> = None;
                let mut r#last_review_date: Option<fhirbolt_model::r4b::types::Date> = None;
                let mut r#effective_period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#topic: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#author: Option<Vec<fhirbolt_model::r4b::types::ContactDetail>> = None;
                let mut r#editor: Option<Vec<fhirbolt_model::r4b::types::ContactDetail>> = None;
                let mut r#reviewer: Option<Vec<fhirbolt_model::r4b::types::ContactDetail>> = None;
                let mut r#endorser: Option<Vec<fhirbolt_model::r4b::types::ContactDetail>> = None;
                let mut r#related_artifact: Option<
                    Vec<fhirbolt_model::r4b::types::RelatedArtifact>,
                > = None;
                let mut r#parameter: Option<Vec<fhirbolt_model::r4b::types::ParameterDefinition>> =
                    None;
                let mut r#data_requirement: Option<
                    Vec<fhirbolt_model::r4b::types::DataRequirement>,
                > = None;
                let mut r#content: Option<Vec<fhirbolt_model::r4b::types::Attachment>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Library" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Library",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
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
                                    Box<fhirbolt_model::r4b::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
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
                                Box<fhirbolt_model::r4b::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
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
                                Box<fhirbolt_model::r4b::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Url => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#url = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                    Vec<fhirbolt_model::r4b::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#version = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#title = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::Subtitle => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#subtitle.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtitle"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#subtitle.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtitle"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#subtitle = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SubtitlePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#subtitle.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_subtitle"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("subtitle");
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#experimental.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#experimental = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("experimental");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SubjectCodeableConcept => {
                            use fhirbolt_model::r4b::resources::LibrarySubject as _Enum;
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#subject = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::SubjectReference => {
                            use fhirbolt_model::r4b::resources::LibrarySubject as _Enum;
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Date => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::DateTime,
                                > = self.0.transmute();
                                r#date = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#publisher = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                    Vec<fhirbolt_model::r4b::types::ContactDetail>,
                                > = self.0.transmute();
                                r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                    Vec<fhirbolt_model::r4b::types::UsageContext>,
                                > = self.0.transmute();
                                r#use_context = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::UsageContext,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Jurisdiction => {
                            if self.0.from == crate::context::Format::Json {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#jurisdiction = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#jurisdiction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Purpose => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#purpose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#purpose = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Usage => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#usage.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usage"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#usage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usage"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#usage = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UsagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#usage.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_usage"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("usage");
                            }
                        }
                        Field::Copyright => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#copyright.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#copyright = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyright");
                            }
                        }
                        Field::ApprovalDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#approval_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Date,
                                > = self.0.transmute();
                                r#approval_date = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#last_review_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Date,
                                > = self.0.transmute();
                                r#last_review_date =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#effective_period = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Topic => {
                            if self.0.from == crate::context::Format::Json {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field("topic"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#topic = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#topic.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Author => {
                            if self.0.from == crate::context::Format::Json {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ContactDetail>,
                                > = self.0.transmute();
                                r#author = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#author.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Editor => {
                            if self.0.from == crate::context::Format::Json {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ContactDetail>,
                                > = self.0.transmute();
                                r#editor = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#editor.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Reviewer => {
                            if self.0.from == crate::context::Format::Json {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ContactDetail>,
                                > = self.0.transmute();
                                r#reviewer = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#reviewer.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Endorser => {
                            if self.0.from == crate::context::Format::Json {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ContactDetail>,
                                > = self.0.transmute();
                                r#endorser = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#endorser.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RelatedArtifact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#related_artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedArtifact",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::RelatedArtifact>,
                                > = self.0.transmute();
                                r#related_artifact =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#related_artifact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::RelatedArtifact,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Parameter => {
                            if self.0.from == crate::context::Format::Json {
                                if r#parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parameter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ParameterDefinition>,
                                > = self.0.transmute();
                                r#parameter = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#parameter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ParameterDefinition,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DataRequirement => {
                            if self.0.from == crate::context::Format::Json {
                                if r#data_requirement.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dataRequirement",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::DataRequirement>,
                                > = self.0.transmute();
                                r#data_requirement =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#data_requirement.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::DataRequirement,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Content => {
                            if self.0.from == crate::context::Format::Json {
                                if r#content.is_some() {
                                    return Err(serde::de::Error::duplicate_field("content"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Attachment>,
                                > = self.0.transmute();
                                r#content = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#content.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Attachment,
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
                Ok(Library {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url,
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#version,
                    r#name,
                    r#title,
                    r#subtitle,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#experimental,
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#subject,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#usage,
                    r#copyright,
                    r#approval_date,
                    r#last_review_date,
                    r#effective_period,
                    r#topic: r#topic.unwrap_or(vec![]),
                    r#author: r#author.unwrap_or(vec![]),
                    r#editor: r#editor.unwrap_or(vec![]),
                    r#reviewer: r#reviewer.unwrap_or(vec![]),
                    r#endorser: r#endorser.unwrap_or(vec![]),
                    r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                    r#parameter: r#parameter.unwrap_or(vec![]),
                    r#data_requirement: r#data_requirement.unwrap_or(vec![]),
                    r#content: r#content.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Library>> {
    type Value = Box<Library>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Library>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Library>> {
    type Value = Vec<Library>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Library>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Library>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Library> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
