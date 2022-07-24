// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct VerificationResultPrimarySource {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#communication_method: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#validation_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#validation_date: Option<super::super::types::DateTime>,
    pub r#can_push_updates: Option<Box<super::super::types::CodeableConcept>>,
    pub r#push_type_available: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for VerificationResultPrimarySource {
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
        if let Some(some) = self.r#validation_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("validationDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_validationDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#can_push_updates.as_ref() {
            state.serialize_entry("canPushUpdates", some)?;
        }
        if !self.r#push_type_available.is_empty() {
            state.serialize_entry("pushTypeAvailable", &self.r#push_type_available)?;
        }
        state.end()
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
                        Field::Who => {
                            if r#who.is_some() {
                                return Err(serde::de::Error::duplicate_field("who"));
                            }
                            r#who = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::CommunicationMethod => {
                            if r#communication_method.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "communicationMethod",
                                ));
                            }
                            r#communication_method = Some(map_access.next_value()?);
                        }
                        Field::ValidationStatus => {
                            if r#validation_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationStatus"));
                            }
                            r#validation_status = Some(map_access.next_value()?);
                        }
                        Field::ValidationDate => {
                            let some = r#validation_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ValidationDatePrimitiveElement => {
                            let some = r#validation_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_validationDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::CanPushUpdates => {
                            if r#can_push_updates.is_some() {
                                return Err(serde::de::Error::duplicate_field("canPushUpdates"));
                            }
                            r#can_push_updates = Some(map_access.next_value()?);
                        }
                        Field::PushTypeAvailable => {
                            if r#push_type_available.is_some() {
                                return Err(serde::de::Error::duplicate_field("pushTypeAvailable"));
                            }
                            r#push_type_available = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct VerificationResultAttestation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#communication_method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#source_identity_certificate: Option<super::super::types::String>,
    pub r#proxy_identity_certificate: Option<super::super::types::String>,
    pub r#proxy_signature: Option<Box<super::super::types::Signature>>,
    pub r#source_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultAttestation {
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
        if let Some(some) = self.r#who.as_ref() {
            state.serialize_entry("who", some)?;
        }
        if let Some(some) = self.r#on_behalf_of.as_ref() {
            state.serialize_entry("onBehalfOf", some)?;
        }
        if let Some(some) = self.r#communication_method.as_ref() {
            state.serialize_entry("communicationMethod", some)?;
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
        if let Some(some) = self.r#source_identity_certificate.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sourceIdentityCertificate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sourceIdentityCertificate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#proxy_identity_certificate.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("proxyIdentityCertificate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_proxyIdentityCertificate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#proxy_signature.as_ref() {
            state.serialize_entry("proxySignature", some)?;
        }
        if let Some(some) = self.r#source_signature.as_ref() {
            state.serialize_entry("sourceSignature", some)?;
        }
        state.end()
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
                        Field::SourceIdentityCertificate => {
                            let some =
                                r#source_identity_certificate.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sourceIdentityCertificate",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SourceIdentityCertificatePrimitiveElement => {
                            let some =
                                r#source_identity_certificate.get_or_insert(Default::default());
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
                        }
                        Field::ProxyIdentityCertificate => {
                            let some =
                                r#proxy_identity_certificate.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "proxyIdentityCertificate",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ProxyIdentityCertificatePrimitiveElement => {
                            let some =
                                r#proxy_identity_certificate.get_or_insert(Default::default());
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
                        }
                        Field::ProxySignature => {
                            if r#proxy_signature.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxySignature"));
                            }
                            r#proxy_signature = Some(map_access.next_value()?);
                        }
                        Field::SourceSignature => {
                            if r#source_signature.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceSignature"));
                            }
                            r#source_signature = Some(map_access.next_value()?);
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct VerificationResultValidator {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#organization: Box<super::super::types::Reference>,
    pub r#identity_certificate: Option<super::super::types::String>,
    pub r#attestation_signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for VerificationResultValidator {
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
        state.serialize_entry("organization", &self.r#organization)?;
        if let Some(some) = self.r#identity_certificate.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("identityCertificate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_identityCertificate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#attestation_signature.as_ref() {
            state.serialize_entry("attestationSignature", some)?;
        }
        state.end()
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
                        Field::Organization => {
                            if r#organization.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            r#organization = Some(map_access.next_value()?);
                        }
                        Field::IdentityCertificate => {
                            let some = r#identity_certificate.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "identityCertificate",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IdentityCertificatePrimitiveElement => {
                            let some = r#identity_certificate.get_or_insert(Default::default());
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
                        }
                        Field::AttestationSignature => {
                            if r#attestation_signature.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "attestationSignature",
                                ));
                            }
                            r#attestation_signature = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(VerificationResultValidator {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#organization: r#organization
                        .ok_or(serde::de::Error::missing_field("organization"))?,
                    r#identity_certificate,
                    r#attestation_signature,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct VerificationResult {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: Vec<Box<super::super::types::Reference>>,
    pub r#target_location: Vec<super::super::types::String>,
    pub r#need: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#validation_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#validation_process: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#frequency: Option<Box<super::super::types::Timing>>,
    pub r#last_performed: Option<super::super::types::DateTime>,
    pub r#next_scheduled: Option<super::super::types::Date>,
    pub r#failure_action: Option<Box<super::super::types::CodeableConcept>>,
    pub r#primary_source: Vec<VerificationResultPrimarySource>,
    pub r#attestation: Option<VerificationResultAttestation>,
    pub r#validator: Vec<VerificationResultValidator>,
}
impl serde::ser::Serialize for VerificationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "VerificationResult")?;
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
        if !self.r#target.is_empty() {
            state.serialize_entry("target", &self.r#target)?;
        }
        if !self.r#target_location.is_empty() {
            let values: Vec<_> = self.r#target_location.iter().map(|v| &v.value).collect();
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
                                id: &e.id,
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
        if let Some(some) = self.r#need.as_ref() {
            state.serialize_entry("need", some)?;
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
        if let Some(some) = self.r#status_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("statusDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_statusDate", &primitive_element)?;
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
        if let Some(some) = self.r#last_performed.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastPerformed", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastPerformed", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#next_scheduled.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("nextScheduled", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_nextScheduled", &primitive_element)?;
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                        Field::Target => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#target = Some(map_access.next_value()?);
                        }
                        Field::TargetLocation => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#target_location.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("targetLocation"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::TargetLocationPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#target_location.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_targetLocation"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Need => {
                            if r#need.is_some() {
                                return Err(serde::de::Error::duplicate_field("need"));
                            }
                            r#need = Some(map_access.next_value()?);
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
                        Field::StatusDate => {
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusDatePrimitiveElement => {
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_statusDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ValidationType => {
                            if r#validation_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationType"));
                            }
                            r#validation_type = Some(map_access.next_value()?);
                        }
                        Field::ValidationProcess => {
                            if r#validation_process.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationProcess"));
                            }
                            r#validation_process = Some(map_access.next_value()?);
                        }
                        Field::Frequency => {
                            if r#frequency.is_some() {
                                return Err(serde::de::Error::duplicate_field("frequency"));
                            }
                            r#frequency = Some(map_access.next_value()?);
                        }
                        Field::LastPerformed => {
                            let some = r#last_performed.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastPerformed"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LastPerformedPrimitiveElement => {
                            let some = r#last_performed.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastPerformed"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::NextScheduled => {
                            let some = r#next_scheduled.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextScheduled"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NextScheduledPrimitiveElement => {
                            let some = r#next_scheduled.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_nextScheduled"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::FailureAction => {
                            if r#failure_action.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureAction"));
                            }
                            r#failure_action = Some(map_access.next_value()?);
                        }
                        Field::PrimarySource => {
                            if r#primary_source.is_some() {
                                return Err(serde::de::Error::duplicate_field("primarySource"));
                            }
                            r#primary_source = Some(map_access.next_value()?);
                        }
                        Field::Attestation => {
                            if r#attestation.is_some() {
                                return Err(serde::de::Error::duplicate_field("attestation"));
                            }
                            r#attestation = Some(map_access.next_value()?);
                        }
                        Field::Validator => {
                            if r#validator.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            r#validator = Some(map_access.next_value()?);
                        }
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
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
