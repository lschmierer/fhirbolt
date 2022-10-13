// Generated on 2022-10-13 by fhirbolt-codegen v0.1.0
#[doc = "Indicates if the individual is deceased or not."]
#[derive(Debug, Clone)]
pub enum PatientDeceased {
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    Invalid,
}
impl Default for PatientDeceased {
    fn default() -> PatientDeceased {
        PatientDeceased::Invalid
    }
}
#[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
#[derive(Debug, Clone)]
pub enum PatientMultipleBirth {
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Invalid,
}
impl Default for PatientMultipleBirth {
    fn default() -> PatientMultipleBirth {
        PatientMultipleBirth::Invalid
    }
}
#[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
#[derive(Default, Debug, Clone)]
pub struct PatientContact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The nature of the relationship between the patient and the contact person."]
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A name associated with the contact person."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the person, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Address for the contact person."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "Administrative Gender - the gender that the contact person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "Organization on behalf of which the contact is acting or for which the contact is working."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The period during which this contact person or organization is valid to be contacted relating to this patient."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl crate::AnyResource for PatientContact {}
impl serde::ser::Serialize for PatientContact {
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
        if !self.r#relationship.is_empty() {
            state.serialize_entry("relationship", &self.r#relationship)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if !self.r#telecom.is_empty() {
            state.serialize_entry("telecom", &self.r#telecom)?;
        }
        if let Some(some) = self.r#address.as_ref() {
            state.serialize_entry("address", some)?;
        }
        if let Some(some) = self.r#gender.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("gender", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_gender", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#organization.as_ref() {
            state.serialize_entry("organization", some)?;
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PatientContact {
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
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "gender")]
            Gender,
            #[serde(rename = "_gender")]
            GenderPrimitiveElement,
            #[serde(rename = "organization")]
            Organization,
            #[serde(rename = "period")]
            Period,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PatientContact;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PatientContact")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PatientContact, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relationship: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#name: Option<Box<super::super::types::HumanName>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#address: Option<Box<super::super::types::Address>> = None;
                let mut r#gender: Option<super::super::types::Code> = None;
                let mut r#organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                            Field::Relationship => {
                                if r#relationship.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                r#relationship = Some(map_access.next_value()?);
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some(map_access.next_value()?);
                            }
                            Field::Address => {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                r#address = Some(map_access.next_value()?);
                            }
                            Field::Gender => {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gender"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::GenderPrimitiveElement => {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_gender"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Organization => {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                r#organization = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "relationship",
                                            "name",
                                            "telecom",
                                            "address",
                                            "gender",
                                            "organization",
                                            "period",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(PatientContact {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relationship: r#relationship.unwrap_or(vec![]),
                        r#name,
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#address,
                        r#gender,
                        r#organization,
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A language which may be used to communicate with the patient about his or her health."]
#[derive(Default, Debug, Clone)]
pub struct PatientCommunication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The ISO-639-1 alpha 2 code in lower case for the language, optionally followed by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g. \"en\" for English, or \"en-US\" for American English versus \"en-EN\" for England English."]
    pub r#language: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates whether or not the patient prefers this language (over other languages he masters up a certain level)."]
    pub r#preferred: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for PatientCommunication {
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
        state.serialize_entry("language", &self.r#language)?;
        if let Some(some) = self.r#preferred.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("preferred", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preferred", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PatientCommunication {
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
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "preferred")]
            Preferred,
            #[serde(rename = "_preferred")]
            PreferredPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PatientCommunication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PatientCommunication")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PatientCommunication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preferred: Option<super::super::types::Boolean> = None;
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
                            Field::Language => {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value()?);
                            }
                            Field::Preferred => {
                                let some = r#preferred.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preferred"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PreferredPrimitiveElement => {
                                let some = r#preferred.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preferred"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "language",
                                            "preferred",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(PatientCommunication {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#language: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#language.unwrap_or(Default::default())
                        } else {
                            r#language.ok_or(serde::de::Error::missing_field("language"))?
                        },
                        r#preferred,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Link to another patient resource that concerns the same actual patient."]
#[derive(Default, Debug, Clone)]
pub struct PatientLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The other patient resource that the link refers to."]
    pub r#other: Box<super::super::types::Reference>,
    #[doc = "The type of link between this patient resource and another patient resource."]
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for PatientLink {
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
        state.serialize_entry("other", &self.r#other)?;
        if let Some(some) = self.r#type.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("type", &some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PatientLink {
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
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PatientLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PatientLink")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PatientLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#other: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
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
                            Field::Other => {
                                if r#other.is_some() {
                                    return Err(serde::de::Error::duplicate_field("other"));
                                }
                                r#other = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TypePrimitiveElement => {
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
                            }
                            Field::Unknown(key) => {
                                if config.mode == fhirbolt_shared::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &["id", "extension", "modifierExtension", "other", "type"],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(PatientLink {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#other: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#other.unwrap_or(Default::default())
                        } else {
                            r#other.ok_or(serde::de::Error::missing_field("other"))?
                        },
                        r#type: if config.mode == fhirbolt_shared::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Demographics and other administrative information about an individual or animal receiving care or other health-related services.\n\nTracking patient is the center of the healthcare process."]
#[derive(Default, Debug, Clone)]
pub struct Patient {
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
    #[doc = "An identifier for this patient."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this patient record is in active use. \nMany systems use this property to mark as non-current patients, such as those that have not been seen for a period of time based on an organization's business rules.\n\nIt is often used to filter patient lists to exclude inactive patients\n\nDeceased patients may also be marked as inactive for the same reasons, but may be active for some time after death."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "A name associated with the individual."]
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail (e.g. a telephone number or an email address) by which the individual may be contacted."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Administrative Gender - the gender that the patient is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date of birth for the individual."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Indicates if the individual is deceased or not."]
    pub r#deceased: Option<PatientDeceased>,
    #[doc = "An address for the individual."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "This field contains a patient's most recent marital (civil) status."]
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates whether the patient is part of a multiple (boolean) or indicates the actual birth order (integer)."]
    pub r#multiple_birth: Option<PatientMultipleBirth>,
    #[doc = "Image of the patient."]
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    #[doc = "A contact party (e.g. guardian, partner, friend) for the patient."]
    pub r#contact: Vec<PatientContact>,
    #[doc = "A language which may be used to communicate with the patient about his or her health."]
    pub r#communication: Vec<PatientCommunication>,
    #[doc = "Patient's nominated care provider."]
    pub r#general_practitioner: Vec<Box<super::super::types::Reference>>,
    #[doc = "Organization that is the custodian of the patient record."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Link to another patient resource that concerns the same actual patient."]
    pub r#link: Vec<PatientLink>,
}
impl serde::ser::Serialize for Patient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Patient")?;
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
        if let Some(some) = self.r#active.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("active", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_active", &primitive_element)?;
            }
        }
        if !self.r#name.is_empty() {
            state.serialize_entry("name", &self.r#name)?;
        }
        if !self.r#telecom.is_empty() {
            state.serialize_entry("telecom", &self.r#telecom)?;
        }
        if let Some(some) = self.r#gender.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("gender", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_gender", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#birth_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("birthDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_birthDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#deceased.as_ref() {
            match some {
                PatientDeceased::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("deceasedBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_deceasedBoolean", &primitive_element)?;
                    }
                }
                PatientDeceased::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("deceasedDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_deceasedDateTime", &primitive_element)?;
                    }
                }
                PatientDeceased::Invalid => {
                    return Err(serde::ser::Error::custom("deceased is invalid"))
                }
            }
        }
        if !self.r#address.is_empty() {
            state.serialize_entry("address", &self.r#address)?;
        }
        if let Some(some) = self.r#marital_status.as_ref() {
            state.serialize_entry("maritalStatus", some)?;
        }
        if let Some(some) = self.r#multiple_birth.as_ref() {
            match some {
                PatientMultipleBirth::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("multipleBirthBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_multipleBirthBoolean", &primitive_element)?;
                    }
                }
                PatientMultipleBirth::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("multipleBirthInteger", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_multipleBirthInteger", &primitive_element)?;
                    }
                }
                PatientMultipleBirth::Invalid => {
                    return Err(serde::ser::Error::custom("multiple_birth is invalid"))
                }
            }
        }
        if !self.r#photo.is_empty() {
            state.serialize_entry("photo", &self.r#photo)?;
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if !self.r#communication.is_empty() {
            state.serialize_entry("communication", &self.r#communication)?;
        }
        if !self.r#general_practitioner.is_empty() {
            state.serialize_entry("generalPractitioner", &self.r#general_practitioner)?;
        }
        if let Some(some) = self.r#managing_organization.as_ref() {
            state.serialize_entry("managingOrganization", some)?;
        }
        if !self.r#link.is_empty() {
            state.serialize_entry("link", &self.r#link)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Patient {
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
            #[serde(rename = "active")]
            Active,
            #[serde(rename = "_active")]
            ActivePrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "gender")]
            Gender,
            #[serde(rename = "_gender")]
            GenderPrimitiveElement,
            #[serde(rename = "birthDate")]
            BirthDate,
            #[serde(rename = "_birthDate")]
            BirthDatePrimitiveElement,
            #[serde(rename = "deceasedBoolean")]
            DeceasedBoolean,
            #[serde(rename = "_deceasedBoolean")]
            DeceasedBooleanPrimitiveElement,
            #[serde(rename = "deceasedDateTime")]
            DeceasedDateTime,
            #[serde(rename = "_deceasedDateTime")]
            DeceasedDateTimePrimitiveElement,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "maritalStatus")]
            MaritalStatus,
            #[serde(rename = "multipleBirthBoolean")]
            MultipleBirthBoolean,
            #[serde(rename = "_multipleBirthBoolean")]
            MultipleBirthBooleanPrimitiveElement,
            #[serde(rename = "multipleBirthInteger")]
            MultipleBirthInteger,
            #[serde(rename = "_multipleBirthInteger")]
            MultipleBirthIntegerPrimitiveElement,
            #[serde(rename = "photo")]
            Photo,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "communication")]
            Communication,
            #[serde(rename = "generalPractitioner")]
            GeneralPractitioner,
            #[serde(rename = "managingOrganization")]
            ManagingOrganization,
            #[serde(rename = "link")]
            Link,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Patient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Patient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Patient, V::Error>
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
                let mut r#active: Option<super::super::types::Boolean> = None;
                let mut r#name: Option<Vec<Box<super::super::types::HumanName>>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#gender: Option<super::super::types::Code> = None;
                let mut r#birth_date: Option<super::super::types::Date> = None;
                let mut r#deceased: Option<PatientDeceased> = None;
                let mut r#address: Option<Vec<Box<super::super::types::Address>>> = None;
                let mut r#marital_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#multiple_birth: Option<PatientMultipleBirth> = None;
                let mut r#photo: Option<Vec<Box<super::super::types::Attachment>>> = None;
                let mut r#contact: Option<Vec<PatientContact>> = None;
                let mut r#communication: Option<Vec<PatientCommunication>> = None;
                let mut r#general_practitioner: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#managing_organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#link: Option<Vec<PatientLink>> = None;
                fhirbolt_shared::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Patient" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Patient",
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
                            Field::Active => {
                                let some = r#active.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("active"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ActivePrimitiveElement => {
                                let some = r#active.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_active"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Name => {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value()?);
                            }
                            Field::Telecom => {
                                if r#telecom.is_some() {
                                    return Err(serde::de::Error::duplicate_field("telecom"));
                                }
                                r#telecom = Some(map_access.next_value()?);
                            }
                            Field::Gender => {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gender"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::GenderPrimitiveElement => {
                                let some = r#gender.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_gender"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::BirthDate => {
                                let some = r#birth_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("birthDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::BirthDatePrimitiveElement => {
                                let some = r#birth_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_birthDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::DeceasedBoolean => {
                                let r#enum = r#deceased
                                    .get_or_insert(PatientDeceased::Boolean(Default::default()));
                                if let PatientDeceased::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deceasedBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("deceased[x]"));
                                }
                            }
                            Field::DeceasedBooleanPrimitiveElement => {
                                let r#enum = r#deceased
                                    .get_or_insert(PatientDeceased::Boolean(Default::default()));
                                if let PatientDeceased::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deceasedBoolean",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                                }
                            }
                            Field::DeceasedDateTime => {
                                let r#enum = r#deceased
                                    .get_or_insert(PatientDeceased::DateTime(Default::default()));
                                if let PatientDeceased::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "deceasedDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("deceased[x]"));
                                }
                            }
                            Field::DeceasedDateTimePrimitiveElement => {
                                let r#enum = r#deceased
                                    .get_or_insert(PatientDeceased::DateTime(Default::default()));
                                if let PatientDeceased::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_deceasedDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_deceased[x]"));
                                }
                            }
                            Field::Address => {
                                if r#address.is_some() {
                                    return Err(serde::de::Error::duplicate_field("address"));
                                }
                                r#address = Some(map_access.next_value()?);
                            }
                            Field::MaritalStatus => {
                                if r#marital_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maritalStatus"));
                                }
                                r#marital_status = Some(map_access.next_value()?);
                            }
                            Field::MultipleBirthBoolean => {
                                let r#enum = r#multiple_birth.get_or_insert(
                                    PatientMultipleBirth::Boolean(Default::default()),
                                );
                                if let PatientMultipleBirth::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "multipleBirthBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "multipleBirth[x]",
                                    ));
                                }
                            }
                            Field::MultipleBirthBooleanPrimitiveElement => {
                                let r#enum = r#multiple_birth.get_or_insert(
                                    PatientMultipleBirth::Boolean(Default::default()),
                                );
                                if let PatientMultipleBirth::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_multipleBirthBoolean",
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
                                        "_multipleBirth[x]",
                                    ));
                                }
                            }
                            Field::MultipleBirthInteger => {
                                let r#enum = r#multiple_birth.get_or_insert(
                                    PatientMultipleBirth::Integer(Default::default()),
                                );
                                if let PatientMultipleBirth::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "multipleBirthInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "multipleBirth[x]",
                                    ));
                                }
                            }
                            Field::MultipleBirthIntegerPrimitiveElement => {
                                let r#enum = r#multiple_birth.get_or_insert(
                                    PatientMultipleBirth::Integer(Default::default()),
                                );
                                if let PatientMultipleBirth::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_multipleBirthInteger",
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
                                        "_multipleBirth[x]",
                                    ));
                                }
                            }
                            Field::Photo => {
                                if r#photo.is_some() {
                                    return Err(serde::de::Error::duplicate_field("photo"));
                                }
                                r#photo = Some(map_access.next_value()?);
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
                            }
                            Field::Communication => {
                                if r#communication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("communication"));
                                }
                                r#communication = Some(map_access.next_value()?);
                            }
                            Field::GeneralPractitioner => {
                                if r#general_practitioner.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "generalPractitioner",
                                    ));
                                }
                                r#general_practitioner = Some(map_access.next_value()?);
                            }
                            Field::ManagingOrganization => {
                                if r#managing_organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "managingOrganization",
                                    ));
                                }
                                r#managing_organization = Some(map_access.next_value()?);
                            }
                            Field::Link => {
                                if r#link.is_some() {
                                    return Err(serde::de::Error::duplicate_field("link"));
                                }
                                r#link = Some(map_access.next_value()?);
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
                                            "active",
                                            "name",
                                            "telecom",
                                            "gender",
                                            "birthDate",
                                            "deceasedBoolean",
                                            "deceasedDateTime",
                                            "address",
                                            "maritalStatus",
                                            "multipleBirthBoolean",
                                            "multipleBirthInteger",
                                            "photo",
                                            "contact",
                                            "communication",
                                            "generalPractitioner",
                                            "managingOrganization",
                                            "link",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Patient {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#active,
                        r#name: r#name.unwrap_or(vec![]),
                        r#telecom: r#telecom.unwrap_or(vec![]),
                        r#gender,
                        r#birth_date,
                        r#deceased,
                        r#address: r#address.unwrap_or(vec![]),
                        r#marital_status,
                        r#multiple_birth,
                        r#photo: r#photo.unwrap_or(vec![]),
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#communication: r#communication.unwrap_or(vec![]),
                        r#general_practitioner: r#general_practitioner.unwrap_or(vec![]),
                        r#managing_organization,
                        r#link: r#link.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
