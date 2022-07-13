// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct CatalogEntryRelatedEntry {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#relationtype: super::super::types::Code,
    pub r#item: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for CatalogEntryRelatedEntry {
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
        if let Some(some) = self.r#relationtype.value.as_ref() {
            state.serialize_entry("relationtype", some)?;
        }
        if self.r#relationtype.id.is_some() || !self.r#relationtype.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#relationtype.id,
                extension: &self.r#relationtype.extension,
            };
            state.serialize_entry("_relationtype", &primitive_element)?;
        }
        state.serialize_entry("item", &self.r#item)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CatalogEntryRelatedEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CatalogEntryRelatedEntry;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CatalogEntryRelatedEntry")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CatalogEntryRelatedEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relationtype: Option<super::super::types::Code> = None;
                let mut r#item: Option<Box<super::super::types::Reference>> = None;
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
                        "relationtype" => {
                            let some = r#relationtype.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationtype"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_relationtype" => {
                            let some = r#relationtype.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_relationtype"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "item" => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            r#item = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "relationtype",
                                    "item",
                                ],
                            ))
                        }
                    }
                }
                Ok(CatalogEntryRelatedEntry {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#relationtype: r#relationtype
                        .ok_or(serde::de::Error::missing_field("relationtype"))?,
                    r#item: r#item.ok_or(serde::de::Error::missing_field("item"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct CatalogEntry {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#orderable: super::super::types::Boolean,
    pub r#referenced_item: Box<super::super::types::Reference>,
    pub r#additional_identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#valid_to: Option<super::super::types::DateTime>,
    pub r#last_updated: Option<super::super::types::DateTime>,
    pub r#additional_characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#additional_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#related_entry: Vec<CatalogEntryRelatedEntry>,
}
impl serde::ser::Serialize for CatalogEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "CatalogEntry")?;
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
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#orderable.value.as_ref() {
            state.serialize_entry("orderable", some)?;
        }
        if self.r#orderable.id.is_some() || !self.r#orderable.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#orderable.id,
                extension: &self.r#orderable.extension,
            };
            state.serialize_entry("_orderable", &primitive_element)?;
        }
        state.serialize_entry("referencedItem", &self.r#referenced_item)?;
        if !self.r#additional_identifier.is_empty() {
            state.serialize_entry("additionalIdentifier", &self.r#additional_identifier)?;
        }
        if !self.r#classification.is_empty() {
            state.serialize_entry("classification", &self.r#classification)?;
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#validity_period.as_ref() {
            state.serialize_entry("validityPeriod", some)?;
        }
        if let Some(some) = self.r#valid_to.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("validTo", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_validTo", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_updated.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastUpdated", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastUpdated", &primitive_element)?;
            }
        }
        if !self.r#additional_characteristic.is_empty() {
            state.serialize_entry(
                "additionalCharacteristic",
                &self.r#additional_characteristic,
            )?;
        }
        if !self.r#additional_classification.is_empty() {
            state.serialize_entry(
                "additionalClassification",
                &self.r#additional_classification,
            )?;
        }
        if !self.r#related_entry.is_empty() {
            state.serialize_entry("relatedEntry", &self.r#related_entry)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CatalogEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CatalogEntry;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CatalogEntry")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CatalogEntry, V::Error>
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
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#orderable: Option<super::super::types::Boolean> = None;
                let mut r#referenced_item: Option<Box<super::super::types::Reference>> = None;
                let mut r#additional_identifier: Option<Vec<Box<super::super::types::Identifier>>> =
                    None;
                let mut r#classification: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#validity_period: Option<Box<super::super::types::Period>> = None;
                let mut r#valid_to: Option<super::super::types::DateTime> = None;
                let mut r#last_updated: Option<super::super::types::DateTime> = None;
                let mut r#additional_characteristic: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#additional_classification: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#related_entry: Option<Vec<CatalogEntryRelatedEntry>> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "orderable" => {
                            let some = r#orderable.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderable"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_orderable" => {
                            let some = r#orderable.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_orderable"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "referencedItem" => {
                            if r#referenced_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("referencedItem"));
                            }
                            r#referenced_item = Some(map_access.next_value()?);
                        }
                        "additionalIdentifier" => {
                            if r#additional_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalIdentifier",
                                ));
                            }
                            r#additional_identifier = Some(map_access.next_value()?);
                        }
                        "classification" => {
                            if r#classification.is_some() {
                                return Err(serde::de::Error::duplicate_field("classification"));
                            }
                            r#classification = Some(map_access.next_value()?);
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
                        "validityPeriod" => {
                            if r#validity_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("validityPeriod"));
                            }
                            r#validity_period = Some(map_access.next_value()?);
                        }
                        "validTo" => {
                            let some = r#valid_to.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("validTo"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_validTo" => {
                            let some = r#valid_to.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_validTo"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "lastUpdated" => {
                            let some = r#last_updated.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_lastUpdated" => {
                            let some = r#last_updated.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastUpdated"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "additionalCharacteristic" => {
                            if r#additional_characteristic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalCharacteristic",
                                ));
                            }
                            r#additional_characteristic = Some(map_access.next_value()?);
                        }
                        "additionalClassification" => {
                            if r#additional_classification.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalClassification",
                                ));
                            }
                            r#additional_classification = Some(map_access.next_value()?);
                        }
                        "relatedEntry" => {
                            if r#related_entry.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedEntry"));
                            }
                            r#related_entry = Some(map_access.next_value()?);
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
                                    "type",
                                    "orderable",
                                    "referenced_item",
                                    "additional_identifier",
                                    "classification",
                                    "status",
                                    "validity_period",
                                    "valid_to",
                                    "last_updated",
                                    "additional_characteristic",
                                    "additional_classification",
                                    "related_entry",
                                ],
                            ))
                        }
                    }
                }
                Ok(CatalogEntry {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type,
                    r#orderable: r#orderable.ok_or(serde::de::Error::missing_field("orderable"))?,
                    r#referenced_item: r#referenced_item
                        .ok_or(serde::de::Error::missing_field("referenced_item"))?,
                    r#additional_identifier: r#additional_identifier.unwrap_or(vec![]),
                    r#classification: r#classification.unwrap_or(vec![]),
                    r#status,
                    r#validity_period,
                    r#valid_to,
                    r#last_updated,
                    r#additional_characteristic: r#additional_characteristic.unwrap_or(vec![]),
                    r#additional_classification: r#additional_classification.unwrap_or(vec![]),
                    r#related_entry: r#related_entry.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
