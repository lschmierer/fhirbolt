// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ProcedurePerformed {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for ProcedurePerformed {
    fn default() -> ProcedurePerformed {
        ProcedurePerformed::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ProcedurePerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ProcedurePerformer {
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
        if let Some(some) = self.r#function.as_ref() {
            state.serialize_entry("function", some)?;
        }
        state.serialize_entry("actor", &self.r#actor)?;
        if let Some(some) = self.r#on_behalf_of.as_ref() {
            state.serialize_entry("onBehalfOf", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ProcedurePerformer {
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
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProcedurePerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProcedurePerformer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProcedurePerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
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
                        Field::OnBehalfOf => {
                            if r#on_behalf_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                            }
                            r#on_behalf_of = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ProcedurePerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: r#actor.ok_or(serde::de::Error::missing_field("actor"))?,
                    r#on_behalf_of,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ProcedureFocalDevice {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Option<Box<super::super::types::CodeableConcept>>,
    pub r#manipulated: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ProcedureFocalDevice {
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
        if let Some(some) = self.r#action.as_ref() {
            state.serialize_entry("action", some)?;
        }
        state.serialize_entry("manipulated", &self.r#manipulated)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ProcedureFocalDevice {
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
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "manipulated")]
            Manipulated,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProcedureFocalDevice;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProcedureFocalDevice")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProcedureFocalDevice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#action: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#manipulated: Option<Box<super::super::types::Reference>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Action => {
                            if r#action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#action = Some(map_access.next_value()?);
                        }
                        Field::Manipulated => {
                            if r#manipulated.is_some() {
                                return Err(serde::de::Error::duplicate_field("manipulated"));
                            }
                            r#manipulated = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ProcedureFocalDevice {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#action,
                    r#manipulated: r#manipulated
                        .ok_or(serde::de::Error::missing_field("manipulated"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Procedure {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#performed: Option<ProcedurePerformed>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#performer: Vec<ProcedurePerformer>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#report: Vec<Box<super::super::types::Reference>>,
    pub r#complication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#complication_detail: Vec<Box<super::super::types::Reference>>,
    pub r#follow_up: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#focal_device: Vec<ProcedureFocalDevice>,
    pub r#used_reference: Vec<Box<super::super::types::Reference>>,
    pub r#used_code: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for Procedure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Procedure")?;
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
        if !self.r#instantiates_canonical.is_empty() {
            let values: Vec<_> = self
                .r#instantiates_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesCanonical", &values)?;
            }
            let requires_elements = self
                .r#instantiates_canonical
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_canonical
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
            }
        }
        if !self.r#instantiates_uri.is_empty() {
            let values: Vec<_> = self.r#instantiates_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiatesUri", &values)?;
            }
            let requires_elements = self
                .r#instantiates_uri
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates_uri
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_instantiatesUri", &primitive_elements)?;
            }
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
        if let Some(some) = self.r#status_reason.as_ref() {
            state.serialize_entry("statusReason", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#performed.as_ref() {
            match some {
                ProcedurePerformed::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("performedDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_performedDateTime", &primitive_element)?;
                    }
                }
                ProcedurePerformed::Period(ref value) => {
                    state.serialize_entry("performedPeriod", value)?;
                }
                ProcedurePerformed::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("performedString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_performedString", &primitive_element)?;
                    }
                }
                ProcedurePerformed::Age(ref value) => {
                    state.serialize_entry("performedAge", value)?;
                }
                ProcedurePerformed::Range(ref value) => {
                    state.serialize_entry("performedRange", value)?;
                }
                ProcedurePerformed::Invalid => {
                    return Err(serde::ser::Error::custom("performed is invalid"))
                }
            }
        }
        if let Some(some) = self.r#recorder.as_ref() {
            state.serialize_entry("recorder", some)?;
        }
        if let Some(some) = self.r#asserter.as_ref() {
            state.serialize_entry("asserter", some)?;
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
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
        if !self.r#body_site.is_empty() {
            state.serialize_entry("bodySite", &self.r#body_site)?;
        }
        if let Some(some) = self.r#outcome.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        if !self.r#report.is_empty() {
            state.serialize_entry("report", &self.r#report)?;
        }
        if !self.r#complication.is_empty() {
            state.serialize_entry("complication", &self.r#complication)?;
        }
        if !self.r#complication_detail.is_empty() {
            state.serialize_entry("complicationDetail", &self.r#complication_detail)?;
        }
        if !self.r#follow_up.is_empty() {
            state.serialize_entry("followUp", &self.r#follow_up)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#focal_device.is_empty() {
            state.serialize_entry("focalDevice", &self.r#focal_device)?;
        }
        if !self.r#used_reference.is_empty() {
            state.serialize_entry("usedReference", &self.r#used_reference)?;
        }
        if !self.r#used_code.is_empty() {
            state.serialize_entry("usedCode", &self.r#used_code)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Procedure {
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "performedDateTime")]
            PerformedDateTime,
            #[serde(rename = "_performedDateTime")]
            PerformedDateTimePrimitiveElement,
            #[serde(rename = "performedPeriod")]
            PerformedPeriod,
            #[serde(rename = "performedString")]
            PerformedString,
            #[serde(rename = "_performedString")]
            PerformedStringPrimitiveElement,
            #[serde(rename = "performedAge")]
            PerformedAge,
            #[serde(rename = "performedRange")]
            PerformedRange,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "asserter")]
            Asserter,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "report")]
            Report,
            #[serde(rename = "complication")]
            Complication,
            #[serde(rename = "complicationDetail")]
            ComplicationDetail,
            #[serde(rename = "followUp")]
            FollowUp,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "focalDevice")]
            FocalDevice,
            #[serde(rename = "usedReference")]
            UsedReference,
            #[serde(rename = "usedCode")]
            UsedCode,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Procedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Procedure")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Procedure, V::Error>
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
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#performed: Option<ProcedurePerformed> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#asserter: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Vec<ProcedurePerformer>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#report: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#complication: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#complication_detail: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#follow_up: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#focal_device: Option<Vec<ProcedureFocalDevice>> = None;
                let mut r#used_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#used_code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Procedure" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Procedure",
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
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::InstantiatesCanonical => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
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
                                    "instantiatesCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesCanonicalPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_canonical.get_or_insert(
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
                                    "_instantiatesCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::InstantiatesUri => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#instantiates_uri.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
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
                        Field::StatusReason => {
                            if r#status_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusReason"));
                            }
                            r#status_reason = Some(map_access.next_value()?);
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
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
                        Field::PerformedDateTime => {
                            let r#enum = r#performed
                                .get_or_insert(ProcedurePerformed::DateTime(Default::default()));
                            if let ProcedurePerformed::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("performed[x]"));
                            }
                        }
                        Field::PerformedDateTimePrimitiveElement => {
                            let r#enum = r#performed
                                .get_or_insert(ProcedurePerformed::DateTime(Default::default()));
                            if let ProcedurePerformed::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_performedDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_performed[x]"));
                            }
                        }
                        Field::PerformedPeriod => {
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedPeriod"));
                            }
                            r#performed =
                                Some(ProcedurePerformed::Period(map_access.next_value()?));
                        }
                        Field::PerformedString => {
                            let r#enum = r#performed
                                .get_or_insert(ProcedurePerformed::String(Default::default()));
                            if let ProcedurePerformed::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "performedString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("performed[x]"));
                            }
                        }
                        Field::PerformedStringPrimitiveElement => {
                            let r#enum = r#performed
                                .get_or_insert(ProcedurePerformed::String(Default::default()));
                            if let ProcedurePerformed::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_performedString",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_performed[x]"));
                            }
                        }
                        Field::PerformedAge => {
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedAge"));
                            }
                            r#performed = Some(ProcedurePerformed::Age(map_access.next_value()?));
                        }
                        Field::PerformedRange => {
                            if r#performed.is_some() {
                                return Err(serde::de::Error::duplicate_field("performedRange"));
                            }
                            r#performed = Some(ProcedurePerformed::Range(map_access.next_value()?));
                        }
                        Field::Recorder => {
                            if r#recorder.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorder"));
                            }
                            r#recorder = Some(map_access.next_value()?);
                        }
                        Field::Asserter => {
                            if r#asserter.is_some() {
                                return Err(serde::de::Error::duplicate_field("asserter"));
                            }
                            r#asserter = Some(map_access.next_value()?);
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
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
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            r#outcome = Some(map_access.next_value()?);
                        }
                        Field::Report => {
                            if r#report.is_some() {
                                return Err(serde::de::Error::duplicate_field("report"));
                            }
                            r#report = Some(map_access.next_value()?);
                        }
                        Field::Complication => {
                            if r#complication.is_some() {
                                return Err(serde::de::Error::duplicate_field("complication"));
                            }
                            r#complication = Some(map_access.next_value()?);
                        }
                        Field::ComplicationDetail => {
                            if r#complication_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "complicationDetail",
                                ));
                            }
                            r#complication_detail = Some(map_access.next_value()?);
                        }
                        Field::FollowUp => {
                            if r#follow_up.is_some() {
                                return Err(serde::de::Error::duplicate_field("followUp"));
                            }
                            r#follow_up = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::FocalDevice => {
                            if r#focal_device.is_some() {
                                return Err(serde::de::Error::duplicate_field("focalDevice"));
                            }
                            r#focal_device = Some(map_access.next_value()?);
                        }
                        Field::UsedReference => {
                            if r#used_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedReference"));
                            }
                            r#used_reference = Some(map_access.next_value()?);
                        }
                        Field::UsedCode => {
                            if r#used_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedCode"));
                            }
                            r#used_code = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Procedure {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                    r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#status_reason,
                    r#category,
                    r#code,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#performed,
                    r#recorder,
                    r#asserter,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#location,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#body_site: r#body_site.unwrap_or(vec![]),
                    r#outcome,
                    r#report: r#report.unwrap_or(vec![]),
                    r#complication: r#complication.unwrap_or(vec![]),
                    r#complication_detail: r#complication_detail.unwrap_or(vec![]),
                    r#follow_up: r#follow_up.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#focal_device: r#focal_device.unwrap_or(vec![]),
                    r#used_reference: r#used_reference.unwrap_or(vec![]),
                    r#used_code: r#used_code.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
