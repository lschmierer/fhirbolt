// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ChargeItemOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
impl Default for ChargeItemOccurrence {
    fn default() -> ChargeItemOccurrence {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ChargeItemProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
impl Default for ChargeItemProduct {
    fn default() -> ChargeItemProduct {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ChargeItemPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ChargeItemPerformer {
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
impl<'de> serde::de::Deserialize<'de> for ChargeItemPerformer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChargeItemPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ChargeItemPerformer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ChargeItemPerformer, V::Error>
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
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "function" => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some(map_access.next_value()?);
                        }
                        "actor" => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "function", "actor"],
                            ))
                        }
                    }
                }
                Ok(ChargeItemPerformer {
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
pub struct ChargeItem {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#definition_uri: Vec<super::super::types::Uri>,
    pub r#definition_canonical: Vec<super::super::types::Canonical>,
    pub r#status: super::super::types::Code,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#occurrence: Option<ChargeItemOccurrence>,
    pub r#performer: Vec<ChargeItemPerformer>,
    pub r#performing_organization: Option<Box<super::super::types::Reference>>,
    pub r#requesting_organization: Option<Box<super::super::types::Reference>>,
    pub r#cost_center: Option<Box<super::super::types::Reference>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#bodysite: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#factor_override: Option<super::super::types::Decimal>,
    pub r#price_override: Option<Box<super::super::types::Money>>,
    pub r#override_reason: Option<super::super::types::String>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#entered_date: Option<super::super::types::DateTime>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#service: Vec<Box<super::super::types::Reference>>,
    pub r#product: Option<ChargeItemProduct>,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ChargeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ChargeItem")?;
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
        if !self.r#definition_uri.is_empty() {
            let values: Vec<_> = self.r#definition_uri.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("definitionUri", &values)?;
            }
            let requires_elements = self
                .r#definition_uri
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#definition_uri
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
                state.serialize_entry("_definitionUri", &primitive_elements)?;
            }
        }
        if !self.r#definition_canonical.is_empty() {
            let values: Vec<_> = self
                .r#definition_canonical
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("definitionCanonical", &values)?;
            }
            let requires_elements = self
                .r#definition_canonical
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#definition_canonical
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
                state.serialize_entry("_definitionCanonical", &primitive_elements)?;
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
        if !self.r#part_of.is_empty() {
            state.serialize_entry("partOf", &self.r#part_of)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#context.as_ref() {
            state.serialize_entry("context", some)?;
        }
        if let Some(some) = self.r#occurrence.as_ref() {
            match some {
                ChargeItemOccurrence::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("occurrenceDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                    }
                }
                ChargeItemOccurrence::Period(ref value) => {
                    state.serialize_entry("occurrencePeriod", value)?;
                }
                ChargeItemOccurrence::Timing(ref value) => {
                    state.serialize_entry("occurrenceTiming", value)?;
                }
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if let Some(some) = self.r#performing_organization.as_ref() {
            state.serialize_entry("performingOrganization", some)?;
        }
        if let Some(some) = self.r#requesting_organization.as_ref() {
            state.serialize_entry("requestingOrganization", some)?;
        }
        if let Some(some) = self.r#cost_center.as_ref() {
            state.serialize_entry("costCenter", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if !self.r#bodysite.is_empty() {
            state.serialize_entry("bodysite", &self.r#bodysite)?;
        }
        if let Some(some) = self.r#factor_override.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factorOverride", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factorOverride", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#price_override.as_ref() {
            state.serialize_entry("priceOverride", some)?;
        }
        if let Some(some) = self.r#override_reason.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("overrideReason", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_overrideReason", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#enterer.as_ref() {
            state.serialize_entry("enterer", some)?;
        }
        if let Some(some) = self.r#entered_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("enteredDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_enteredDate", &primitive_element)?;
            }
        }
        if !self.r#reason.is_empty() {
            state.serialize_entry("reason", &self.r#reason)?;
        }
        if !self.r#service.is_empty() {
            state.serialize_entry("service", &self.r#service)?;
        }
        if let Some(some) = self.r#product.as_ref() {
            match some {
                ChargeItemProduct::Reference(ref value) => {
                    state.serialize_entry("productReference", value)?;
                }
                ChargeItemProduct::CodeableConcept(ref value) => {
                    state.serialize_entry("productCodeableConcept", value)?;
                }
            }
        }
        if !self.r#account.is_empty() {
            state.serialize_entry("account", &self.r#account)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#supporting_information.is_empty() {
            state.serialize_entry("supportingInformation", &self.r#supporting_information)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ChargeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChargeItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ChargeItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ChargeItem, V::Error>
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
                let mut r#definition_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#definition_canonical: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#context: Option<Box<super::super::types::Reference>> = None;
                let mut r#occurrence: Option<ChargeItemOccurrence> = None;
                let mut r#performer: Option<Vec<ChargeItemPerformer>> = None;
                let mut r#performing_organization: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#requesting_organization: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#cost_center: Option<Box<super::super::types::Reference>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#bodysite: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#factor_override: Option<super::super::types::Decimal> = None;
                let mut r#price_override: Option<Box<super::super::types::Money>> = None;
                let mut r#override_reason: Option<super::super::types::String> = None;
                let mut r#enterer: Option<Box<super::super::types::Reference>> = None;
                let mut r#entered_date: Option<super::super::types::DateTime> = None;
                let mut r#reason: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#service: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#product: Option<ChargeItemProduct> = None;
                let mut r#account: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#supporting_information: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
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
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "definitionUri" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#definition_uri.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("definitionUri"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_definitionUri" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#definition_uri.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_definitionUri"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "definitionCanonical" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#definition_canonical
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "definitionCanonical",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_definitionCanonical" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#definition_canonical
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                    "_definitionCanonical",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
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
                        "partOf" => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "context" => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        "occurrenceDateTime" => {
                            let r#enum = r#occurrence
                                .get_or_insert(ChargeItemOccurrence::DateTime(Default::default()));
                            if let ChargeItemOccurrence::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        "_occurrenceDateTime" => {
                            let r#enum = r#occurrence
                                .get_or_insert(ChargeItemOccurrence::DateTime(Default::default()));
                            if let ChargeItemOccurrence::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_occurrence[x]"));
                            }
                        }
                        "occurrencePeriod" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence =
                                Some(ChargeItemOccurrence::Period(map_access.next_value()?));
                        }
                        "occurrenceTiming" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence =
                                Some(ChargeItemOccurrence::Timing(map_access.next_value()?));
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "performingOrganization" => {
                            if r#performing_organization.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "performingOrganization",
                                ));
                            }
                            r#performing_organization = Some(map_access.next_value()?);
                        }
                        "requestingOrganization" => {
                            if r#requesting_organization.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "requestingOrganization",
                                ));
                            }
                            r#requesting_organization = Some(map_access.next_value()?);
                        }
                        "costCenter" => {
                            if r#cost_center.is_some() {
                                return Err(serde::de::Error::duplicate_field("costCenter"));
                            }
                            r#cost_center = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "bodysite" => {
                            if r#bodysite.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodysite"));
                            }
                            r#bodysite = Some(map_access.next_value()?);
                        }
                        "factorOverride" => {
                            let some = r#factor_override.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factorOverride"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factorOverride" => {
                            let some = r#factor_override.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factorOverride"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "priceOverride" => {
                            if r#price_override.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceOverride"));
                            }
                            r#price_override = Some(map_access.next_value()?);
                        }
                        "overrideReason" => {
                            let some = r#override_reason.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideReason"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_overrideReason" => {
                            let some = r#override_reason.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_overrideReason"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "enterer" => {
                            if r#enterer.is_some() {
                                return Err(serde::de::Error::duplicate_field("enterer"));
                            }
                            r#enterer = Some(map_access.next_value()?);
                        }
                        "enteredDate" => {
                            let some = r#entered_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("enteredDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_enteredDate" => {
                            let some = r#entered_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_enteredDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "reason" => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        "service" => {
                            if r#service.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            r#service = Some(map_access.next_value()?);
                        }
                        "productReference" => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("productReference"));
                            }
                            r#product =
                                Some(ChargeItemProduct::Reference(map_access.next_value()?));
                        }
                        "productCodeableConcept" => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "productCodeableConcept",
                                ));
                            }
                            r#product =
                                Some(ChargeItemProduct::CodeableConcept(map_access.next_value()?));
                        }
                        "account" => {
                            if r#account.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            r#account = Some(map_access.next_value()?);
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "supportingInformation" => {
                            if r#supporting_information.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "supportingInformation",
                                ));
                            }
                            r#supporting_information = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "definition_uri",
                                    "definition_canonical",
                                    "status",
                                    "part_of",
                                    "code",
                                    "subject",
                                    "context",
                                    "occurrence",
                                    "performer",
                                    "performing_organization",
                                    "requesting_organization",
                                    "cost_center",
                                    "quantity",
                                    "bodysite",
                                    "factor_override",
                                    "price_override",
                                    "override_reason",
                                    "enterer",
                                    "entered_date",
                                    "reason",
                                    "service",
                                    "product",
                                    "account",
                                    "note",
                                    "supporting_information",
                                ],
                            ))
                        }
                    }
                }
                Ok(ChargeItem {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#definition_uri: r#definition_uri.unwrap_or(vec![]),
                    r#definition_canonical: r#definition_canonical.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
                    r#context,
                    r#occurrence,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#performing_organization,
                    r#requesting_organization,
                    r#cost_center,
                    r#quantity,
                    r#bodysite: r#bodysite.unwrap_or(vec![]),
                    r#factor_override,
                    r#price_override,
                    r#override_reason,
                    r#enterer,
                    r#entered_date,
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#service: r#service.unwrap_or(vec![]),
                    r#product,
                    r#account: r#account.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
