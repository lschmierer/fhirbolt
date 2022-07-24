// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ClinicalImpressionEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClinicalImpressionEffective {
    fn default() -> ClinicalImpressionEffective {
        ClinicalImpressionEffective::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClinicalImpressionInvestigation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#item: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClinicalImpressionInvestigation {
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
        state.serialize_entry("code", &self.r#code)?;
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpressionInvestigation {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "item")]
            Item,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpressionInvestigation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpressionInvestigation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClinicalImpressionInvestigation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#item: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Item => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            r#item = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClinicalImpressionInvestigation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#item: r#item.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClinicalImpressionFinding {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>>,
    pub r#item_reference: Option<Box<super::super::types::Reference>>,
    pub r#basis: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ClinicalImpressionFinding {
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
        if let Some(some) = self.r#item_codeable_concept.as_ref() {
            state.serialize_entry("itemCodeableConcept", some)?;
        }
        if let Some(some) = self.r#item_reference.as_ref() {
            state.serialize_entry("itemReference", some)?;
        }
        if let Some(some) = self.r#basis.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("basis", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_basis", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpressionFinding {
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
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            #[serde(rename = "itemReference")]
            ItemReference,
            #[serde(rename = "basis")]
            Basis,
            #[serde(rename = "_basis")]
            BasisPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpressionFinding;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpressionFinding")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClinicalImpressionFinding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_codeable_concept: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#item_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#basis: Option<super::super::types::String> = None;
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
                        Field::ItemCodeableConcept => {
                            if r#item_codeable_concept.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "itemCodeableConcept",
                                ));
                            }
                            r#item_codeable_concept = Some(map_access.next_value()?);
                        }
                        Field::ItemReference => {
                            if r#item_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemReference"));
                            }
                            r#item_reference = Some(map_access.next_value()?);
                        }
                        Field::Basis => {
                            let some = r#basis.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("basis"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::BasisPrimitiveElement => {
                            let some = r#basis.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_basis"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(ClinicalImpressionFinding {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_codeable_concept,
                    r#item_reference,
                    r#basis,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClinicalImpression {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#effective: Option<ClinicalImpressionEffective>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#assessor: Option<Box<super::super::types::Reference>>,
    pub r#previous: Option<Box<super::super::types::Reference>>,
    pub r#problem: Vec<Box<super::super::types::Reference>>,
    pub r#investigation: Vec<ClinicalImpressionInvestigation>,
    pub r#protocol: Vec<super::super::types::Uri>,
    pub r#summary: Option<super::super::types::String>,
    pub r#finding: Vec<ClinicalImpressionFinding>,
    pub r#prognosis_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#prognosis_reference: Vec<Box<super::super::types::Reference>>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for ClinicalImpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ClinicalImpression")?;
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
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#effective.as_ref() {
            match some {
                ClinicalImpressionEffective::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("effectiveDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                    }
                }
                ClinicalImpressionEffective::Period(ref value) => {
                    state.serialize_entry("effectivePeriod", value)?;
                }
                ClinicalImpressionEffective::Invalid => {
                    return Err(serde::ser::Error::custom("effective is invalid"))
                }
            }
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#assessor.as_ref() {
            state.serialize_entry("assessor", some)?;
        }
        if let Some(some) = self.r#previous.as_ref() {
            state.serialize_entry("previous", some)?;
        }
        if !self.r#problem.is_empty() {
            state.serialize_entry("problem", &self.r#problem)?;
        }
        if !self.r#investigation.is_empty() {
            state.serialize_entry("investigation", &self.r#investigation)?;
        }
        if !self.r#protocol.is_empty() {
            let values: Vec<_> = self.r#protocol.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("protocol", &values)?;
            }
            let requires_elements = self
                .r#protocol
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#protocol
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
                state.serialize_entry("_protocol", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#summary.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("summary", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_summary", &primitive_element)?;
            }
        }
        if !self.r#finding.is_empty() {
            state.serialize_entry("finding", &self.r#finding)?;
        }
        if !self.r#prognosis_codeable_concept.is_empty() {
            state.serialize_entry(
                "prognosisCodeableConcept",
                &self.r#prognosis_codeable_concept,
            )?;
        }
        if !self.r#prognosis_reference.is_empty() {
            state.serialize_entry("prognosisReference", &self.r#prognosis_reference)?;
        }
        if !self.r#supporting_info.is_empty() {
            state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClinicalImpression {
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
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "effectiveDateTime")]
            EffectiveDateTime,
            #[serde(rename = "_effectiveDateTime")]
            EffectiveDateTimePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "assessor")]
            Assessor,
            #[serde(rename = "previous")]
            Previous,
            #[serde(rename = "problem")]
            Problem,
            #[serde(rename = "investigation")]
            Investigation,
            #[serde(rename = "protocol")]
            Protocol,
            #[serde(rename = "_protocol")]
            ProtocolPrimitiveElement,
            #[serde(rename = "summary")]
            Summary,
            #[serde(rename = "_summary")]
            SummaryPrimitiveElement,
            #[serde(rename = "finding")]
            Finding,
            #[serde(rename = "prognosisCodeableConcept")]
            PrognosisCodeableConcept,
            #[serde(rename = "prognosisReference")]
            PrognosisReference,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "note")]
            Note,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClinicalImpression;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClinicalImpression")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClinicalImpression, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#effective: Option<ClinicalImpressionEffective> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#assessor: Option<Box<super::super::types::Reference>> = None;
                let mut r#previous: Option<Box<super::super::types::Reference>> = None;
                let mut r#problem: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#investigation: Option<Vec<ClinicalImpressionInvestigation>> = None;
                let mut r#protocol: Option<Vec<super::super::types::Uri>> = None;
                let mut r#summary: Option<super::super::types::String> = None;
                let mut r#finding: Option<Vec<ClinicalImpressionFinding>> = None;
                let mut r#prognosis_codeable_concept: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#prognosis_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "ClinicalImpression" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"ClinicalImpression",
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
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
                        Field::EffectiveDateTime => {
                            let r#enum = r#effective.get_or_insert(
                                ClinicalImpressionEffective::DateTime(Default::default()),
                            );
                            if let ClinicalImpressionEffective::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectiveDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("effective[x]"));
                            }
                        }
                        Field::EffectiveDateTimePrimitiveElement => {
                            let r#enum = r#effective.get_or_insert(
                                ClinicalImpressionEffective::DateTime(Default::default()),
                            );
                            if let ClinicalImpressionEffective::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_effectiveDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_effective[x]"));
                            }
                        }
                        Field::EffectivePeriod => {
                            if r#effective.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            r#effective = Some(ClinicalImpressionEffective::Period(
                                map_access.next_value()?,
                            ));
                        }
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_date"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Assessor => {
                            if r#assessor.is_some() {
                                return Err(serde::de::Error::duplicate_field("assessor"));
                            }
                            r#assessor = Some(map_access.next_value()?);
                        }
                        Field::Previous => {
                            if r#previous.is_some() {
                                return Err(serde::de::Error::duplicate_field("previous"));
                            }
                            r#previous = Some(map_access.next_value()?);
                        }
                        Field::Problem => {
                            if r#problem.is_some() {
                                return Err(serde::de::Error::duplicate_field("problem"));
                            }
                            r#problem = Some(map_access.next_value()?);
                        }
                        Field::Investigation => {
                            if r#investigation.is_some() {
                                return Err(serde::de::Error::duplicate_field("investigation"));
                            }
                            r#investigation = Some(map_access.next_value()?);
                        }
                        Field::Protocol => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#protocol.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ProtocolPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#protocol.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_protocol"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Summary => {
                            let some = r#summary.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("summary"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SummaryPrimitiveElement => {
                            let some = r#summary.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_summary"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Finding => {
                            if r#finding.is_some() {
                                return Err(serde::de::Error::duplicate_field("finding"));
                            }
                            r#finding = Some(map_access.next_value()?);
                        }
                        Field::PrognosisCodeableConcept => {
                            if r#prognosis_codeable_concept.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "prognosisCodeableConcept",
                                ));
                            }
                            r#prognosis_codeable_concept = Some(map_access.next_value()?);
                        }
                        Field::PrognosisReference => {
                            if r#prognosis_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "prognosisReference",
                                ));
                            }
                            r#prognosis_reference = Some(map_access.next_value()?);
                        }
                        Field::SupportingInfo => {
                            if r#supporting_info.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportingInfo"));
                            }
                            r#supporting_info = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClinicalImpression {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#status_reason,
                    r#code,
                    r#description,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#effective,
                    r#date,
                    r#assessor,
                    r#previous,
                    r#problem: r#problem.unwrap_or(vec![]),
                    r#investigation: r#investigation.unwrap_or(vec![]),
                    r#protocol: r#protocol.unwrap_or(vec![]),
                    r#summary,
                    r#finding: r#finding.unwrap_or(vec![]),
                    r#prognosis_codeable_concept: r#prognosis_codeable_concept.unwrap_or(vec![]),
                    r#prognosis_reference: r#prognosis_reference.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
