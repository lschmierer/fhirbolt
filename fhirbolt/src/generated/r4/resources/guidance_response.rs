// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum GuidanceResponseModule {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for GuidanceResponseModule {
    fn default() -> GuidanceResponseModule {
        GuidanceResponseModule::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct GuidanceResponse {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#request_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#module: GuidanceResponseModule,
    pub r#status: super::super::types::Code,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#occurrence_date_time: Option<super::super::types::DateTime>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#evaluation_message: Vec<Box<super::super::types::Reference>>,
    pub r#output_parameters: Option<Box<super::super::types::Reference>>,
    pub r#result: Option<Box<super::super::types::Reference>>,
    pub r#data_requirement: Vec<Box<super::super::types::DataRequirement>>,
}
impl serde::ser::Serialize for GuidanceResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "GuidanceResponse")?;
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
        if let Some(some) = self.r#request_identifier.as_ref() {
            state.serialize_entry("requestIdentifier", some)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        match self.r#module {
            GuidanceResponseModule::Uri(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("moduleUri", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_moduleUri", &primitive_element)?;
                }
            }
            GuidanceResponseModule::Canonical(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("moduleCanonical", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_moduleCanonical", &primitive_element)?;
                }
            }
            GuidanceResponseModule::CodeableConcept(ref value) => {
                state.serialize_entry("moduleCodeableConcept", value)?;
            }
            GuidanceResponseModule::Invalid => {
                return Err(serde::ser::Error::custom("module is a required field"))
            }
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
        if let Some(some) = self.r#subject.as_ref() {
            state.serialize_entry("subject", some)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#occurrence_date_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("occurrenceDateTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
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
        if !self.r#evaluation_message.is_empty() {
            state.serialize_entry("evaluationMessage", &self.r#evaluation_message)?;
        }
        if let Some(some) = self.r#output_parameters.as_ref() {
            state.serialize_entry("outputParameters", some)?;
        }
        if let Some(some) = self.r#result.as_ref() {
            state.serialize_entry("result", some)?;
        }
        if !self.r#data_requirement.is_empty() {
            state.serialize_entry("dataRequirement", &self.r#data_requirement)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for GuidanceResponse {
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
            #[serde(rename = "requestIdentifier")]
            RequestIdentifier,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "moduleUri")]
            ModuleUri,
            #[serde(rename = "_moduleUri")]
            ModuleUriPrimitiveElement,
            #[serde(rename = "moduleCanonical")]
            ModuleCanonical,
            #[serde(rename = "_moduleCanonical")]
            ModuleCanonicalPrimitiveElement,
            #[serde(rename = "moduleCodeableConcept")]
            ModuleCodeableConcept,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "evaluationMessage")]
            EvaluationMessage,
            #[serde(rename = "outputParameters")]
            OutputParameters,
            #[serde(rename = "result")]
            Result,
            #[serde(rename = "dataRequirement")]
            DataRequirement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GuidanceResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GuidanceResponse")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GuidanceResponse, V::Error>
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
                let mut r#request_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#module: Option<GuidanceResponseModule> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence_date_time: Option<super::super::types::DateTime> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#evaluation_message: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#output_parameters: Option<Box<super::super::types::Reference>> = None;
                let mut r#result: Option<Box<super::super::types::Reference>> = None;
                let mut r#data_requirement: Option<Vec<Box<super::super::types::DataRequirement>>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "GuidanceResponse" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"GuidanceResponse",
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
                        Field::RequestIdentifier => {
                            if r#request_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestIdentifier"));
                            }
                            r#request_identifier = Some(map_access.next_value()?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::ModuleUri => {
                            let r#enum = r#module
                                .get_or_insert(GuidanceResponseModule::Uri(Default::default()));
                            if let GuidanceResponseModule::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("moduleUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("module[x]"));
                            }
                        }
                        Field::ModuleUriPrimitiveElement => {
                            let r#enum = r#module
                                .get_or_insert(GuidanceResponseModule::Uri(Default::default()));
                            if let GuidanceResponseModule::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_moduleUri"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_module[x]"));
                            }
                        }
                        Field::ModuleCanonical => {
                            let r#enum = r#module.get_or_insert(GuidanceResponseModule::Canonical(
                                Default::default(),
                            ));
                            if let GuidanceResponseModule::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "moduleCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("module[x]"));
                            }
                        }
                        Field::ModuleCanonicalPrimitiveElement => {
                            let r#enum = r#module.get_or_insert(GuidanceResponseModule::Canonical(
                                Default::default(),
                            ));
                            if let GuidanceResponseModule::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_moduleCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_module[x]"));
                            }
                        }
                        Field::ModuleCodeableConcept => {
                            if r#module.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "moduleCodeableConcept",
                                ));
                            }
                            r#module = Some(GuidanceResponseModule::CodeableConcept(
                                map_access.next_value()?,
                            ));
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
                        Field::OccurrenceDateTime => {
                            let some = r#occurrence_date_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "occurrenceDateTime",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OccurrenceDateTimePrimitiveElement => {
                            let some = r#occurrence_date_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_occurrenceDateTime",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        Field::ReasonCode => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        Field::ReasonReference => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::EvaluationMessage => {
                            if r#evaluation_message.is_some() {
                                return Err(serde::de::Error::duplicate_field("evaluationMessage"));
                            }
                            r#evaluation_message = Some(map_access.next_value()?);
                        }
                        Field::OutputParameters => {
                            if r#output_parameters.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputParameters"));
                            }
                            r#output_parameters = Some(map_access.next_value()?);
                        }
                        Field::Result => {
                            if r#result.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            r#result = Some(map_access.next_value()?);
                        }
                        Field::DataRequirement => {
                            if r#data_requirement.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataRequirement"));
                            }
                            r#data_requirement = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(GuidanceResponse {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#request_identifier,
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#module: r#module.ok_or(serde::de::Error::missing_field("module[x]"))?,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#subject,
                    r#encounter,
                    r#occurrence_date_time,
                    r#performer,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#evaluation_message: r#evaluation_message.unwrap_or(vec![]),
                    r#output_parameters,
                    r#result,
                    r#data_requirement: r#data_requirement.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
