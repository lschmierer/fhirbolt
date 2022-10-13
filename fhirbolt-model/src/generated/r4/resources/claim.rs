// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "The date when or period to which this information refers."]
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoTiming {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClaimSupportingInfoTiming {
    fn default() -> ClaimSupportingInfoTiming {
        ClaimSupportingInfoTiming::Invalid
    }
}
#[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
#[derive(Debug, Clone)]
pub enum ClaimSupportingInfoValue {
    Boolean(Box<super::super::types::Boolean>),
    String(Box<super::super::types::String>),
    Quantity(Box<super::super::types::Quantity>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimSupportingInfoValue {
    fn default() -> ClaimSupportingInfoValue {
        ClaimSupportingInfoValue::Invalid
    }
}
#[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
#[derive(Debug, Clone)]
pub enum ClaimDiagnosisDiagnosis {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimDiagnosisDiagnosis {
    fn default() -> ClaimDiagnosisDiagnosis {
        ClaimDiagnosisDiagnosis::Invalid
    }
}
#[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
#[derive(Debug, Clone)]
pub enum ClaimProcedureProcedure {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimProcedureProcedure {
    fn default() -> ClaimProcedureProcedure {
        ClaimProcedureProcedure::Invalid
    }
}
#[doc = "The physical location of the accident event."]
#[derive(Debug, Clone)]
pub enum ClaimAccidentLocation {
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimAccidentLocation {
    fn default() -> ClaimAccidentLocation {
        ClaimAccidentLocation::Invalid
    }
}
#[doc = "The date or dates when the service or product was supplied, performed or completed."]
#[derive(Debug, Clone)]
pub enum ClaimItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClaimItemServiced {
    fn default() -> ClaimItemServiced {
        ClaimItemServiced::Invalid
    }
}
#[doc = "Where the product or service was provided."]
#[derive(Debug, Clone)]
pub enum ClaimItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimItemLocation {
    fn default() -> ClaimItemLocation {
        ClaimItemLocation::Invalid
    }
}
#[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
#[derive(Default, Debug, Clone)]
pub struct ClaimRelated {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to a related claim."]
    pub r#claim: Option<Box<super::super::types::Reference>>,
    #[doc = "A code to convey how the claims are related."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An alternate organizational reference to the case or file to which this particular claim pertains."]
    pub r#reference: Option<Box<super::super::types::Identifier>>,
}
impl crate::AnyResource for ClaimRelated {}
impl serde::ser::Serialize for ClaimRelated {
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
        if let Some(some) = self.r#claim.as_ref() {
            state.serialize_entry("claim", some)?;
        }
        if let Some(some) = self.r#relationship.as_ref() {
            state.serialize_entry("relationship", some)?;
        }
        if let Some(some) = self.r#reference.as_ref() {
            state.serialize_entry("reference", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimRelated {
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
            #[serde(rename = "claim")]
            Claim,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "reference")]
            Reference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimRelated;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimRelated")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimRelated, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#claim: Option<Box<super::super::types::Reference>> = None;
                let mut r#relationship: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reference: Option<Box<super::super::types::Identifier>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Claim => {
                                if r#claim.is_some() {
                                    return Err(serde::de::Error::duplicate_field("claim"));
                                }
                                r#claim = Some(map_access.next_value()?);
                            }
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "claim",
                                            "relationship",
                                            "reference",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimRelated {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#claim,
                        r#relationship,
                        r#reference,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
#[derive(Default, Debug, Clone)]
pub struct ClaimPayee {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of Party to be reimbursed: subscriber, provider, other."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to the individual or organization to whom any payment will be made."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimPayee {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#party.as_ref() {
            state.serialize_entry("party", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimPayee {
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
            #[serde(rename = "party")]
            Party,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimPayee;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimPayee")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimPayee, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#party: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Party => {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "modifierExtension", "type", "party"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimPayee {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#party,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The members of the team who provided the products and services."]
#[derive(Default, Debug, Clone)]
pub struct ClaimCareTeam {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify care team entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "Member of the team who provided the product or service."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The party who is billing and/or responsible for the claimed products or services."]
    pub r#responsible: Option<super::super::types::Boolean>,
    #[doc = "The lead, assisting or supervising practitioner and their discipline if a multidisciplinary team."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The qualification of the practitioner which is applicable for this service."]
    pub r#qualification: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimCareTeam {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        state.serialize_entry("provider", &self.r#provider)?;
        if let Some(some) = self.r#responsible.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("responsible", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_responsible", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        if let Some(some) = self.r#qualification.as_ref() {
            state.serialize_entry("qualification", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimCareTeam {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "responsible")]
            Responsible,
            #[serde(rename = "_responsible")]
            ResponsiblePrimitiveElement,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "qualification")]
            Qualification,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimCareTeam;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimCareTeam")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimCareTeam, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#responsible: Option<super::super::types::Boolean> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#qualification: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::Responsible => {
                                let some = r#responsible.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("responsible"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ResponsiblePrimitiveElement => {
                                let some = r#responsible.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_responsible"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Qualification => {
                                if r#qualification.is_some() {
                                    return Err(serde::de::Error::duplicate_field("qualification"));
                                }
                                r#qualification = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "provider",
                                            "responsible",
                                            "role",
                                            "qualification",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimCareTeam {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#provider: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#provider.unwrap_or(Default::default())
                        } else {
                            r#provider.ok_or(serde::de::Error::missing_field("provider"))?
                        },
                        r#responsible,
                        r#role,
                        r#qualification,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
#[derive(Default, Debug, Clone)]
pub struct ClaimSupportingInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify supporting information entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The general class of the information supplied: information; exception; accident, employment; onset, etc."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "System and code pertaining to the specific information regarding special conditions relating to the setting, treatment or patient  for which care is sought."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date when or period to which this information refers."]
    pub r#timing: Option<ClaimSupportingInfoTiming>,
    #[doc = "Additional data or information such as resources, documents, images etc. including references to the data or the actual inclusion of the data."]
    pub r#value: Option<ClaimSupportingInfoValue>,
    #[doc = "Provides the reason in the situation where a reason code is required in addition to the content."]
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimSupportingInfo {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        state.serialize_entry("category", &self.r#category)?;
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#timing.as_ref() {
            match some {
                ClaimSupportingInfoTiming::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timingDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDate", &primitive_element)?;
                    }
                }
                ClaimSupportingInfoTiming::Period(ref value) => {
                    state.serialize_entry("timingPeriod", value)?;
                }
                ClaimSupportingInfoTiming::Invalid => {
                    return Err(serde::ser::Error::custom("timing is invalid"))
                }
            }
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ClaimSupportingInfoValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ClaimSupportingInfoValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ClaimSupportingInfoValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ClaimSupportingInfoValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ClaimSupportingInfoValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
                ClaimSupportingInfoValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimSupportingInfo {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "timingDate")]
            TimingDate,
            #[serde(rename = "_timingDate")]
            TimingDatePrimitiveElement,
            #[serde(rename = "timingPeriod")]
            TimingPeriod,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "reason")]
            Reason,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimSupportingInfo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimSupportingInfo")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimSupportingInfo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#timing: Option<ClaimSupportingInfoTiming> = None;
                let mut r#value: Option<ClaimSupportingInfoValue> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::TimingDate => {
                                let r#enum = r#timing.get_or_insert(
                                    ClaimSupportingInfoTiming::Date(Default::default()),
                                );
                                if let ClaimSupportingInfoTiming::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            }
                            Field::TimingDatePrimitiveElement => {
                                let r#enum = r#timing.get_or_insert(
                                    ClaimSupportingInfoTiming::Date(Default::default()),
                                );
                                if let ClaimSupportingInfoTiming::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timingDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_timing[x]"));
                                }
                            }
                            Field::TimingPeriod => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingPeriod"));
                                }
                                r#timing = Some(ClaimSupportingInfoTiming::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueBoolean => {
                                let r#enum = r#value.get_or_insert(
                                    ClaimSupportingInfoValue::Boolean(Default::default()),
                                );
                                if let ClaimSupportingInfoValue::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    ClaimSupportingInfoValue::Boolean(Default::default()),
                                );
                                if let ClaimSupportingInfoValue::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            }
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    ClaimSupportingInfoValue::String(Default::default()),
                                );
                                if let ClaimSupportingInfoValue::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    ClaimSupportingInfoValue::String(Default::default()),
                                );
                                if let ClaimSupportingInfoValue::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ClaimSupportingInfoValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value = Some(ClaimSupportingInfoValue::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(ClaimSupportingInfoValue::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "category",
                                            "code",
                                            "timingDate",
                                            "timingPeriod",
                                            "valueBoolean",
                                            "valueString",
                                            "valueQuantity",
                                            "valueAttachment",
                                            "valueReference",
                                            "reason",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimSupportingInfo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#category: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#category.unwrap_or(Default::default())
                        } else {
                            r#category.ok_or(serde::de::Error::missing_field("category"))?
                        },
                        r#code,
                        r#timing,
                        r#value,
                        r#reason,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about diagnoses relevant to the claim items."]
#[derive(Default, Debug, Clone)]
pub struct ClaimDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify diagnosis entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The nature of illness or problem in a coded form or as a reference to an external defined Condition."]
    pub r#diagnosis: ClaimDiagnosisDiagnosis,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indication of whether the diagnosis was present on admission to a facility."]
    pub r#on_admission: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A package billing code or bundle code used to group products and services to a particular health condition (such as heart attack) which is based on a predetermined grouping code system."]
    pub r#package_code: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimDiagnosis {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        match self.r#diagnosis {
            ClaimDiagnosisDiagnosis::CodeableConcept(ref value) => {
                state.serialize_entry("diagnosisCodeableConcept", value)?;
            }
            ClaimDiagnosisDiagnosis::Reference(ref value) => {
                state.serialize_entry("diagnosisReference", value)?;
            }
            ClaimDiagnosisDiagnosis::Invalid => {
                return Err(serde::ser::Error::custom("diagnosis is a required field"))
            }
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if let Some(some) = self.r#on_admission.as_ref() {
            state.serialize_entry("onAdmission", some)?;
        }
        if let Some(some) = self.r#package_code.as_ref() {
            state.serialize_entry("packageCode", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimDiagnosis {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "diagnosisCodeableConcept")]
            DiagnosisCodeableConcept,
            #[serde(rename = "diagnosisReference")]
            DiagnosisReference,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "onAdmission")]
            OnAdmission,
            #[serde(rename = "packageCode")]
            PackageCode,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimDiagnosis;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimDiagnosis")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimDiagnosis, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#diagnosis: Option<ClaimDiagnosisDiagnosis> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#on_admission: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#package_code: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DiagnosisCodeableConcept => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "diagnosisCodeableConcept",
                                    ));
                                }
                                r#diagnosis = Some(ClaimDiagnosisDiagnosis::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DiagnosisReference => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "diagnosisReference",
                                    ));
                                }
                                r#diagnosis = Some(ClaimDiagnosisDiagnosis::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::OnAdmission => {
                                if r#on_admission.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onAdmission"));
                                }
                                r#on_admission = Some(map_access.next_value()?);
                            }
                            Field::PackageCode => {
                                if r#package_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("packageCode"));
                                }
                                r#package_code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "diagnosisCodeableConcept",
                                            "diagnosisReference",
                                            "type",
                                            "onAdmission",
                                            "packageCode",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimDiagnosis {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#diagnosis: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#diagnosis.unwrap_or(Default::default())
                        } else {
                            r#diagnosis.ok_or(serde::de::Error::missing_field("diagnosis[x]"))?
                        },
                        r#type: r#type.unwrap_or(vec![]),
                        r#on_admission,
                        r#package_code,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
