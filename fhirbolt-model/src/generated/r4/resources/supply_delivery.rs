// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Identifies the medication, substance or device being dispensed. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
#[derive(Debug, Clone)]
pub enum SupplyDeliverySuppliedItemItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SupplyDeliverySuppliedItemItem {
    fn default() -> SupplyDeliverySuppliedItemItem {
        SupplyDeliverySuppliedItemItem::Invalid
    }
}
#[doc = "The date or time(s) the activity occurred."]
#[derive(Debug, Clone)]
pub enum SupplyDeliveryOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for SupplyDeliveryOccurrence {
    fn default() -> SupplyDeliveryOccurrence {
        SupplyDeliveryOccurrence::Invalid
    }
}
#[doc = "The item that is being delivered or has been supplied."]
#[derive(Default, Debug, Clone)]
pub struct SupplyDeliverySuppliedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The amount of supply that has been dispensed. Includes unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the medication, substance or device being dispensed. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
    pub r#item: Option<SupplyDeliverySuppliedItemItem>,
}
impl serde::ser::Serialize for SupplyDeliverySuppliedItem {
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
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if let Some(some) = self.r#item.as_ref() {
                match some {
                    SupplyDeliverySuppliedItemItem::CodeableConcept(ref value) => {
                        state.serialize_entry("itemCodeableConcept", value)?;
                    }
                    SupplyDeliverySuppliedItemItem::Reference(ref value) => {
                        state.serialize_entry("itemReference", value)?;
                    }
                    SupplyDeliverySuppliedItemItem::Invalid => {
                        return Err(serde::ser::Error::custom("item is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SupplyDeliverySuppliedItem {
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
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "itemCodeableConcept")]
            ItemCodeableConcept,
            #[serde(rename = "itemReference")]
            ItemReference,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SupplyDeliverySuppliedItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyDeliverySuppliedItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SupplyDeliverySuppliedItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#item: Option<SupplyDeliverySuppliedItemItem> = None;
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
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::ItemCodeableConcept => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "itemCodeableConcept",
                                    ));
                                }
                                r#item = Some(SupplyDeliverySuppliedItemItem::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ItemReference => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("itemReference"));
                                }
                                r#item = Some(SupplyDeliverySuppliedItemItem::Reference(
                                    map_access.next_value()?,
                                ));
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
                                        "quantity",
                                        "itemCodeableConcept",
                                        "itemReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SupplyDeliverySuppliedItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#quantity,
                        r#item,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Record of delivery of what is supplied."]
