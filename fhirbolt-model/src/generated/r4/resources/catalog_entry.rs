// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Used for example, to point to a substance, or to a device used to administer a medication."]
#[derive(Default, Debug, Clone)]
pub struct CatalogEntryRelatedEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relation to the related item: child, parent, packageContent, containerPackage, usedIn, uses, requires, etc."]
    pub r#relationtype: super::super::types::Code,
    #[doc = "The reference to the related item."]
    pub r#item: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for CatalogEntryRelatedEntry {
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
                if let Some(some) = self.r#relationtype.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("relationtype", &some)?;
                }
                if self.r#relationtype.id.is_some() || !self.r#relationtype.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#relationtype.id.as_ref(),
                        extension: &self.r#relationtype.extension,
                    };
                    state.serialize_entry("_relationtype", &primitive_element)?;
                }
            } else {
                state.serialize_entry("relationtype", &self.r#relationtype)?;
            }
            state.serialize_entry("item", &self.r#item)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CatalogEntryRelatedEntry {
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
            #[serde(rename = "relationtype")]
            Relationtype,
            #[serde(rename = "_relationtype")]
            RelationtypePrimitiveElement,
            #[serde(rename = "item")]
            Item,
            Unknown(std::string::String),
        }
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
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Relationtype => {
                                if _ctx.from_json {
                                    let some = r#relationtype.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relationtype",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#relationtype.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "relationtype",
                                        ));
                                    }
                                    r#relationtype = Some(map_access.next_value()?);
                                }
                            }
                            Field::RelationtypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#relationtype.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_relationtype",
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
                                        "relationtype",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "relationtype",
                                            "item",
                                        ],
                                    ));
                                }
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
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
                                        "relationtype",
                                        "item",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CatalogEntryRelatedEntry {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationtype: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#relationtype.unwrap_or(Default::default())
                        } else {
                            r#relationtype.ok_or(serde::de::Error::missing_field("relationtype"))?
                        },
                        r#item: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#item.unwrap_or(Default::default())
                        } else {
                            r#item.ok_or(serde::de::Error::missing_field("item"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Catalog entries are wrappers that contextualize items included in a catalog."]
#[derive(Default, Debug, Clone)]
pub struct CatalogEntry {
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
    #[doc = "Used in supporting different identifiers for the same product, e.g. manufacturer code and retailer code."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The type of item - medication, device, service, protocol or other."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether the entry represents an orderable item."]
    pub r#orderable: super::super::types::Boolean,
    #[doc = "The item in a catalog or definition."]
    pub r#referenced_item: Box<super::super::types::Reference>,
    #[doc = "Used in supporting related concepts, e.g. NDC to RxNorm."]
    pub r#additional_identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Classes of devices, or ATC for medication."]
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to support catalog exchange even for unsupported products, e.g. getting list of medications even if not prescribable."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The time period in which this catalog entry is expected to be active."]
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date until which this catalog entry is expected to be active."]
    pub r#valid_to: Option<super::super::types::DateTime>,
    #[doc = "Typically date of issue is different from the beginning of the validity. This can be used to see when an item was last updated."]
    pub r#last_updated: Option<super::super::types::DateTime>,
    #[doc = "Used for examplefor Out of Formulary, or any specifics."]
    pub r#additional_characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "User for example for ATC classification, or."]
    pub r#additional_classification: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used for example, to point to a substance, or to a device used to administer a medication."]
    pub r#related_entry: Vec<CatalogEntryRelatedEntry>,
}
impl crate::AnyResource for CatalogEntry {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for CatalogEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CatalogEntry")?;
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
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#orderable.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("orderable", &some)?;
                }
                if self.r#orderable.id.is_some() || !self.r#orderable.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#orderable.id.as_ref(),
                        extension: &self.r#orderable.extension,
                    };
                    state.serialize_entry("_orderable", &primitive_element)?;
                }
            } else {
                state.serialize_entry("orderable", &self.r#orderable)?;
            }
            state.serialize_entry("referencedItem", &self.r#referenced_item)?;
            if !self.r#additional_identifier.is_empty() {
                state.serialize_entry("additionalIdentifier", &self.r#additional_identifier)?;
            }
            if !self.r#classification.is_empty() {
                state.serialize_entry("classification", &self.r#classification)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("status", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_status", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status.as_ref() {
                    state.serialize_entry("status", some)?;
                }
            }
            if let Some(some) = self.r#validity_period.as_ref() {
                state.serialize_entry("validityPeriod", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#valid_to.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("validTo", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_validTo", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#valid_to.as_ref() {
                    state.serialize_entry("validTo", some)?;
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CatalogEntry {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "orderable")]
            Orderable,
            #[serde(rename = "_orderable")]
            OrderablePrimitiveElement,
            #[serde(rename = "referencedItem")]
            ReferencedItem,
            #[serde(rename = "additionalIdentifier")]
            AdditionalIdentifier,
            #[serde(rename = "classification")]
            Classification,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "validityPeriod")]
            ValidityPeriod,
            #[serde(rename = "validTo")]
            ValidTo,
            #[serde(rename = "_validTo")]
            ValidToPrimitiveElement,
            #[serde(rename = "lastUpdated")]
            LastUpdated,
            #[serde(rename = "_lastUpdated")]
            LastUpdatedPrimitiveElement,
            #[serde(rename = "additionalCharacteristic")]
            AdditionalCharacteristic,
            #[serde(rename = "additionalClassification")]
            AdditionalClassification,
            #[serde(rename = "relatedEntry")]
            RelatedEntry,
            Unknown(std::string::String),
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CatalogEntry" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CatalogEntry",
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
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
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
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Orderable => {
                                if _ctx.from_json {
                                    let some = r#orderable.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("orderable"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#orderable.is_some() {
                                        return Err(serde::de::Error::duplicate_field("orderable"));
                                    }
                                    r#orderable = Some(map_access.next_value()?);
                                }
                            }
                            Field::OrderablePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#orderable.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_orderable",
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
                                        "orderable",
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
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
                                        ],
                                    ));
                                }
                            }
                            Field::ReferencedItem => {
                                if r#referenced_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referencedItem",
                                    ));
                                }
                                r#referenced_item = Some(map_access.next_value()?);
                            }
                            Field::AdditionalIdentifier => {
                                if r#additional_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalIdentifier",
                                    ));
                                }
                                r#additional_identifier = Some(map_access.next_value()?);
                            }
                            Field::Classification => {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some(map_access.next_value()?);
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
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
                                        ],
                                    ));
                                }
                            }
                            Field::ValidityPeriod => {
                                if r#validity_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validityPeriod",
                                    ));
                                }
                                r#validity_period = Some(map_access.next_value()?);
                            }
                            Field::ValidTo => {
                                if _ctx.from_json {
                                    let some = r#valid_to.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("validTo"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#valid_to.is_some() {
                                        return Err(serde::de::Error::duplicate_field("validTo"));
                                    }
                                    r#valid_to = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValidToPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "validTo",
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
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "type",
                                            "orderable",
                                            "referencedItem",
                                            "additionalIdentifier",
                                            "classification",
                                            "status",
                                            "validityPeriod",
                                            "validTo",
                                            "lastUpdated",
                                            "additionalCharacteristic",
                                            "additionalClassification",
                                            "relatedEntry",
                                        ],
                                    ));
                                }
                            }
                            Field::AdditionalCharacteristic => {
                                if r#additional_characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalCharacteristic",
                                    ));
                                }
                                r#additional_characteristic = Some(map_access.next_value()?);
                            }
                            Field::AdditionalClassification => {
                                if r#additional_classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalClassification",
                                    ));
                                }
                                r#additional_classification = Some(map_access.next_value()?);
                            }
                            Field::RelatedEntry => {
                                if r#related_entry.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedEntry"));
                                }
                                r#related_entry = Some(map_access.next_value()?);
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
                                        "type",
                                        "orderable",
                                        "referencedItem",
                                        "additionalIdentifier",
                                        "classification",
                                        "status",
                                        "validityPeriod",
                                        "validTo",
                                        "lastUpdated",
                                        "additionalCharacteristic",
                                        "additionalClassification",
                                        "relatedEntry",
                                    ],
                                ));
                            },
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
                        r#orderable: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#orderable.unwrap_or(Default::default())
                        } else {
                            r#orderable.ok_or(serde::de::Error::missing_field("orderable"))?
                        },
                        r#referenced_item: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#referenced_item.unwrap_or(Default::default())
                        } else {
                            r#referenced_item
                                .ok_or(serde::de::Error::missing_field("referencedItem"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