#[derive(Default, Debug, Clone)]
pub struct ClaimProcedure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify procedure entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "When the condition was observed or the relative ranking."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date and optionally time the procedure was performed."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The code or reference to a Procedure resource which identifies the clinical intervention performed."]
    pub r#procedure: ClaimProcedureProcedure,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimProcedure {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
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
        match self.r#procedure {
            ClaimProcedureProcedure::CodeableConcept(ref value) => {
                state.serialize_entry("procedureCodeableConcept", value)?;
            }
            ClaimProcedureProcedure::Reference(ref value) => {
                state.serialize_entry("procedureReference", value)?;
            }
            ClaimProcedureProcedure::Invalid => {
                return Err(serde::ser::Error::custom("procedure is a required field"))
            }
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimProcedure {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "procedureCodeableConcept")]
            ProcedureCodeableConcept,
            #[serde(rename = "procedureReference")]
            ProcedureReference,
            #[serde(rename = "udi")]
            Udi,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimProcedure;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimProcedure")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimProcedure, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#procedure: Option<ClaimProcedureProcedure> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
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
                            Field::ProcedureCodeableConcept => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "procedureCodeableConcept",
                                    ));
                                }
                                r#procedure = Some(ClaimProcedureProcedure::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ProcedureReference => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "procedureReference",
                                    ));
                                }
                                r#procedure = Some(ClaimProcedureProcedure::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "type",
                                            "date",
                                            "procedureCodeableConcept",
                                            "procedureReference",
                                            "udi",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimProcedure {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#type: r#type.unwrap_or(vec![]),
                        r#date,
                        r#procedure: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#procedure.unwrap_or(Default::default())
                        } else {
                            r#procedure.ok_or(serde::de::Error::missing_field("procedure[x]"))?
                        },
                        r#udi: r#udi.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