#[derive(Default, Debug, Clone)]
pub struct SupplyDelivery {
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
    #[doc = "Identifier for the supply delivery event that is used to identify it across multiple disparate systems."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code specifying the state of the dispense event."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "A link to a resource representing the person whom the delivered item is for."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of dispensing event that is performed. Examples include: Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The item that is being delivered or has been supplied."]
    pub r#supplied_item: Option<SupplyDeliverySuppliedItem>,
    #[doc = "The date or time(s) the activity occurred."]
    pub r#occurrence: Option<SupplyDeliveryOccurrence>,
    #[doc = "The individual responsible for dispensing the medication, supplier or device."]
    pub r#supplier: Option<Box<super::super::types::Reference>>,
    #[doc = "Identification of the facility/location where the Supply was shipped to, as part of the dispense event."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the person who picked up the Supply."]
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for SupplyDelivery {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SupplyDelivery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SupplyDelivery")?;
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
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
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
            if let Some(some) = self.r#patient.as_ref() {
                state.serialize_entry("patient", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#supplied_item.as_ref() {
                state.serialize_entry("suppliedItem", some)?;
            }
            if let Some(some) = self.r#occurrence.as_ref() {
                match some {
                    SupplyDeliveryOccurrence::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("occurrenceDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("occurrenceDateTime", value)?;
                        }
                    }
                    SupplyDeliveryOccurrence::Period(ref value) => {
                        state.serialize_entry("occurrencePeriod", value)?;
                    }
                    SupplyDeliveryOccurrence::Timing(ref value) => {
                        state.serialize_entry("occurrenceTiming", value)?;
                    }
                    SupplyDeliveryOccurrence::Invalid => {
                        return Err(serde::ser::Error::custom("occurrence is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#supplier.as_ref() {
                state.serialize_entry("supplier", some)?;
            }
            if let Some(some) = self.r#destination.as_ref() {
                state.serialize_entry("destination", some)?;
            }
            if !self.r#receiver.is_empty() {
                state.serialize_entry("receiver", &self.r#receiver)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SupplyDelivery {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "suppliedItem")]
            SuppliedItem,
            #[serde(rename = "occurrenceDateTime")]
            OccurrenceDateTime,
            #[serde(rename = "_occurrenceDateTime")]
            OccurrenceDateTimePrimitiveElement,
            #[serde(rename = "occurrencePeriod")]
            OccurrencePeriod,
            #[serde(rename = "occurrenceTiming")]
            OccurrenceTiming,
            #[serde(rename = "supplier")]
            Supplier,
            #[serde(rename = "destination")]
            Destination,
            #[serde(rename = "receiver")]
            Receiver,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SupplyDelivery;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SupplyDelivery")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SupplyDelivery, V::Error>
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
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#supplied_item: Option<SupplyDeliverySuppliedItem> = None;
                let mut r#occurrence: Option<SupplyDeliveryOccurrence> = None;
                let mut r#supplier: Option<Box<super::super::types::Reference>> = None;
                let mut r#destination: Option<Box<super::super::types::Reference>> = None;
                let mut r#receiver: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SupplyDelivery" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SupplyDelivery",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "patient",
                                            "type",
                                            "suppliedItem",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "supplier",
                                            "destination",
                                            "receiver",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "patient",
                                            "type",
                                            "suppliedItem",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "supplier",
                                            "destination",
                                            "receiver",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "patient",
                                            "type",
                                            "suppliedItem",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "supplier",
                                            "destination",
                                            "receiver",
                                        ],
                                    ));
                                }
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::SuppliedItem => {
                                if r#supplied_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("suppliedItem"));
                                }
                                r#supplied_item = Some(map_access.next_value()?);
                            }
                            Field::OccurrenceDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        SupplyDeliveryOccurrence::DateTime(Default::default()),
                                    );
                                    if let SupplyDeliveryOccurrence::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "occurrenceDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrence[x]",
                                        ));
                                    }
                                } else {
                                    if r#occurrence.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurrenceDateTime",
                                        ));
                                    }
                                    r#occurrence = Some(SupplyDeliveryOccurrence::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::OccurrenceDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#occurrence.get_or_insert(
                                        SupplyDeliveryOccurrence::DateTime(Default::default()),
                                    );
                                    if let SupplyDeliveryOccurrence::DateTime(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "_occurrence[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "occurrenceDateTime",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "patient",
                                            "type",
                                            "suppliedItem",
                                            "occurrenceDateTime",
                                            "occurrencePeriod",
                                            "occurrenceTiming",
                                            "supplier",
                                            "destination",
                                            "receiver",
                                        ],
                                    ));
                                }
                            }
                            Field::OccurrencePeriod => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrencePeriod",
                                    ));
                                }
                                r#occurrence = Some(SupplyDeliveryOccurrence::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::OccurrenceTiming => {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceTiming",
                                    ));
                                }
                                r#occurrence = Some(SupplyDeliveryOccurrence::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Supplier => {
                                if r#supplier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("supplier"));
                                }
                                r#supplier = Some(map_access.next_value()?);
                            }
                            Field::Destination => {
                                if r#destination.is_some() {
                                    return Err(serde::de::Error::duplicate_field("destination"));
                                }
                                r#destination = Some(map_access.next_value()?);
                            }
                            Field::Receiver => {
                                if r#receiver.is_some() {
                                    return Err(serde::de::Error::duplicate_field("receiver"));
                                }
                                r#receiver = Some(map_access.next_value()?);
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
                                        "basedOn",
                                        "partOf",
                                        "status",
                                        "patient",
                                        "type",
                                        "suppliedItem",
                                        "occurrenceDateTime",
                                        "occurrencePeriod",
                                        "occurrenceTiming",
                                        "supplier",
                                        "destination",
                                        "receiver",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SupplyDelivery {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#status,
                        r#patient,
                        r#type,
                        r#supplied_item,
                        r#occurrence,
                        r#supplier,
                        r#destination,
                        r#receiver: r#receiver.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
