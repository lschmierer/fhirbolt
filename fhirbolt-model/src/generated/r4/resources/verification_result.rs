// Generated on 2023-04-05 by fhirbolt-codegen v0.1.0
#[doc = "Information about the primary source(s) involved in validation."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultPrimarySource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to the primary source."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Method for communicating with the primary source (manual; API; Push)."]
    pub r#communication_method: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the validation of the target against the primary source (successful; failed; unknown)."]
    pub r#validation_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the target was validated against the primary source."]
    pub r#validation_date: Option<super::super::types::DateTime>,
    #[doc = "Ability of the primary source to push updates/alerts (yes; no; undetermined)."]
    pub r#can_push_updates: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of alerts/updates the primary source can send (specific requested changes; any changes; as defined by source)."]
    pub r#push_type_available: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for VerificationResultPrimarySource {
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
            if let Some(some) = self.r#who.as_ref() {
                state.serialize_entry("who", some)?;
            }
            if !self.r#type.is_empty() {
                state.serialize_entry("type", &self.r#type)?;
            }
            if !self.r#communication_method.is_empty() {
                state.serialize_entry("communicationMethod", &self.r#communication_method)?;
            }
            if let Some(some) = self.r#validation_status.as_ref() {
                state.serialize_entry("validationStatus", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#validation_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("validationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_validationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#validation_date.as_ref() {
                    state.serialize_entry("validationDate", some)?;
                }
            }
            if let Some(some) = self.r#can_push_updates.as_ref() {
                state.serialize_entry("canPushUpdates", some)?;
            }
            if !self.r#push_type_available.is_empty() {
                state.serialize_entry("pushTypeAvailable", &self.r#push_type_available)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VerificationResultPrimarySource {
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
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "communicationMethod")]
            CommunicationMethod,
            #[serde(rename = "validationStatus")]
            ValidationStatus,
            #[serde(rename = "validationDate")]
            ValidationDate,
            #[serde(rename = "_validationDate")]
            ValidationDatePrimitiveElement,
            #[serde(rename = "canPushUpdates")]
            CanPushUpdates,
            #[serde(rename = "pushTypeAvailable")]
            PushTypeAvailable,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VerificationResultPrimarySource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VerificationResultPrimarySource")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VerificationResultPrimarySource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#communication_method: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#validation_status: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#validation_date: Option<super::super::types::DateTime> = None;
                let mut r#can_push_updates: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#push_type_available: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
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
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Who => {
                                if r#who.is_some() {
                                    return Err(serde::de::Error::duplicate_field("who"));
                                }
                                r#who = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if _ctx.from_json {
                                    if r#type.is_some() {
                                        return Err(serde::de::Error::duplicate_field("type"));
                                    }
                                    r#type = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#type.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::CommunicationMethod => {
                                if _ctx.from_json {
                                    if r#communication_method.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "communicationMethod",
                                        ));
                                    }
                                    r#communication_method = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#communication_method.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ValidationStatus => {
                                if r#validation_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validationStatus",
                                    ));
                                }
                                r#validation_status = Some(map_access.next_value()?);
                            }
                            Field::ValidationDate => {
                                if _ctx.from_json {
                                    let some = r#validation_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "validationDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#validation_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "validationDate",
                                        ));
                                    }
                                    r#validation_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::ValidationDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#validation_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_validationDate",
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
                                        "validationDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "who",
                                            "type",
                                            "communicationMethod",
                                            "validationStatus",
                                            "validationDate",
                                            "canPushUpdates",
                                            "pushTypeAvailable",
                                        ],
                                    ));
                                }
                            }
                            Field::CanPushUpdates => {
                                if r#can_push_updates.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "canPushUpdates",
                                    ));
                                }
                                r#can_push_updates = Some(map_access.next_value()?);
                            }
                            Field::PushTypeAvailable => {
                                if _ctx.from_json {
                                    if r#push_type_available.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "pushTypeAvailable",
                                        ));
                                    }
                                    r#push_type_available = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#push_type_available.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                        "who",
                                        "type",
                                        "communicationMethod",
                                        "validationStatus",
                                        "validationDate",
                                        "canPushUpdates",
                                        "pushTypeAvailable",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VerificationResultPrimarySource {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#who,
                        r#type: r#type.unwrap_or(vec![]),
                        r#communication_method: r#communication_method.unwrap_or(vec![]),
                        r#validation_status,
                        r#validation_date,
                        r#can_push_updates,
                        r#push_type_available: r#push_type_available.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about the entity attesting to information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultAttestation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The individual or organization attesting to information."]
    pub r#who: Option<Box<super::super::types::Reference>>,
    #[doc = "When the who is asserting on behalf of another (organization or individual)."]
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    #[doc = "The method by which attested information was submitted/retrieved (manual; API; Push)."]
    pub r#communication_method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date the information was attested to."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "A digital identity certificate associated with the attestation source."]
    pub r#source_identity_certificate: Option<super::super::types::String>,
    #[doc = "A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source."]
    pub r#proxy_identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the proxy entity indicating that they have the right to submit attested information on behalf of the attestation source."]
    pub r#proxy_signature: Option<Box<super::super::types::Signature>>,
    #[doc = "Signed assertion by the attestation source that they have attested to the information."]
    pub r#source_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultAttestation {
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
            if let Some(some) = self.r#who.as_ref() {
                state.serialize_entry("who", some)?;
            }
            if let Some(some) = self.r#on_behalf_of.as_ref() {
                state.serialize_entry("onBehalfOf", some)?;
            }
            if let Some(some) = self.r#communication_method.as_ref() {
                state.serialize_entry("communicationMethod", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#source_identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("sourceIdentityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sourceIdentityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#source_identity_certificate.as_ref() {
                    state.serialize_entry("sourceIdentityCertificate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#proxy_identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("proxyIdentityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_proxyIdentityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#proxy_identity_certificate.as_ref() {
                    state.serialize_entry("proxyIdentityCertificate", some)?;
                }
            }
            if let Some(some) = self.r#proxy_signature.as_ref() {
                state.serialize_entry("proxySignature", some)?;
            }
            if let Some(some) = self.r#source_signature.as_ref() {
                state.serialize_entry("sourceSignature", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VerificationResultAttestation {
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
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "onBehalfOf")]
            OnBehalfOf,
            #[serde(rename = "communicationMethod")]
            CommunicationMethod,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "sourceIdentityCertificate")]
            SourceIdentityCertificate,
            #[serde(rename = "_sourceIdentityCertificate")]
            SourceIdentityCertificatePrimitiveElement,
            #[serde(rename = "proxyIdentityCertificate")]
            ProxyIdentityCertificate,
            #[serde(rename = "_proxyIdentityCertificate")]
            ProxyIdentityCertificatePrimitiveElement,
            #[serde(rename = "proxySignature")]
            ProxySignature,
            #[serde(rename = "sourceSignature")]
            SourceSignature,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VerificationResultAttestation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VerificationResultAttestation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VerificationResultAttestation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#on_behalf_of: Option<Box<super::super::types::Reference>> = None;
                let mut r#communication_method: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#source_identity_certificate: Option<super::super::types::String> = None;
                let mut r#proxy_identity_certificate: Option<super::super::types::String> = None;
                let mut r#proxy_signature: Option<Box<super::super::types::Signature>> = None;
                let mut r#source_signature: Option<Box<super::super::types::Signature>> = None;
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
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Who => {
                                if r#who.is_some() {
                                    return Err(serde::de::Error::duplicate_field("who"));
                                }
                                r#who = Some(map_access.next_value()?);
                            }
                            Field::OnBehalfOf => {
                                if r#on_behalf_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onBehalfOf"));
                                }
                                r#on_behalf_of = Some(map_access.next_value()?);
                            }
                            Field::CommunicationMethod => {
                                if r#communication_method.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "communicationMethod",
                                    ));
                                }
                                r#communication_method = Some(map_access.next_value()?);
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
                                            "who",
                                            "onBehalfOf",
                                            "communicationMethod",
                                            "date",
                                            "sourceIdentityCertificate",
                                            "proxyIdentityCertificate",
                                            "proxySignature",
                                            "sourceSignature",
                                        ],
                                    ));
                                }
                            }
                            Field::SourceIdentityCertificate => {
                                if _ctx.from_json {
                                    let some = r#source_identity_certificate
                                        .get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "sourceIdentityCertificate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#source_identity_certificate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "sourceIdentityCertificate",
                                        ));
                                    }
                                    r#source_identity_certificate = Some(map_access.next_value()?);
                                }
                            }
                            Field::SourceIdentityCertificatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#source_identity_certificate
                                        .get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_sourceIdentityCertificate",
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
                                        "sourceIdentityCertificate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "who",
                                            "onBehalfOf",
                                            "communicationMethod",
                                            "date",
                                            "sourceIdentityCertificate",
                                            "proxyIdentityCertificate",
                                            "proxySignature",
                                            "sourceSignature",
                                        ],
                                    ));
                                }
                            }
                            Field::ProxyIdentityCertificate => {
                                if _ctx.from_json {
                                    let some = r#proxy_identity_certificate
                                        .get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "proxyIdentityCertificate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#proxy_identity_certificate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "proxyIdentityCertificate",
                                        ));
                                    }
                                    r#proxy_identity_certificate = Some(map_access.next_value()?);
                                }
                            }
                            Field::ProxyIdentityCertificatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#proxy_identity_certificate
                                        .get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_proxyIdentityCertificate",
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
                                        "proxyIdentityCertificate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "who",
                                            "onBehalfOf",
                                            "communicationMethod",
                                            "date",
                                            "sourceIdentityCertificate",
                                            "proxyIdentityCertificate",
                                            "proxySignature",
                                            "sourceSignature",
                                        ],
                                    ));
                                }
                            }
                            Field::ProxySignature => {
                                if r#proxy_signature.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "proxySignature",
                                    ));
                                }
                                r#proxy_signature = Some(map_access.next_value()?);
                            }
                            Field::SourceSignature => {
                                if r#source_signature.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sourceSignature",
                                    ));
                                }
                                r#source_signature = Some(map_access.next_value()?);
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
                                        "who",
                                        "onBehalfOf",
                                        "communicationMethod",
                                        "date",
                                        "sourceIdentityCertificate",
                                        "proxyIdentityCertificate",
                                        "proxySignature",
                                        "sourceSignature",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VerificationResultAttestation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#who,
                        r#on_behalf_of,
                        r#communication_method,
                        r#date,
                        r#source_identity_certificate,
                        r#proxy_identity_certificate,
                        r#proxy_signature,
                        r#source_signature,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about the entity validating information."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResultValidator {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to the organization validating information."]
    pub r#organization: Box<super::super::types::Reference>,
    #[doc = "A digital identity certificate associated with the validator."]
    pub r#identity_certificate: Option<super::super::types::String>,
    #[doc = "Signed assertion by the validator that they have validated the information."]
    pub r#attestation_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultValidator {
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
            state.serialize_entry("organization", &self.r#organization)?;
            if _ctx.output_json {
                if let Some(some) = self.r#identity_certificate.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("identityCertificate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_identityCertificate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#identity_certificate.as_ref() {
                    state.serialize_entry("identityCertificate", some)?;
                }
            }
            if let Some(some) = self.r#attestation_signature.as_ref() {
                state.serialize_entry("attestationSignature", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VerificationResultValidator {
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
            #[serde(rename = "organization")]
            Organization,
            #[serde(rename = "identityCertificate")]
            IdentityCertificate,
            #[serde(rename = "_identityCertificate")]
            IdentityCertificatePrimitiveElement,
            #[serde(rename = "attestationSignature")]
            AttestationSignature,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VerificationResultValidator;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VerificationResultValidator")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VerificationResultValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#identity_certificate: Option<super::super::types::String> = None;
                let mut r#attestation_signature: Option<Box<super::super::types::Signature>> = None;
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
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Organization => {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                r#organization = Some(map_access.next_value()?);
                            }
                            Field::IdentityCertificate => {
                                if _ctx.from_json {
                                    let some =
                                        r#identity_certificate.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identityCertificate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#identity_certificate.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "identityCertificate",
                                        ));
                                    }
                                    r#identity_certificate = Some(map_access.next_value()?);
                                }
                            }
                            Field::IdentityCertificatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#identity_certificate.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_identityCertificate",
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
                                        "identityCertificate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "organization",
                                            "identityCertificate",
                                            "attestationSignature",
                                        ],
                                    ));
                                }
                            }
                            Field::AttestationSignature => {
                                if r#attestation_signature.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "attestationSignature",
                                    ));
                                }
                                r#attestation_signature = Some(map_access.next_value()?);
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
                                        "organization",
                                        "identityCertificate",
                                        "attestationSignature",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VerificationResultValidator {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#organization: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#organization.unwrap_or(Default::default())
                        } else {
                            r#organization.ok_or(serde::de::Error::missing_field("organization"))?
                        },
                        r#identity_certificate,
                        r#attestation_signature,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Describes validation requirements, source(s), status and dates for one or more elements."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerificationResult {
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
    #[doc = "A resource that was validated."]
    pub r#target: Vec<Box<super::super::types::Reference>>,
    #[doc = "The fhirpath location(s) within the resource that was validated."]
    pub r#target_location: Vec<super::super::types::String>,
    #[doc = "The frequency with which the target must be validated (none; initial; periodic)."]
    pub r#need: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The validation status of the target (attested; validated; in process; requires revalidation; validation failed; revalidation failed)."]
    pub r#status: super::super::types::Code,
    #[doc = "When the validation status was updated."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "What the target is validated against (nothing; primary source; multiple sources)."]
    pub r#validation_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)."]
    pub r#validation_process: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Frequency of revalidation."]
    pub r#frequency: Option<Box<super::super::types::Timing>>,
    #[doc = "The date/time validation was last completed (including failed validations)."]
    pub r#last_performed: Option<super::super::types::DateTime>,
    #[doc = "The date when target is next validated, if appropriate."]
    pub r#next_scheduled: Option<super::super::types::Date>,
    #[doc = "The result if validation fails (fatal; warning; record only; none)."]
    pub r#failure_action: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the primary source(s) involved in validation."]
    pub r#primary_source: Vec<VerificationResultPrimarySource>,
    #[doc = "Information about the entity attesting to information."]
    pub r#attestation: Option<VerificationResultAttestation>,
    #[doc = "Information about the entity validating information."]
    pub r#validator: Vec<VerificationResultValidator>,
}
impl crate::AnyResource for VerificationResult {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
}
impl serde::ser::Serialize for VerificationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "VerificationResult")?;
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
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if _ctx.output_json {
                if !self.r#target_location.is_empty() {
                    let values = self
                        .r#target_location
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("targetLocation", &values)?;
                    }
                    let requires_elements = self
                        .r#target_location
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#target_location
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_targetLocation", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#target_location.is_empty() {
                    state.serialize_entry("targetLocation", &self.r#target_location)?;
                }
            }
            if let Some(some) = self.r#need.as_ref() {
                state.serialize_entry("need", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#status_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_date.as_ref() {
                    state.serialize_entry("statusDate", some)?;
                }
            }
            if let Some(some) = self.r#validation_type.as_ref() {
                state.serialize_entry("validationType", some)?;
            }
            if !self.r#validation_process.is_empty() {
                state.serialize_entry("validationProcess", &self.r#validation_process)?;
            }
            if let Some(some) = self.r#frequency.as_ref() {
                state.serialize_entry("frequency", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#last_performed.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("lastPerformed", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_lastPerformed", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#last_performed.as_ref() {
                    state.serialize_entry("lastPerformed", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#next_scheduled.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("nextScheduled", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_nextScheduled", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#next_scheduled.as_ref() {
                    state.serialize_entry("nextScheduled", some)?;
                }
            }
            if let Some(some) = self.r#failure_action.as_ref() {
                state.serialize_entry("failureAction", some)?;
            }
            if !self.r#primary_source.is_empty() {
                state.serialize_entry("primarySource", &self.r#primary_source)?;
            }
            if let Some(some) = self.r#attestation.as_ref() {
                state.serialize_entry("attestation", some)?;
            }
            if !self.r#validator.is_empty() {
                state.serialize_entry("validator", &self.r#validator)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VerificationResult {
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
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "targetLocation")]
            TargetLocation,
            #[serde(rename = "_targetLocation")]
            TargetLocationPrimitiveElement,
            #[serde(rename = "need")]
            Need,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "_statusDate")]
            StatusDatePrimitiveElement,
            #[serde(rename = "validationType")]
            ValidationType,
            #[serde(rename = "validationProcess")]
            ValidationProcess,
            #[serde(rename = "frequency")]
            Frequency,
            #[serde(rename = "lastPerformed")]
            LastPerformed,
            #[serde(rename = "_lastPerformed")]
            LastPerformedPrimitiveElement,
            #[serde(rename = "nextScheduled")]
            NextScheduled,
            #[serde(rename = "_nextScheduled")]
            NextScheduledPrimitiveElement,
            #[serde(rename = "failureAction")]
            FailureAction,
            #[serde(rename = "primarySource")]
            PrimarySource,
            #[serde(rename = "attestation")]
            Attestation,
            #[serde(rename = "validator")]
            Validator,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VerificationResult;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VerificationResult")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<VerificationResult, V::Error>
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
                let mut r#target: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#target_location: Option<Vec<super::super::types::String>> = None;
                let mut r#need: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_date: Option<super::super::types::DateTime> = None;
                let mut r#validation_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#validation_process: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#frequency: Option<Box<super::super::types::Timing>> = None;
                let mut r#last_performed: Option<super::super::types::DateTime> = None;
                let mut r#next_scheduled: Option<super::super::types::Date> = None;
                let mut r#failure_action: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#primary_source: Option<Vec<VerificationResultPrimarySource>> = None;
                let mut r#attestation: Option<VerificationResultAttestation> = None;
                let mut r#validator: Option<Vec<VerificationResultValidator>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "VerificationResult" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"VerificationResult",
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
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
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
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
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
                                if _ctx.from_json {
                                    if r#contained.is_some() {
                                        return Err(serde::de::Error::duplicate_field("contained"));
                                    }
                                    r#contained = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#contained.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Extension => {
                                if _ctx.from_json {
                                    if r#extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field("extension"));
                                    }
                                    r#extension = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::ModifierExtension => {
                                if _ctx.from_json {
                                    if r#modifier_extension.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "modifierExtension",
                                        ));
                                    }
                                    r#modifier_extension = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#modifier_extension.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Target => {
                                if _ctx.from_json {
                                    if r#target.is_some() {
                                        return Err(serde::de::Error::duplicate_field("target"));
                                    }
                                    r#target = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#target.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::TargetLocation => {
                                if _ctx.from_json {
                                    let values: Vec<Option<_>> = map_access.next_value()?;
                                    let vec = r#target_location.get_or_insert(
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
                                            "targetLocation",
                                        ));
                                    }
                                    for (i, value) in values.into_iter().enumerate() {
                                        if let Some(value) = value {
                                            vec[i].value = Some(value);
                                        }
                                    }
                                } else {
                                    let vec = r#target_location.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::TargetLocationPrimitiveElement => {
                                if _ctx.from_json {
                                    let elements: Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    > = map_access.next_value()?;
                                    let vec = r#target_location.get_or_insert(
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
                                            "_targetLocation",
                                        ));
                                    }
                                    for (i, element) in elements.into_iter().enumerate() {
                                        if let Some(element) = element {
                                            vec[i].id = element.id;
                                            vec[i].extension = element.extension;
                                        }
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "targetLocation",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
                                        ],
                                    ));
                                }
                            }
                            Field::Need => {
                                if r#need.is_some() {
                                    return Err(serde::de::Error::duplicate_field("need"));
                                }
                                r#need = Some(map_access.next_value()?);
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
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusDate => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    r#status_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_statusDate",
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
                                        "statusDate",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
                                        ],
                                    ));
                                }
                            }
                            Field::ValidationType => {
                                if r#validation_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "validationType",
                                    ));
                                }
                                r#validation_type = Some(map_access.next_value()?);
                            }
                            Field::ValidationProcess => {
                                if _ctx.from_json {
                                    if r#validation_process.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "validationProcess",
                                        ));
                                    }
                                    r#validation_process = Some(map_access.next_value()?);
                                } else {
                                    let vec =
                                        r#validation_process.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Frequency => {
                                if r#frequency.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frequency"));
                                }
                                r#frequency = Some(map_access.next_value()?);
                            }
                            Field::LastPerformed => {
                                if _ctx.from_json {
                                    let some = r#last_performed.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastPerformed",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#last_performed.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lastPerformed",
                                        ));
                                    }
                                    r#last_performed = Some(map_access.next_value()?);
                                }
                            }
                            Field::LastPerformedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#last_performed.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lastPerformed",
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
                                        "lastPerformed",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
                                        ],
                                    ));
                                }
                            }
                            Field::NextScheduled => {
                                if _ctx.from_json {
                                    let some = r#next_scheduled.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "nextScheduled",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#next_scheduled.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "nextScheduled",
                                        ));
                                    }
                                    r#next_scheduled = Some(map_access.next_value()?);
                                }
                            }
                            Field::NextScheduledPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#next_scheduled.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_nextScheduled",
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
                                        "nextScheduled",
                                        &[
                                            "id",
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "target",
                                            "targetLocation",
                                            "need",
                                            "status",
                                            "statusDate",
                                            "validationType",
                                            "validationProcess",
                                            "frequency",
                                            "lastPerformed",
                                            "nextScheduled",
                                            "failureAction",
                                            "primarySource",
                                            "attestation",
                                            "validator",
                                        ],
                                    ));
                                }
                            }
                            Field::FailureAction => {
                                if r#failure_action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("failureAction"));
                                }
                                r#failure_action = Some(map_access.next_value()?);
                            }
                            Field::PrimarySource => {
                                if _ctx.from_json {
                                    if r#primary_source.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "primarySource",
                                        ));
                                    }
                                    r#primary_source = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#primary_source.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
                            }
                            Field::Attestation => {
                                if r#attestation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("attestation"));
                                }
                                r#attestation = Some(map_access.next_value()?);
                            }
                            Field::Validator => {
                                if _ctx.from_json {
                                    if r#validator.is_some() {
                                        return Err(serde::de::Error::duplicate_field("validator"));
                                    }
                                    r#validator = Some(map_access.next_value()?);
                                } else {
                                    let vec = r#validator.get_or_insert(Default::default());
                                    vec.push(map_access.next_value()?);
                                }
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
                                        "target",
                                        "targetLocation",
                                        "need",
                                        "status",
                                        "statusDate",
                                        "validationType",
                                        "validationProcess",
                                        "frequency",
                                        "lastPerformed",
                                        "nextScheduled",
                                        "failureAction",
                                        "primarySource",
                                        "attestation",
                                        "validator",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VerificationResult {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#target: r#target.unwrap_or(vec![]),
                        r#target_location: r#target_location.unwrap_or(vec![]),
                        r#need,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_date,
                        r#validation_type,
                        r#validation_process: r#validation_process.unwrap_or(vec![]),
                        r#frequency,
                        r#last_performed,
                        r#next_scheduled,
                        r#failure_action,
                        r#primary_source: r#primary_source.unwrap_or(vec![]),
                        r#attestation,
                        r#validator: r#validator.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
