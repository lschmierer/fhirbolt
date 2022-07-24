// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationAdministrationMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicationAdministrationMedication {
    fn default() -> MedicationAdministrationMedication {
        MedicationAdministrationMedication::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationAdministrationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for MedicationAdministrationEffective {
    fn default() -> MedicationAdministrationEffective {
        MedicationAdministrationEffective::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum MedicationAdministrationDosageRate {
    Ratio(Box<super::super::types::Ratio>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for MedicationAdministrationDosageRate {
    fn default() -> MedicationAdministrationDosageRate {
        MedicationAdministrationDosageRate::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationAdministrationPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for MedicationAdministrationPerformer {
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
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationAdministrationPerformer {
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
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationAdministrationPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationPerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationAdministrationPerformer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#function: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<super::super::types::Reference>> = None;
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
                    }
                }
                Ok(MedicationAdministrationPerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: r#actor.ok_or(serde::de::Error::missing_field("actor"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationAdministrationDosage {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose: Option<Box<super::super::types::Quantity>>,
    pub r#rate: Option<MedicationAdministrationDosageRate>,
}
impl serde::ser::Serialize for MedicationAdministrationDosage {
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
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#site.as_ref() {
            state.serialize_entry("site", some)?;
        }
        if let Some(some) = self.r#route.as_ref() {
            state.serialize_entry("route", some)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if let Some(some) = self.r#dose.as_ref() {
            state.serialize_entry("dose", some)?;
        }
        if let Some(some) = self.r#rate.as_ref() {
            match some {
                MedicationAdministrationDosageRate::Ratio(ref value) => {
                    state.serialize_entry("rateRatio", value)?;
                }
                MedicationAdministrationDosageRate::Quantity(ref value) => {
                    state.serialize_entry("rateQuantity", value)?;
                }
                MedicationAdministrationDosageRate::Invalid => {
                    return Err(serde::ser::Error::custom("rate is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationAdministrationDosage {
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
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "route")]
            Route,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "dose")]
            Dose,
            #[serde(rename = "rateRatio")]
            RateRatio,
            #[serde(rename = "rateQuantity")]
            RateQuantity,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationAdministrationDosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationDosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationAdministrationDosage, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#dose: Option<Box<super::super::types::Quantity>> = None;
                let mut r#rate: Option<MedicationAdministrationDosageRate> = None;
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
                        Field::Text => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TextPrimitiveElement => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some(map_access.next_value()?);
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            r#route = Some(map_access.next_value()?);
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                        Field::Dose => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("dose"));
                            }
                            r#dose = Some(map_access.next_value()?);
                        }
                        Field::RateRatio => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            r#rate = Some(MedicationAdministrationDosageRate::Ratio(
                                map_access.next_value()?,
                            ));
                        }
                        Field::RateQuantity => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            r#rate = Some(MedicationAdministrationDosageRate::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                    }
                }
                Ok(MedicationAdministrationDosage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#text,
                    r#site,
                    r#route,
                    r#method,
                    r#dose,
                    r#rate,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicationAdministration {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#instantiates: Vec<super::super::types::Uri>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#medication: MedicationAdministrationMedication,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#effective: MedicationAdministrationEffective,
    pub r#performer: Vec<MedicationAdministrationPerformer>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#dosage: Option<MedicationAdministrationDosage>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MedicationAdministration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationAdministration")?;
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
        if !self.r#instantiates.is_empty() {
            let values: Vec<_> = self.r#instantiates.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("instantiates", &values)?;
            }
            let requires_elements = self
                .r#instantiates
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#instantiates
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
                state.serialize_entry("_instantiates", &primitive_elements)?;
            }
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
        if !self.r#status_reason.is_empty() {
            state.serialize_entry("statusReason", &self.r#status_reason)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        match self.r#medication {
            MedicationAdministrationMedication::CodeableConcept(ref value) => {
                state.serialize_entry("medicationCodeableConcept", value)?;
            }
            MedicationAdministrationMedication::Reference(ref value) => {
                state.serialize_entry("medicationReference", value)?;
            }
            MedicationAdministrationMedication::Invalid => {
                return Err(serde::ser::Error::custom("medication is a required field"))
            }
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#context.as_ref() {
            state.serialize_entry("context", some)?;
        }
        if !self.r#supporting_information.is_empty() {
            state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
        }
        match self.r#effective {
            MedicationAdministrationEffective::DateTime(ref value) => {
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
            MedicationAdministrationEffective::Period(ref value) => {
                state.serialize_entry("effectivePeriod", value)?;
            }
            MedicationAdministrationEffective::Invalid => {
                return Err(serde::ser::Error::custom("effective is a required field"))
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if let Some(some) = self.r#request.as_ref() {
            state.serialize_entry("request", some)?;
        }
        if !self.r#device.is_empty() {
            state.serialize_entry("device", &self.r#device)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if let Some(some) = self.r#dosage.as_ref() {
            state.serialize_entry("dosage", some)?;
        }
        if !self.r#event_history.is_empty() {
            state.serialize_entry("eventHistory", &self.r#event_history)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicationAdministration {
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
            #[serde(rename = "instantiates")]
            Instantiates,
            #[serde(rename = "_instantiates")]
            InstantiatesPrimitiveElement,
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
            #[serde(rename = "medicationCodeableConcept")]
            MedicationCodeableConcept,
            #[serde(rename = "medicationReference")]
            MedicationReference,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "context")]
            Context,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "effectiveDateTime")]
            EffectiveDateTime,
            #[serde(rename = "_effectiveDateTime")]
            EffectiveDateTimePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "dosage")]
            Dosage,
            #[serde(rename = "eventHistory")]
            EventHistory,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicationAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministration")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationAdministration, V::Error>
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
                let mut r#instantiates: Option<Vec<super::super::types::Uri>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#medication: Option<MedicationAdministrationMedication> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#context: Option<Box<super::super::types::Reference>> = None;
                let mut r#supporting_information: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#effective: Option<MedicationAdministrationEffective> = None;
                let mut r#performer: Option<Vec<MedicationAdministrationPerformer>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#device: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#dosage: Option<MedicationAdministrationDosage> = None;
                let mut r#event_history: Option<Vec<Box<super::super::types::Reference>>> = None;
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
                        Field::Instantiates => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("instantiates"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::InstantiatesPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#instantiates.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_instantiates"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
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
                        Field::MedicationCodeableConcept => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationCodeableConcept",
                                ));
                            }
                            r#medication =
                                Some(MedicationAdministrationMedication::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                        }
                        Field::MedicationReference => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "medicationReference",
                                ));
                            }
                            r#medication = Some(MedicationAdministrationMedication::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        Field::Context => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        Field::SupportingInformation => {
                            if r#supporting_information.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supportingInformation",
                                ));
                            }
                            r#supporting_information = Some(map_access.next_value()?);
                        }
                        Field::EffectiveDateTime => {
                            let r#enum = r#effective.get_or_insert(
                                MedicationAdministrationEffective::DateTime(Default::default()),
                            );
                            if let MedicationAdministrationEffective::DateTime(variant) = r#enum {
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
                                MedicationAdministrationEffective::DateTime(Default::default()),
                            );
                            if let MedicationAdministrationEffective::DateTime(variant) = r#enum {
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
                            r#effective = Some(MedicationAdministrationEffective::Period(
                                map_access.next_value()?,
                            ));
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
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(map_access.next_value()?);
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        Field::Note => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        Field::Dosage => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            r#dosage = Some(map_access.next_value()?);
                        }
                        Field::EventHistory => {
                            if r#event_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventHistory"));
                            }
                            r#event_history = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicationAdministration {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#instantiates: r#instantiates.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#status_reason: r#status_reason.unwrap_or(vec![]),
                    r#category,
                    r#medication: r#medication
                        .ok_or(serde::de::Error::missing_field("medication"))?,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#context,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#effective: r#effective.ok_or(serde::de::Error::missing_field("effective"))?,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#request,
                    r#device: r#device.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#dosage,
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