#[derive(Default, Debug, Clone)]
pub struct ClaimInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify insurance entries and provide a sequence of coverages to convey coordination of benefit order."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "A flag to indicate that this Coverage is to be used for adjudication of this claim when set to true."]
    pub r#focal: super::super::types::Boolean,
    #[doc = "The business identifier to be used when the claim is sent for adjudication against this insurance policy."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "A business agreement number established between the provider and the insurer for special business processing purposes."]
    pub r#business_arrangement: Option<super::super::types::String>,
    #[doc = "Reference numbers previously provided by the insurer to the provider to be quoted on subsequent claims containing services or products related to the prior authorization."]
    pub r#pre_auth_ref: Vec<super::super::types::String>,
    #[doc = "The result of the adjudication of the line items for the Coverage specified in this insurance."]
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimInsurance {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#focal.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("focal", &some)?;
        }
        if self.r#focal.id.is_some() || !self.r#focal.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#focal.id,
                extension: &self.r#focal.extension,
            };
            state.serialize_entry("_focal", &primitive_element)?;
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.serialize_entry("coverage", &self.r#coverage)?;
        if let Some(some) = self.r#business_arrangement.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("businessArrangement", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_businessArrangement", &primitive_element)?;
            }
        }
        if !self.r#pre_auth_ref.is_empty() {
            let values = self
                .r#pre_auth_ref
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("preAuthRef", &values)?;
            }
            let requires_elements = self
                .r#pre_auth_ref
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#pre_auth_ref
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
                state.serialize_entry("_preAuthRef", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#claim_response.as_ref() {
            state.serialize_entry("claimResponse", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimInsurance {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "focal")]
            Focal,
            #[serde(rename = "_focal")]
            FocalPrimitiveElement,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "businessArrangement")]
            BusinessArrangement,
            #[serde(rename = "_businessArrangement")]
            BusinessArrangementPrimitiveElement,
            #[serde(rename = "preAuthRef")]
            PreAuthRef,
            #[serde(rename = "_preAuthRef")]
            PreAuthRefPrimitiveElement,
            #[serde(rename = "claimResponse")]
            ClaimResponse,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimInsurance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimInsurance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#focal: Option<super::super::types::Boolean> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#business_arrangement: Option<super::super::types::String> = None;
                let mut r#pre_auth_ref: Option<Vec<super::super::types::String>> = None;
                let mut r#claim_response: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Focal => {
                                let some = r#focal.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focal"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::FocalPrimitiveElement => {
                                let some = r#focal.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_focal"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Coverage => {
                                if r#coverage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("coverage"));
                                }
                                r#coverage = Some(map_access.next_value()?);
                            }
                            Field::BusinessArrangement => {
                                let some = r#business_arrangement.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "businessArrangement",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::BusinessArrangementPrimitiveElement => {
                                let some = r#business_arrangement.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_businessArrangement",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PreAuthRef => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::PreAuthRefPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#pre_auth_ref.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::ClaimResponse => {
                                if r#claim_response.is_some() {
                                    return Err(serde::de::Error::duplicate_field("claimResponse"));
                                }
                                r#claim_response = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "focal",
                                            "identifier",
                                            "coverage",
                                            "businessArrangement",
                                            "preAuthRef",
                                            "claimResponse",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimInsurance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#focal: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#focal.unwrap_or(Default::default())
                        } else {
                            r#focal.ok_or(serde::de::Error::missing_field("focal"))?
                        },
                        r#identifier,
                        r#coverage: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#coverage.unwrap_or(Default::default())
                        } else {
                            r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                        },
                        r#business_arrangement,
                        r#pre_auth_ref: r#pre_auth_ref.unwrap_or(vec![]),
                        r#claim_response,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Details of an accident which resulted in injuries which required the products and services listed in the claim."]
