// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Distribution of the payment amount for a previously acknowledged payable."]
#[derive(Default, Debug, Clone)]
pub struct PaymentReconciliationDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Unique identifier for the current payment item for the referenced payable."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Unique identifier for the prior payment item for the referenced payable."]
    pub r#predecessor: Option<Box<super::super::types::Identifier>>,
    #[doc = "Code to indicate the nature of the payment."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A resource, such as a Claim, the evaluation of which could lead to payment."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The party which submitted the claim or financial transaction."]
    pub r#submitter: Option<Box<super::super::types::Reference>>,
    #[doc = "A resource, such as a ClaimResponse, which contains a commitment to payment."]
    pub r#response: Option<Box<super::super::types::Reference>>,
    #[doc = "The date from the response resource containing a commitment to pay."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "A reference to the individual who is responsible for inquiries regarding the response and its payment."]
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    #[doc = "The party which is receiving the payment."]
    pub r#payee: Option<Box<super::super::types::Reference>>,
    #[doc = "The monetary amount allocated from the total payment to the payable."]
    pub r#amount: Option<Box<super::super::types::Money>>,
}
impl serde::ser::Serialize for PaymentReconciliationDetail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#identifier.as_ref() {
                state.serialize_entry("identifier", some)?;
            }
            if let Some(some) = self.r#predecessor.as_ref() {
                state.serialize_entry("predecessor", some)?;
            }
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#request.as_ref() {
                state.serialize_entry("request", some)?;
            }
            if let Some(some) = self.r#submitter.as_ref() {
                state.serialize_entry("submitter", some)?;
            }
            if let Some(some) = self.r#response.as_ref() {
                state.serialize_entry("response", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("date", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_date", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date.as_ref() {
                    state.serialize_entry("date", some)?;
                }
            }
            if let Some(some) = self.r#responsible.as_ref() {
                state.serialize_entry("responsible", some)?;
            }
            if let Some(some) = self.r#payee.as_ref() {
                state.serialize_entry("payee", some)?;
            }
            if let Some(some) = self.r#amount.as_ref() {
                state.serialize_entry("amount", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PaymentReconciliationDetail {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "predecessor")]
            Predecessor,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "submitter")]
            Submitter,
            #[serde(rename = "response")]
            Response,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "responsible")]
            Responsible,
            #[serde(rename = "payee")]
            Payee,
            #[serde(rename = "amount")]
            Amount,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PaymentReconciliationDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliationDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PaymentReconciliationDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#predecessor: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#submitter: Option<Box<super::super::types::Reference>> = None;
                let mut r#response: Option<Box<super::super::types::Reference>> = None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#responsible: Option<Box<super::super::types::Reference>> = None;
                let mut r#payee: Option<Box<super::super::types::Reference>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Predecessor => {
                                if r#predecessor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("predecessor"));
                                }
                                r#predecessor = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Request => {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                r#request = Some(map_access.next_value()?);
                            }
                            Field::Submitter => {
                                if r#submitter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("submitter"));
                                }
                                r#submitter = Some(map_access.next_value()?);
                            }
                            Field::Response => {
                                if r#response.is_some() {
                                    return Err(serde::de::Error::duplicate_field("response"));
                                }
                                r#response = Some(map_access.next_value()?);
                            }
                            Field::Date => {
                                if _ctx.from_json {
                                    let some = r#date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date.is_some() {
                                        return Err(serde::de::Error::duplicate_field("date"));
                                    }
                                    r#date = Some(map_access.next_value()?);
                                }
                            }
                            Field::DatePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "date",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "predecessor",
                                            "type",
                                            "request",
                                            "submitter",
                                            "response",
                                            "date",
                                            "responsible",
                                            "payee",
                                            "amount",
                                        ],
                                    ));
                                }
                            }
                            Field::Responsible => {
                                if r#responsible.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                r#responsible = Some(map_access.next_value()?);
                            }
                            Field::Payee => {
                                if r#payee.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payee"));
                                }
                                r#payee = Some(map_access.next_value()?);
                            }
                            Field::Amount => {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                r#amount = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "predecessor",
                                        "type",
                                        "request",
                                        "submitter",
                                        "response",
                                        "date",
                                        "responsible",
                                        "payee",
                                        "amount",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PaymentReconciliationDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#predecessor,
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#request,
                        r#submitter,
                        r#response,
                        r#date,
                        r#responsible,
                        r#payee,
                        r#amount,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A note that describes or explains the processing in a human readable form."]
#[derive(Default, Debug, Clone)]
pub struct PaymentReconciliationProcessNote {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The business purpose of the note text."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The explanation or description associated with the processing."]
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for PaymentReconciliationProcessNote {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PaymentReconciliationProcessNote {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PaymentReconciliationProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliationProcessNote")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PaymentReconciliationProcessNote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#text: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Type => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                }
                            }
                            Field::TypePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#type.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_type"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "type",
                                        &["id", "extension", "modifierExtension", "type", "text"],
                                    ));
                                }
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &["id", "extension", "modifierExtension", "type", "text"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "text"],
                                ));
                            },
                        }
                    }
                    Ok(PaymentReconciliationProcessNote {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#text,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This resource provides the details including amount of a payment and allocates the payment items being paid."]
#[derive(Default, Debug, Clone)]
pub struct PaymentReconciliation {
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
    #[doc = "A unique identifier assigned to this payment reconciliation."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The period of time for which payments have been gathered into this bulk payment for settlement."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The date when the resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "The party who generated the payment."]
    pub r#payment_issuer: Option<Box<super::super::types::Reference>>,
    #[doc = "Original request resource reference."]
    pub r#request: Option<Box<super::super::types::Reference>>,
    #[doc = "The practitioner who is responsible for the services rendered to the patient."]
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    #[doc = "The outcome of a request for a reconciliation."]
    pub r#outcome: Option<super::super::types::Code>,
    #[doc = "A human readable description of the status of the request for the reconciliation."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "The date of payment as indicated on the financial instrument."]
    pub r#payment_date: super::super::types::Date,
    #[doc = "Total payment amount as indicated on the financial instrument."]
    pub r#payment_amount: Box<super::super::types::Money>,
    #[doc = "Issuer's unique identifier for the payment instrument."]
    pub r#payment_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Distribution of the payment amount for a previously acknowledged payable."]
    pub r#detail: Vec<PaymentReconciliationDetail>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A note that describes or explains the processing in a human readable form."]
    pub r#process_note: Vec<PaymentReconciliationProcessNote>,
}
impl crate::AnyResource for PaymentReconciliation {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for PaymentReconciliation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "PaymentReconciliation")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("created", &some)?;
                }
                if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#created.id.as_ref(),
                        extension: &self.r#created.extension,
                    };
                    state.serialize_entry("_created", &primitive_element)?;
                }
            } else {
                state.serialize_entry("created", &self.r#created)?;
            }
            if let Some(some) = self.r#payment_issuer.as_ref() {
                state.serialize_entry("paymentIssuer", some)?;
            }
            if let Some(some) = self.r#request.as_ref() {
                state.serialize_entry("request", some)?;
            }
            if let Some(some) = self.r#requestor.as_ref() {
                state.serialize_entry("requestor", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#outcome.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("outcome", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_outcome", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#outcome.as_ref() {
                    state.serialize_entry("outcome", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#disposition.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("disposition", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_disposition", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#disposition.as_ref() {
                    state.serialize_entry("disposition", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#payment_date.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("paymentDate", &some)?;
                }
                if self.r#payment_date.id.is_some() || !self.r#payment_date.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#payment_date.id.as_ref(),
                        extension: &self.r#payment_date.extension,
                    };
                    state.serialize_entry("_paymentDate", &primitive_element)?;
                }
            } else {
                state.serialize_entry("paymentDate", &self.r#payment_date)?;
            }
            state.serialize_entry("paymentAmount", &self.r#payment_amount)?;
            if let Some(some) = self.r#payment_identifier.as_ref() {
                state.serialize_entry("paymentIdentifier", some)?;
            }
            if !self.r#detail.is_empty() {
                state.serialize_entry("detail", &self.r#detail)?;
            }
            if let Some(some) = self.r#form_code.as_ref() {
                state.serialize_entry("formCode", some)?;
            }
            if !self.r#process_note.is_empty() {
                state.serialize_entry("processNote", &self.r#process_note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for PaymentReconciliation {
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
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "paymentIssuer")]
            PaymentIssuer,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "requestor")]
            Requestor,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "disposition")]
            Disposition,
            #[serde(rename = "_disposition")]
            DispositionPrimitiveElement,
            #[serde(rename = "paymentDate")]
            PaymentDate,
            #[serde(rename = "_paymentDate")]
            PaymentDatePrimitiveElement,
            #[serde(rename = "paymentAmount")]
            PaymentAmount,
            #[serde(rename = "paymentIdentifier")]
            PaymentIdentifier,
            #[serde(rename = "detail")]
            Detail,
            #[serde(rename = "formCode")]
            FormCode,
            #[serde(rename = "processNote")]
            ProcessNote,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PaymentReconciliation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentReconciliation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PaymentReconciliation, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#payment_issuer: Option<Box<super::super::types::Reference>> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#requestor: Option<Box<super::super::types::Reference>> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#disposition: Option<super::super::types::String> = None;
                let mut r#payment_date: Option<super::super::types::Date> = None;
                let mut r#payment_amount: Option<Box<super::super::types::Money>> = None;
                let mut r#payment_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#detail: Option<Vec<PaymentReconciliationDetail>> = None;
                let mut r#form_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#process_note: Option<Vec<PaymentReconciliationProcessNote>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "PaymentReconciliation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"PaymentReconciliation",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
                                        ],
                                    ));
                                }
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Created => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#created.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    r#created = Some(map_access.next_value()?);
                                }
                            }
                            Field::CreatedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_created"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "created",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
                                        ],
                                    ));
                                }
                            }
                            Field::PaymentIssuer => {
                                if r#payment_issuer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("paymentIssuer"));
                                }
                                r#payment_issuer = Some(map_access.next_value()?);
                            }
                            Field::Request => {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                r#request = Some(map_access.next_value()?);
                            }
                            Field::Requestor => {
                                if r#requestor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestor"));
                                }
                                r#requestor = Some(map_access.next_value()?);
                            }
                            Field::Outcome => {
                                if _ctx.from_json {
                                    let some = r#outcome.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("outcome"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#outcome.is_some() {
                                        return Err(serde::de::Error::duplicate_field("outcome"));
                                    }
                                    r#outcome = Some(map_access.next_value()?);
                                }
                            }
                            Field::OutcomePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#outcome.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_outcome"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "outcome",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
                                        ],
                                    ));
                                }
                            }
                            Field::Disposition => {
                                if _ctx.from_json {
                                    let some = r#disposition.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "disposition",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#disposition.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "disposition",
                                        ));
                                    }
                                    r#disposition = Some(map_access.next_value()?);
                                }
                            }
                            Field::DispositionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#disposition.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_disposition",
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
                                        "disposition",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
                                        ],
                                    ));
                                }
                            }
                            Field::PaymentDate => {
                                if _ctx.from_json {
                                    let some = r#payment_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "paymentDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#payment_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "paymentDate",
                                        ));
                                    }
                                    r#payment_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::PaymentDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#payment_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_paymentDate",
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
                                        "paymentDate",
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
                                            "status",
                                            "period",
                                            "created",
                                            "paymentIssuer",
                                            "request",
                                            "requestor",
                                            "outcome",
                                            "disposition",
                                            "paymentDate",
                                            "paymentAmount",
                                            "paymentIdentifier",
                                            "detail",
                                            "formCode",
                                            "processNote",
                                        ],
                                    ));
                                }
                            }
                            Field::PaymentAmount => {
                                if r#payment_amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("paymentAmount"));
                                }
                                r#payment_amount = Some(map_access.next_value()?);
                            }
                            Field::PaymentIdentifier => {
                                if r#payment_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "paymentIdentifier",
                                    ));
                                }
                                r#payment_identifier = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
                            }
                            Field::FormCode => {
                                if r#form_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("formCode"));
                                }
                                r#form_code = Some(map_access.next_value()?);
                            }
                            Field::ProcessNote => {
                                if r#process_note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("processNote"));
                                }
                                r#process_note = Some(map_access.next_value()?);
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
                                        "status",
                                        "period",
                                        "created",
                                        "paymentIssuer",
                                        "request",
                                        "requestor",
                                        "outcome",
                                        "disposition",
                                        "paymentDate",
                                        "paymentAmount",
                                        "paymentIdentifier",
                                        "detail",
                                        "formCode",
                                        "processNote",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PaymentReconciliation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#period,
                        r#created: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#payment_issuer,
                        r#request,
                        r#requestor,
                        r#outcome,
                        r#disposition,
                        r#payment_date: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#payment_date.unwrap_or(Default::default())
                        } else {
                            r#payment_date.ok_or(serde::de::Error::missing_field("paymentDate"))?
                        },
                        r#payment_amount: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#payment_amount.unwrap_or(Default::default())
                        } else {
                            r#payment_amount
                                .ok_or(serde::de::Error::missing_field("paymentAmount"))?
                        },
                        r#payment_identifier,
                        r#detail: r#detail.unwrap_or(vec![]),
                        r#form_code,
                        r#process_note: r#process_note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
