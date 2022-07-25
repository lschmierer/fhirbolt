// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct AdverseEventSuspectEntityCausality {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#assessment: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_relatedness: Option<super::super::types::String>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for AdverseEventSuspectEntityCausality {
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
        if let Some(some) = self.r#assessment.as_ref() {
            state.serialize_entry("assessment", some)?;
        }
        if let Some(some) = self.r#product_relatedness.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("productRelatedness", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_productRelatedness", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#author.as_ref() {
            state.serialize_entry("author", some)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEventSuspectEntityCausality {
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
            #[serde(rename = "assessment")]
            Assessment,
            #[serde(rename = "productRelatedness")]
            ProductRelatedness,
            #[serde(rename = "_productRelatedness")]
            ProductRelatednessPrimitiveElement,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "method")]
            Method,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEventSuspectEntityCausality;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEventSuspectEntityCausality")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdverseEventSuspectEntityCausality, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#assessment: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_relatedness: Option<super::super::types::String> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::Assessment => {
                            if r#assessment.is_some() {
                                return Err(serde::de::Error::duplicate_field("assessment"));
                            }
                            r#assessment = Some(map_access.next_value()?);
                        }
                        Field::ProductRelatedness => {
                            let some = r#product_relatedness.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productRelatedness",
                                ));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::ProductRelatednessPrimitiveElement => {
                            let some = r#product_relatedness.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_productRelatedness",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Author => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(map_access.next_value()?);
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AdverseEventSuspectEntityCausality {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#assessment,
                    r#product_relatedness,
                    r#author,
                    r#method,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AdverseEventSuspectEntity {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#instance: Box<super::super::types::Reference>,
    pub r#causality: Vec<AdverseEventSuspectEntityCausality>,
}
impl serde::ser::Serialize for AdverseEventSuspectEntity {
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
        state.serialize_entry("instance", &self.r#instance)?;
        if !self.r#causality.is_empty() {
            state.serialize_entry("causality", &self.r#causality)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEventSuspectEntity {
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
            #[serde(rename = "instance")]
            Instance,
            #[serde(rename = "causality")]
            Causality,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEventSuspectEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEventSuspectEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AdverseEventSuspectEntity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#instance: Option<Box<super::super::types::Reference>> = None;
                let mut r#causality: Option<Vec<AdverseEventSuspectEntityCausality>> = None;
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
                        Field::Instance => {
                            if r#instance.is_some() {
                                return Err(serde::de::Error::duplicate_field("instance"));
                            }
                            r#instance = Some(map_access.next_value()?);
                        }
                        Field::Causality => {
                            if r#causality.is_some() {
                                return Err(serde::de::Error::duplicate_field("causality"));
                            }
                            r#causality = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AdverseEventSuspectEntity {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#instance: r#instance.ok_or(serde::de::Error::missing_field("instance"))?,
                    r#causality: r#causality.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AdverseEvent {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#actuality: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#event: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#detected: Option<super::super::types::DateTime>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#resulting_condition: Vec<Box<super::super::types::Reference>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
    pub r#subject_medical_history: Vec<Box<super::super::types::Reference>>,
    pub r#reference_document: Vec<Box<super::super::types::Reference>>,
    pub r#study: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for AdverseEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "AdverseEvent")?;
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#actuality.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("actuality", &some)?;
        }
        if self.r#actuality.id.is_some() || !self.r#actuality.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#actuality.id,
                extension: &self.r#actuality.extension,
            };
            state.serialize_entry("_actuality", &primitive_element)?;
        }
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if let Some(some) = self.r#event.as_ref() {
            state.serialize_entry("event", some)?;
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("date", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#detected.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("detected", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_detected", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#recorded_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("recordedDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_recordedDate", &primitive_element)?;
            }
        }
        if !self.r#resulting_condition.is_empty() {
            state.serialize_entry("resultingCondition", &self.r#resulting_condition)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if let Some(some) = self.r#seriousness.as_ref() {
            state.serialize_entry("seriousness", some)?;
        }
        if let Some(some) = self.r#severity.as_ref() {
            state.serialize_entry("severity", some)?;
        }
        if let Some(some) = self.r#outcome.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        if let Some(some) = self.r#recorder.as_ref() {
            state.serialize_entry("recorder", some)?;
        }
        if !self.r#contributor.is_empty() {
            state.serialize_entry("contributor", &self.r#contributor)?;
        }
        if !self.r#suspect_entity.is_empty() {
            state.serialize_entry("suspectEntity", &self.r#suspect_entity)?;
        }
        if !self.r#subject_medical_history.is_empty() {
            state.serialize_entry("subjectMedicalHistory", &self.r#subject_medical_history)?;
        }
        if !self.r#reference_document.is_empty() {
            state.serialize_entry("referenceDocument", &self.r#reference_document)?;
        }
        if !self.r#study.is_empty() {
            state.serialize_entry("study", &self.r#study)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AdverseEvent {
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
            #[serde(rename = "actuality")]
            Actuality,
            #[serde(rename = "_actuality")]
            ActualityPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "event")]
            Event,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "detected")]
            Detected,
            #[serde(rename = "_detected")]
            DetectedPrimitiveElement,
            #[serde(rename = "recordedDate")]
            RecordedDate,
            #[serde(rename = "_recordedDate")]
            RecordedDatePrimitiveElement,
            #[serde(rename = "resultingCondition")]
            ResultingCondition,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "seriousness")]
            Seriousness,
            #[serde(rename = "severity")]
            Severity,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "recorder")]
            Recorder,
            #[serde(rename = "contributor")]
            Contributor,
            #[serde(rename = "suspectEntity")]
            SuspectEntity,
            #[serde(rename = "subjectMedicalHistory")]
            SubjectMedicalHistory,
            #[serde(rename = "referenceDocument")]
            ReferenceDocument,
            #[serde(rename = "study")]
            Study,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdverseEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdverseEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AdverseEvent, V::Error>
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
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#actuality: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#event: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#detected: Option<super::super::types::DateTime> = None;
                let mut r#recorded_date: Option<super::super::types::DateTime> = None;
                let mut r#resulting_condition: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#seriousness: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#severity: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#outcome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#recorder: Option<Box<super::super::types::Reference>> = None;
                let mut r#contributor: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#suspect_entity: Option<Vec<AdverseEventSuspectEntity>> = None;
                let mut r#subject_medical_history: Option<
                    Vec<Box<super::super::types::Reference>>,
                > = None;
                let mut r#reference_document: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#study: Option<Vec<Box<super::super::types::Reference>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "AdverseEvent" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"AdverseEvent",
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
                        Field::Actuality => {
                            let some = r#actuality.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("actuality"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::ActualityPrimitiveElement => {
                            let some = r#actuality.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_actuality"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Event => {
                            if r#event.is_some() {
                                return Err(serde::de::Error::duplicate_field("event"));
                            }
                            r#event = Some(map_access.next_value()?);
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
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
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
                        Field::Detected => {
                            let some = r#detected.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("detected"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::DetectedPrimitiveElement => {
                            let some = r#detected.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_detected"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::RecordedDate => {
                            let some = r#recorded_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("recordedDate"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::RecordedDatePrimitiveElement => {
                            let some = r#recorded_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_recordedDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ResultingCondition => {
                            if r#resulting_condition.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "resultingCondition",
                                ));
                            }
                            r#resulting_condition = Some(map_access.next_value()?);
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        Field::Seriousness => {
                            if r#seriousness.is_some() {
                                return Err(serde::de::Error::duplicate_field("seriousness"));
                            }
                            r#seriousness = Some(map_access.next_value()?);
                        }
                        Field::Severity => {
                            if r#severity.is_some() {
                                return Err(serde::de::Error::duplicate_field("severity"));
                            }
                            r#severity = Some(map_access.next_value()?);
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            r#outcome = Some(map_access.next_value()?);
                        }
                        Field::Recorder => {
                            if r#recorder.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorder"));
                            }
                            r#recorder = Some(map_access.next_value()?);
                        }
                        Field::Contributor => {
                            if r#contributor.is_some() {
                                return Err(serde::de::Error::duplicate_field("contributor"));
                            }
                            r#contributor = Some(map_access.next_value()?);
                        }
                        Field::SuspectEntity => {
                            if r#suspect_entity.is_some() {
                                return Err(serde::de::Error::duplicate_field("suspectEntity"));
                            }
                            r#suspect_entity = Some(map_access.next_value()?);
                        }
                        Field::SubjectMedicalHistory => {
                            if r#subject_medical_history.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectMedicalHistory",
                                ));
                            }
                            r#subject_medical_history = Some(map_access.next_value()?);
                        }
                        Field::ReferenceDocument => {
                            if r#reference_document.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceDocument"));
                            }
                            r#reference_document = Some(map_access.next_value()?);
                        }
                        Field::Study => {
                            if r#study.is_some() {
                                return Err(serde::de::Error::duplicate_field("study"));
                            }
                            r#study = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AdverseEvent {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#actuality: r#actuality.ok_or(serde::de::Error::missing_field("actuality"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#event,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#encounter,
                    r#date,
                    r#detected,
                    r#recorded_date,
                    r#resulting_condition: r#resulting_condition.unwrap_or(vec![]),
                    r#location,
                    r#seriousness,
                    r#severity,
                    r#outcome,
                    r#recorder,
                    r#contributor: r#contributor.unwrap_or(vec![]),
                    r#suspect_entity: r#suspect_entity.unwrap_or(vec![]),
                    r#subject_medical_history: r#subject_medical_history.unwrap_or(vec![]),
                    r#reference_document: r#reference_document.unwrap_or(vec![]),
                    r#study: r#study.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