#[derive(Default, Debug, Clone)]
pub struct ClaimAccident {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Date of an accident event  related to the products and services contained in the claim."]
    pub r#date: super::super::types::Date,
    #[doc = "The type or context of the accident event for the purposes of selection of potential insurance coverages and determination of coordination between insurers."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The physical location of the accident event."]
    pub r#location: Option<ClaimAccidentLocation>,
}
impl serde::ser::Serialize for ClaimAccident {
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
        if let Some(some) = self.r#date.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("date", &some)?;
        }
        if self.r#date.id.is_some() || !self.r#date.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date.id,
                extension: &self.r#date.extension,
            };
            state.serialize_entry("_date", &primitive_element)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ClaimAccidentLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ClaimAccidentLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ClaimAccidentLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimAccident {
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
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimAccident;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimAccident")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimAccident, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#location: Option<ClaimAccidentLocation> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::LocationAddress => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationAddress",
                                    ));
                                }
                                r#location =
                                    Some(ClaimAccidentLocation::Address(map_access.next_value()?));
                            }
                            Field::LocationReference => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationReference",
                                    ));
                                }
                                r#location = Some(ClaimAccidentLocation::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "date",
                                            "type",
                                            "locationAddress",
                                            "locationReference",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimAccident {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#date: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#date.unwrap_or(Default::default())
                        } else {
                            r#date.ok_or(serde::de::Error::missing_field("date"))?
                        },
                        r#type,
                        r#location,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
#[derive(Default, Debug, Clone)]
pub struct ClaimItemDetailSubDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify item entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimItemDetailSubDetail {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("factor", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItemDetailSubDetail {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItemDetailSubDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItemDetailSubDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::FactorPrimitiveElement => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimItemDetailSubDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#revenue,
                        r#category,
                        r#product_or_service: if config.mode
                            == fhirbolt_shared::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#program_code: r#program_code.unwrap_or(vec![]),
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#net,
                        r#udi: r#udi.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
#[derive(Default, Debug, Clone)]
pub struct ClaimItemDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify item entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#sub_detail: Vec<ClaimItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimItemDetail {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("factor", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItemDetail {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            #[serde(rename = "subDetail")]
            SubDetail,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItemDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#sub_detail: Option<Vec<ClaimItemDetailSubDetail>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::FactorPrimitiveElement => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::SubDetail => {
                                if r#sub_detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subDetail"));
                                }
                                r#sub_detail = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "subDetail",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimItemDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#revenue,
                        r#category,
                        r#product_or_service: if config.mode
                            == fhirbolt_shared::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#program_code: r#program_code.unwrap_or(vec![]),
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#net,
                        r#udi: r#udi.unwrap_or(vec![]),
                        r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A claim line. Either a simple  product or service or a 'group' of details which can each be a simple items or groups of sub-details."]
#[derive(Default, Debug, Clone)]
pub struct ClaimItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A number to uniquely identify item entries."]
    pub r#sequence: super::super::types::PositiveInt,
    #[doc = "CareTeam members related to this service or product."]
    pub r#care_team_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Diagnosis applicable for this service or product."]
    pub r#diagnosis_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Procedures applicable for this service or product."]
    pub r#procedure_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "Exceptions, special conditions and supporting information applicable for this service or product."]
    pub r#information_sequence: Vec<super::super::types::PositiveInt>,
    #[doc = "The type of revenue or cost center providing the product and/or service."]
    pub r#revenue: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "When the value is a group code then this item collects a set of related claim details, otherwise this contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the program under which this may be recovered."]
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date or dates when the service or product was supplied, performed or completed."]
    pub r#serviced: Option<ClaimItemServiced>,
    #[doc = "Where the product or service was provided."]
    pub r#location: Option<ClaimItemLocation>,
    #[doc = "The number of repetitions of a service or product."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "If the item is not a group then this is the fee for the product or service, otherwise this is the total of the fees for the details of the group."]
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    #[doc = "A real number that represents a multiplier used in determining the overall value of services delivered and/or goods received. The concept of a Factor allows for a discount or surcharge multiplier to be applied to a monetary amount."]
    pub r#factor: Option<super::super::types::Decimal>,
    #[doc = "The quantity times the unit price for an additional service or product or charge."]
    pub r#net: Option<Box<super::super::types::Money>>,
    #[doc = "Unique Device Identifiers associated with this line item."]
    pub r#udi: Vec<Box<super::super::types::Reference>>,
    #[doc = "Physical service site on the patient (limb, tooth, etc.)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A region or surface of the bodySite, e.g. limb region or tooth surface(s)."]
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The Encounters during which this Claim was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    #[doc = "A claim detail line. Either a simple (a product or service) or a 'group' of sub-details which are simple items."]
    pub r#detail: Vec<ClaimItemDetail>,
}
impl serde::ser::Serialize for ClaimItem {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("sequence", &some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if !self.r#care_team_sequence.is_empty() {
            let values = self
                .r#care_team_sequence
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("careTeamSequence", &values)?;
            }
            let requires_elements = self
                .r#care_team_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#care_team_sequence
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
                state.serialize_entry("_careTeamSequence", &primitive_elements)?;
            }
        }
        if !self.r#diagnosis_sequence.is_empty() {
            let values = self
                .r#diagnosis_sequence
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("diagnosisSequence", &values)?;
            }
            let requires_elements = self
                .r#diagnosis_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#diagnosis_sequence
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
                state.serialize_entry("_diagnosisSequence", &primitive_elements)?;
            }
        }
        if !self.r#procedure_sequence.is_empty() {
            let values = self
                .r#procedure_sequence
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("procedureSequence", &values)?;
            }
            let requires_elements = self
                .r#procedure_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#procedure_sequence
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
                state.serialize_entry("_procedureSequence", &primitive_elements)?;
            }
        }
        if !self.r#information_sequence.is_empty() {
            let values = self
                .r#information_sequence
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("informationSequence", &values)?;
            }
            let requires_elements = self
                .r#information_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#information_sequence
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
                state.serialize_entry("_informationSequence", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#revenue.as_ref() {
            state.serialize_entry("revenue", some)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#serviced.as_ref() {
            match some {
                ClaimItemServiced::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("servicedDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_servicedDate", &primitive_element)?;
                    }
                }
                ClaimItemServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
                ClaimItemServiced::Invalid => {
                    return Err(serde::ser::Error::custom("serviced is invalid"))
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ClaimItemLocation::CodeableConcept(ref value) => {
                    state.serialize_entry("locationCodeableConcept", value)?;
                }
                ClaimItemLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ClaimItemLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ClaimItemLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("factor", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#udi.is_empty() {
            state.serialize_entry("udi", &self.r#udi)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if !self.r#sub_site.is_empty() {
            state.serialize_entry("subSite", &self.r#sub_site)?;
        }
        if !self.r#encounter.is_empty() {
            state.serialize_entry("encounter", &self.r#encounter)?;
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimItem {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "careTeamSequence")]
            CareTeamSequence,
            #[serde(rename = "_careTeamSequence")]
            CareTeamSequencePrimitiveElement,
            #[serde(rename = "diagnosisSequence")]
            DiagnosisSequence,
            #[serde(rename = "_diagnosisSequence")]
            DiagnosisSequencePrimitiveElement,
            #[serde(rename = "procedureSequence")]
            ProcedureSequence,
            #[serde(rename = "_procedureSequence")]
            ProcedureSequencePrimitiveElement,
            #[serde(rename = "informationSequence")]
            InformationSequence,
            #[serde(rename = "_informationSequence")]
            InformationSequencePrimitiveElement,
            #[serde(rename = "revenue")]
            Revenue,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "servicedDate")]
            ServicedDate,
            #[serde(rename = "_servicedDate")]
            ServicedDatePrimitiveElement,
            #[serde(rename = "servicedPeriod")]
            ServicedPeriod,
            #[serde(rename = "locationCodeableConcept")]
            LocationCodeableConcept,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "udi")]
            Udi,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "subSite")]
            SubSite,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "detail")]
            Detail,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#care_team_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#diagnosis_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#procedure_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#information_sequence: Option<Vec<super::super::types::PositiveInt>> =
                    None;
                let mut r#revenue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#serviced: Option<ClaimItemServiced> = None;
                let mut r#location: Option<ClaimItemLocation> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#udi: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#encounter: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#detail: Option<Vec<ClaimItemDetail>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                            Field::Sequence => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SequencePrimitiveElement => {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::CareTeamSequence => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#care_team_sequence.get_or_insert(
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
                                        "careTeamSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::CareTeamSequencePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#care_team_sequence.get_or_insert(
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
                                        "_careTeamSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::DiagnosisSequence => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#diagnosis_sequence.get_or_insert(
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
                                        "diagnosisSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::DiagnosisSequencePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#diagnosis_sequence.get_or_insert(
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
                                        "_diagnosisSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::ProcedureSequence => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#procedure_sequence.get_or_insert(
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
                                        "procedureSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::ProcedureSequencePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#procedure_sequence.get_or_insert(
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
                                        "_procedureSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::InformationSequence => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#information_sequence.get_or_insert(
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
                                        "informationSequence",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InformationSequencePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#information_sequence.get_or_insert(
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
                                        "_informationSequence",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Revenue => {
                                if r#revenue.is_some() {
                                    return Err(serde::de::Error::duplicate_field("revenue"));
                                }
                                r#revenue = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::ProgramCode => {
                                if r#program_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("programCode"));
                                }
                                r#program_code = Some(map_access.next_value()?);
                            }
                            Field::ServicedDate => {
                                let r#enum = r#serviced
                                    .get_or_insert(ClaimItemServiced::Date(Default::default()));
                                if let ClaimItemServiced::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "servicedDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("serviced[x]"));
                                }
                            }
                            Field::ServicedDatePrimitiveElement => {
                                let r#enum = r#serviced
                                    .get_or_insert(ClaimItemServiced::Date(Default::default()));
                                if let ClaimItemServiced::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_servicedDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                                }
                            }
                            Field::ServicedPeriod => {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "servicedPeriod",
                                    ));
                                }
                                r#serviced =
                                    Some(ClaimItemServiced::Period(map_access.next_value()?));
                            }
                            Field::LocationCodeableConcept => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationCodeableConcept",
                                    ));
                                }
                                r#location = Some(ClaimItemLocation::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::LocationAddress => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationAddress",
                                    ));
                                }
                                r#location =
                                    Some(ClaimItemLocation::Address(map_access.next_value()?));
                            }
                            Field::LocationReference => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "locationReference",
                                    ));
                                }
                                r#location =
                                    Some(ClaimItemLocation::Reference(map_access.next_value()?));
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
                            }
                            Field::UnitPrice => {
                                if r#unit_price.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unitPrice"));
                                }
                                r#unit_price = Some(map_access.next_value()?);
                            }
                            Field::Factor => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            }
                            Field::FactorPrimitiveElement => {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Net => {
                                if r#net.is_some() {
                                    return Err(serde::de::Error::duplicate_field("net"));
                                }
                                r#net = Some(map_access.next_value()?);
                            }
                            Field::Udi => {
                                if r#udi.is_some() {
                                    return Err(serde::de::Error::duplicate_field("udi"));
                                }
                                r#udi = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::SubSite => {
                                if r#sub_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subSite"));
                                }
                                r#sub_site = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "sequence",
                                            "careTeamSequence",
                                            "diagnosisSequence",
                                            "procedureSequence",
                                            "informationSequence",
                                            "revenue",
                                            "category",
                                            "productOrService",
                                            "modifier",
                                            "programCode",
                                            "servicedDate",
                                            "servicedPeriod",
                                            "locationCodeableConcept",
                                            "locationAddress",
                                            "locationReference",
                                            "quantity",
                                            "unitPrice",
                                            "factor",
                                            "net",
                                            "udi",
                                            "bodySite",
                                            "subSite",
                                            "encounter",
                                            "detail",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(ClaimItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#sequence: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#sequence.unwrap_or(Default::default())
                        } else {
                            r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?
                        },
                        r#care_team_sequence: r#care_team_sequence.unwrap_or(vec![]),
                        r#diagnosis_sequence: r#diagnosis_sequence.unwrap_or(vec![]),
                        r#procedure_sequence: r#procedure_sequence.unwrap_or(vec![]),
                        r#information_sequence: r#information_sequence.unwrap_or(vec![]),
                        r#revenue,
                        r#category,
                        r#product_or_service: if config.mode
                            == fhirbolt_shared::DeserializationMode::Lax
                        {
                            r#product_or_service.unwrap_or(Default::default())
                        } else {
                            r#product_or_service
                                .ok_or(serde::de::Error::missing_field("productOrService"))?
                        },
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#program_code: r#program_code.unwrap_or(vec![]),
                        r#serviced,
                        r#location,
                        r#quantity,
                        r#unit_price,
                        r#factor,
                        r#net,
                        r#udi: r#udi.unwrap_or(vec![]),
                        r#body_site,
                        r#sub_site: r#sub_site.unwrap_or(vec![]),
                        r#encounter: r#encounter.unwrap_or(vec![]),
                        r#detail: r#detail.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.\n\nThe Claim resource is used by providers to exchange services and products rendered to patients or planned to be rendered with insurers for reimbuserment. It is also used by insurers to exchange claims information with statutory reporting and data analytics firms."]
#[derive(Default, Debug, Clone)]
pub struct Claim {
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
    #[doc = "A unique identifier assigned to this claim."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The category of claim, e.g. oral, pharmacy, vision, institutional, professional."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A finer grained suite of claim type codes which may convey additional information such as Inpatient vs Outpatient and/or a specialty service."]
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate whether the nature of the request is: to request adjudication of products and services previously rendered; or requesting authorization and adjudication for provision in the future; or requesting the non-binding adjudication of the listed products and services which could be provided in the future."]
    pub r#use: super::super::types::Code,
    #[doc = "The party to whom the professional services and/or products have been supplied or are being considered and for whom actual or forecast reimbursement is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The period for which charges are being submitted."]
    pub r#billable_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "Individual who created the claim, predetermination or preauthorization."]
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    #[doc = "The Insurer who is target of the request."]
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    #[doc = "The provider which is responsible for the claim, predetermination or preauthorization."]
    pub r#provider: Box<super::super::types::Reference>,
    #[doc = "The provider-required urgency of processing the request. Typical values include: stat, routine deferred."]
    pub r#priority: Box<super::super::types::CodeableConcept>,
    #[doc = "A code to indicate whether and for whom funds are to be reserved for future claims."]
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Other claims which are related to this claim such as prior submissions or claims for related services or for the same event."]
    pub r#related: Vec<ClaimRelated>,
    #[doc = "Prescription to support the dispensing of pharmacy, device or vision products."]
    pub r#prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "Original prescription which has been superseded by this prescription to support the dispensing of pharmacy services, medications or products."]
    pub r#original_prescription: Option<Box<super::super::types::Reference>>,
    #[doc = "The party to be reimbursed for cost of the products and services according to the terms of the policy."]
    pub r#payee: Option<ClaimPayee>,
    #[doc = "A reference to a referral resource."]
    pub r#referral: Option<Box<super::super::types::Reference>>,
    #[doc = "Facility where the services were provided."]
    pub r#facility: Option<Box<super::super::types::Reference>>,
    #[doc = "The members of the team who provided the products and services."]
    pub r#care_team: Vec<ClaimCareTeam>,
    #[doc = "Additional information codes regarding exceptions, special considerations, the condition, situation, prior or concurrent issues."]
    pub r#supporting_info: Vec<ClaimSupportingInfo>,
    #[doc = "Information about diagnoses relevant to the claim items."]
    pub r#diagnosis: Vec<ClaimDiagnosis>,
    #[doc = "Procedures performed on the patient relevant to the billing items with the claim."]
    pub r#procedure: Vec<ClaimProcedure>,
    #[doc = "Financial instruments for reimbursement for the health care products and services specified on the claim."]
    pub r#insurance: Vec<ClaimInsurance>,
    #[doc = "Details of an accident which resulted in injuries which required the products and services listed in the claim."]
    pub r#accident: Option<ClaimAccident>,
    #[doc = "A claim line. Either a simple  product or service or a 'group' of details which can each be a simple items or groups of sub-details."]
    pub r#item: Vec<ClaimItem>,
    #[doc = "The total value of the all the items in the claim."]
    pub r#total: Option<Box<super::super::types::Money>>,
}
impl serde::ser::Serialize for Claim {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Claim")?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#sub_type.as_ref() {
            state.serialize_entry("subType", some)?;
        }
        if let Some(some) = self.r#use.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("use", &some)?;
        }
        if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#use.id,
                extension: &self.r#use.extension,
            };
            state.serialize_entry("_use", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#billable_period.as_ref() {
            state.serialize_entry("billablePeriod", some)?;
        }
        if let Some(some) = self.r#created.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("created", &some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        if let Some(some) = self.r#enterer.as_ref() {
            state.serialize_entry("enterer", some)?;
        }
        if let Some(some) = self.r#insurer.as_ref() {
            state.serialize_entry("insurer", some)?;
        }
        state.serialize_entry("provider", &self.r#provider)?;
        state.serialize_entry("priority", &self.r#priority)?;
        if let Some(some) = self.r#funds_reserve.as_ref() {
            state.serialize_entry("fundsReserve", some)?;
        }
        if !self.r#related.is_empty() {
            state.serialize_entry("related", &self.r#related)?;
        }
        if let Some(some) = self.r#prescription.as_ref() {
            state.serialize_entry("prescription", some)?;
        }
        if let Some(some) = self.r#original_prescription.as_ref() {
            state.serialize_entry("originalPrescription", some)?;
        }
        if let Some(some) = self.r#payee.as_ref() {
            state.serialize_entry("payee", some)?;
        }
        if let Some(some) = self.r#referral.as_ref() {
            state.serialize_entry("referral", some)?;
        }
        if let Some(some) = self.r#facility.as_ref() {
            state.serialize_entry("facility", some)?;
        }
        if !self.r#care_team.is_empty() {
            state.serialize_entry("careTeam", &self.r#care_team)?;
        }
        if !self.r#supporting_info.is_empty() {
            state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
        }
        if !self.r#diagnosis.is_empty() {
            state.serialize_entry("diagnosis", &self.r#diagnosis)?;
        }
        if !self.r#procedure.is_empty() {
            state.serialize_entry("procedure", &self.r#procedure)?;
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if let Some(some) = self.r#accident.as_ref() {
            state.serialize_entry("accident", some)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        if let Some(some) = self.r#total.as_ref() {
            state.serialize_entry("total", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Claim {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "billablePeriod")]
            BillablePeriod,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "enterer")]
            Enterer,
            #[serde(rename = "insurer")]
            Insurer,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "fundsReserve")]
            FundsReserve,
            #[serde(rename = "related")]
            Related,
            #[serde(rename = "prescription")]
            Prescription,
            #[serde(rename = "originalPrescription")]
            OriginalPrescription,
            #[serde(rename = "payee")]
            Payee,
            #[serde(rename = "referral")]
            Referral,
            #[serde(rename = "facility")]
            Facility,
            #[serde(rename = "careTeam")]
            CareTeam,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "diagnosis")]
            Diagnosis,
            #[serde(rename = "procedure")]
            Procedure,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "accident")]
            Accident,
            #[serde(rename = "item")]
            Item,
            #[serde(rename = "total")]
            Total,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Claim;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Claim")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Claim, V::Error>
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
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#billable_period: Option<Box<super::super::types::Period>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#enterer: Option<Box<super::super::types::Reference>> = None;
                let mut r#insurer: Option<Box<super::super::types::Reference>> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#priority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#funds_reserve: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#related: Option<Vec<ClaimRelated>> = None;
                let mut r#prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#original_prescription: Option<Box<super::super::types::Reference>> = None;
                let mut r#payee: Option<ClaimPayee> = None;
                let mut r#referral: Option<Box<super::super::types::Reference>> = None;
                let mut r#facility: Option<Box<super::super::types::Reference>> = None;
                let mut r#care_team: Option<Vec<ClaimCareTeam>> = None;
                let mut r#supporting_info: Option<Vec<ClaimSupportingInfo>> = None;
                let mut r#diagnosis: Option<Vec<ClaimDiagnosis>> = None;
                let mut r#procedure: Option<Vec<ClaimProcedure>> = None;
                let mut r#insurance: Option<Vec<ClaimInsurance>> = None;
                let mut r#accident: Option<ClaimAccident> = None;
                let mut r#item: Option<Vec<ClaimItem>> = None;
                let mut r#total: Option<Box<super::super::types::Money>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Claim" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Claim",
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
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::SubType => {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type = Some(map_access.next_value()?);
                            }
                            Field::Use => {
                                let some = r#use.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("use"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UsePrimitiveElement => {
                                let some = r#use.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_use"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::BillablePeriod => {
                                if r#billable_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "billablePeriod",
                                    ));
                                }
                                r#billable_period = Some(map_access.next_value()?);
                            }
                            Field::Created => {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CreatedPrimitiveElement => {
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
                            }
                            Field::Enterer => {
                                if r#enterer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("enterer"));
                                }
                                r#enterer = Some(map_access.next_value()?);
                            }
                            Field::Insurer => {
                                if r#insurer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurer"));
                                }
                                r#insurer = Some(map_access.next_value()?);
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value()?);
                            }
                            Field::FundsReserve => {
                                if r#funds_reserve.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fundsReserve"));
                                }
                                r#funds_reserve = Some(map_access.next_value()?);
                            }
                            Field::Related => {
                                if r#related.is_some() {
                                    return Err(serde::de::Error::duplicate_field("related"));
                                }
                                r#related = Some(map_access.next_value()?);
                            }
                            Field::Prescription => {
                                if r#prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prescription"));
                                }
                                r#prescription = Some(map_access.next_value()?);
                            }
                            Field::OriginalPrescription => {
                                if r#original_prescription.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "originalPrescription",
                                    ));
                                }
                                r#original_prescription = Some(map_access.next_value()?);
                            }
                            Field::Payee => {
                                if r#payee.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payee"));
                                }
                                r#payee = Some(map_access.next_value()?);
                            }
                            Field::Referral => {
                                if r#referral.is_some() {
                                    return Err(serde::de::Error::duplicate_field("referral"));
                                }
                                r#referral = Some(map_access.next_value()?);
                            }
                            Field::Facility => {
                                if r#facility.is_some() {
                                    return Err(serde::de::Error::duplicate_field("facility"));
                                }
                                r#facility = Some(map_access.next_value()?);
                            }
                            Field::CareTeam => {
                                if r#care_team.is_some() {
                                    return Err(serde::de::Error::duplicate_field("careTeam"));
                                }
                                r#care_team = Some(map_access.next_value()?);
                            }
                            Field::SupportingInfo => {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                r#supporting_info = Some(map_access.next_value()?);
                            }
                            Field::Diagnosis => {
                                if r#diagnosis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diagnosis"));
                                }
                                r#diagnosis = Some(map_access.next_value()?);
                            }
                            Field::Procedure => {
                                if r#procedure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("procedure"));
                                }
                                r#procedure = Some(map_access.next_value()?);
                            }
                            Field::Insurance => {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some(map_access.next_value()?);
                            }
                            Field::Accident => {
                                if r#accident.is_some() {
                                    return Err(serde::de::Error::duplicate_field("accident"));
                                }
                                r#accident = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
                            }
                            Field::Total => {
                                if r#total.is_some() {
                                    return Err(serde::de::Error::duplicate_field("total"));
                                }
                                r#total = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
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
                                            "type",
                                            "subType",
                                            "use",
                                            "patient",
                                            "billablePeriod",
                                            "created",
                                            "enterer",
                                            "insurer",
                                            "provider",
                                            "priority",
                                            "fundsReserve",
                                            "related",
                                            "prescription",
                                            "originalPrescription",
                                            "payee",
                                            "referral",
                                            "facility",
                                            "careTeam",
                                            "supportingInfo",
                                            "diagnosis",
                                            "procedure",
                                            "insurance",
                                            "accident",
                                            "item",
                                            "total",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Claim {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#type: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#sub_type,
                        r#use: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#use.unwrap_or(Default::default())
                        } else {
                            r#use.ok_or(serde::de::Error::missing_field("use"))?
                        },
                        r#patient: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#billable_period,
                        r#created: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#enterer,
                        r#insurer,
                        r#provider: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#provider.unwrap_or(Default::default())
                        } else {
                            r#provider.ok_or(serde::de::Error::missing_field("provider"))?
                        },
                        r#priority: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#priority.unwrap_or(Default::default())
                        } else {
                            r#priority.ok_or(serde::de::Error::missing_field("priority"))?
                        },
                        r#funds_reserve,
                        r#related: r#related.unwrap_or(vec![]),
                        r#prescription,
                        r#original_prescription,
                        r#payee,
                        r#referral,
                        r#facility,
                        r#care_team: r#care_team.unwrap_or(vec![]),
                        r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                        r#diagnosis: r#diagnosis.unwrap_or(vec![]),
                        r#procedure: r#procedure.unwrap_or(vec![]),
                        r#insurance: r#insurance.unwrap_or(vec![]),
                        r#accident,
                        r#item: r#item.unwrap_or(vec![]),
                        r#total,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
