// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Citation Resource or display of suggested citation for this report."]
#[derive(Debug, Clone)]
pub enum EvidenceReportCiteAs {
    Reference(Box<super::super::types::Reference>),
    Markdown(Box<super::super::types::Markdown>),
    Invalid,
}
impl Default for EvidenceReportCiteAs {
    fn default() -> EvidenceReportCiteAs {
        EvidenceReportCiteAs::Invalid
    }
}
#[doc = "Characteristic value."]
#[derive(Debug, Clone)]
pub enum EvidenceReportSubjectCharacteristicValue {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for EvidenceReportSubjectCharacteristicValue {
    fn default() -> EvidenceReportSubjectCharacteristicValue {
        EvidenceReportSubjectCharacteristicValue::Invalid
    }
}
#[doc = "The target composition/document of this relationship."]
#[derive(Debug, Clone)]
pub enum EvidenceReportRelatesToTarget {
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for EvidenceReportRelatesToTarget {
    fn default() -> EvidenceReportRelatesToTarget {
        EvidenceReportRelatesToTarget::Invalid
    }
}
#[doc = "Characteristic."]
#[derive(Default, Debug, Clone)]
pub struct EvidenceReportSubjectCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Characteristic code."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Characteristic value."]
    pub r#value: EvidenceReportSubjectCharacteristicValue,
    #[doc = "Is used to express not the characteristic."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Timeframe for the characteristic."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl crate::AnyResource for EvidenceReportSubjectCharacteristic {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for EvidenceReportSubjectCharacteristic {
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
        state.serialize_entry("code", &self.r#code)?;
        match self.r#value {
            EvidenceReportSubjectCharacteristicValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            EvidenceReportSubjectCharacteristicValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            EvidenceReportSubjectCharacteristicValue::Boolean(ref value) => {
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
            EvidenceReportSubjectCharacteristicValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            EvidenceReportSubjectCharacteristicValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            EvidenceReportSubjectCharacteristicValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        if let Some(some) = self.r#exclude.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("exclude", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_exclude", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceReportSubjectCharacteristic {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "exclude")]
            Exclude,
            #[serde(rename = "_exclude")]
            ExcludePrimitiveElement,
            #[serde(rename = "period")]
            Period,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceReportSubjectCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSubjectCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<EvidenceReportSubjectCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<EvidenceReportSubjectCharacteristicValue> = None;
                let mut r#exclude: Option<super::super::types::Boolean> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value =
                                    Some(EvidenceReportSubjectCharacteristicValue::Reference(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(
                                    EvidenceReportSubjectCharacteristicValue::CodeableConcept(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::ValueBoolean => {
                                let r#enum = r#value.get_or_insert(
                                    EvidenceReportSubjectCharacteristicValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let EvidenceReportSubjectCharacteristicValue::Boolean(variant) =
                                    r#enum
                                {
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
                                    EvidenceReportSubjectCharacteristicValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let EvidenceReportSubjectCharacteristicValue::Boolean(variant) =
                                    r#enum
                                {
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
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(EvidenceReportSubjectCharacteristicValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(EvidenceReportSubjectCharacteristicValue::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Exclude => {
                                let some = r#exclude.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exclude"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExcludePrimitiveElement => {
                                let some = r#exclude.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_exclude"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "code",
                                        "valueReference",
                                        "valueCodeableConcept",
                                        "valueBoolean",
                                        "valueQuantity",
                                        "valueRange",
                                        "exclude",
                                        "period",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceReportSubjectCharacteristic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#value: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                        r#exclude,
                        r#period,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specifies the subject or focus of the report. Answers \"What is this report about?\"."]
#[derive(Default, Debug, Clone)]
pub struct EvidenceReportSubject {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Characteristic."]
    pub r#characteristic: Vec<EvidenceReportSubjectCharacteristic>,
    #[doc = "Used for general notes and annotations not coded elsewhere."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for EvidenceReportSubject {
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
        if !self.r#characteristic.is_empty() {
            state.serialize_entry("characteristic", &self.r#characteristic)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceReportSubject {
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
            #[serde(rename = "characteristic")]
            Characteristic,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceReportSubject;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSubject")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceReportSubject, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#characteristic: Option<Vec<EvidenceReportSubjectCharacteristic>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Characteristic => {
                                if r#characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristic",
                                    ));
                                }
                                r#characteristic = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "characteristic",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceReportSubject {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#characteristic: r#characteristic.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Relationships that this composition has with other compositions or documents that already exist."]
#[derive(Default, Debug, Clone)]
pub struct EvidenceReportRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship that this composition has with anther composition or document."]
    pub r#code: super::super::types::Code,
    #[doc = "The target composition/document of this relationship."]
    pub r#target: EvidenceReportRelatesToTarget,
}
impl serde::ser::Serialize for EvidenceReportRelatesTo {
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
        if let Some(some) = self.r#code.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("code", &some)?;
        }
        if self.r#code.id.is_some() || !self.r#code.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#code.id,
                extension: &self.r#code.extension,
            };
            state.serialize_entry("_code", &primitive_element)?;
        }
        match self.r#target {
            EvidenceReportRelatesToTarget::Identifier(ref value) => {
                state.serialize_entry("targetIdentifier", value)?;
            }
            EvidenceReportRelatesToTarget::Reference(ref value) => {
                state.serialize_entry("targetReference", value)?;
            }
            EvidenceReportRelatesToTarget::Invalid => {
                return Err(serde::ser::Error::custom("target is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceReportRelatesTo {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "_code")]
            CodePrimitiveElement,
            #[serde(rename = "targetIdentifier")]
            TargetIdentifier,
            #[serde(rename = "targetReference")]
            TargetReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceReportRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportRelatesTo")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceReportRelatesTo, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<super::super::types::Code> = None;
                let mut r#target: Option<EvidenceReportRelatesToTarget> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Code => {
                                let some = r#code.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CodePrimitiveElement => {
                                let some = r#code.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_code"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::TargetIdentifier => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetIdentifier",
                                    ));
                                }
                                r#target = Some(EvidenceReportRelatesToTarget::Identifier(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TargetReference => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "targetReference",
                                    ));
                                }
                                r#target = Some(EvidenceReportRelatesToTarget::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "code",
                                        "targetIdentifier",
                                        "targetReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceReportRelatesTo {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#target: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#target.unwrap_or(Default::default())
                        } else {
                            r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The root of the sections that make up the composition."]
#[derive(Default, Debug, Clone)]
pub struct EvidenceReportSection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The label for this particular section.  This will be part of the rendered content for the document, and is often used to build a table of contents."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A code identifying the kind of content contained within the section. This should be consistent with the section title."]
    pub r#focus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A definitional Resource identifying the kind of content contained within the section. This should be consistent with the section title."]
    pub r#focus_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies who is responsible for the information in this section, not necessarily who typed it in."]
    pub r#author: Vec<Box<super::super::types::Reference>>,
    #[doc = "A human-readable narrative that contains the attested content of the section, used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is peferred to contain sufficient detail to make it acceptable for a human to just read the narrative."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "How the entry list was prepared - whether it is a working list that is suitable for being maintained on an ongoing basis, or if it represents a snapshot of a list of items from another source, or whether it is a prepared list where items may be marked as added, modified or deleted."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "Specifies the order applied to the items in the section entries."]
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies any type of classification of the evidence report."]
    pub r#entry_classifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the actual resource from which the narrative in the section is derived."]
    pub r#entry_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Quantity as content."]
    pub r#entry_quantity: Vec<Box<super::super::types::Quantity>>,
    #[doc = "If the section is empty, why the list is empty. An empty section typically has some text explaining the empty reason."]
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A nested sub-section within this section."]
    pub r#section: Vec<EvidenceReportSection>,
}
impl serde::ser::Serialize for EvidenceReportSection {
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
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("title", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#focus.as_ref() {
            state.serialize_entry("focus", some)?;
        }
        if let Some(some) = self.r#focus_reference.as_ref() {
            state.serialize_entry("focusReference", some)?;
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if let Some(some) = self.r#mode.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("mode", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_mode", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#ordered_by.as_ref() {
            state.serialize_entry("orderedBy", some)?;
        }
        if !self.r#entry_classifier.is_empty() {
            state.serialize_entry("entryClassifier", &self.r#entry_classifier)?;
        }
        if !self.r#entry_reference.is_empty() {
            state.serialize_entry("entryReference", &self.r#entry_reference)?;
        }
        if !self.r#entry_quantity.is_empty() {
            state.serialize_entry("entryQuantity", &self.r#entry_quantity)?;
        }
        if let Some(some) = self.r#empty_reason.as_ref() {
            state.serialize_entry("emptyReason", some)?;
        }
        if !self.r#section.is_empty() {
            state.serialize_entry("section", &self.r#section)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceReportSection {
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
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "focus")]
            Focus,
            #[serde(rename = "focusReference")]
            FocusReference,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "orderedBy")]
            OrderedBy,
            #[serde(rename = "entryClassifier")]
            EntryClassifier,
            #[serde(rename = "entryReference")]
            EntryReference,
            #[serde(rename = "entryQuantity")]
            EntryQuantity,
            #[serde(rename = "emptyReason")]
            EmptyReason,
            #[serde(rename = "section")]
            Section,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceReportSection;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSection")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceReportSection, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#focus: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#focus_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#ordered_by: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#entry_classifier: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#entry_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#entry_quantity: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#empty_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#section: Option<Vec<EvidenceReportSection>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Title => {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TitlePrimitiveElement => {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Focus => {
                                if r#focus.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focus"));
                                }
                                r#focus = Some(map_access.next_value()?);
                            }
                            Field::FocusReference => {
                                if r#focus_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "focusReference",
                                    ));
                                }
                                r#focus_reference = Some(map_access.next_value()?);
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Mode => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ModePrimitiveElement => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::OrderedBy => {
                                if r#ordered_by.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderedBy"));
                                }
                                r#ordered_by = Some(map_access.next_value()?);
                            }
                            Field::EntryClassifier => {
                                if r#entry_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entryClassifier",
                                    ));
                                }
                                r#entry_classifier = Some(map_access.next_value()?);
                            }
                            Field::EntryReference => {
                                if r#entry_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entryReference",
                                    ));
                                }
                                r#entry_reference = Some(map_access.next_value()?);
                            }
                            Field::EntryQuantity => {
                                if r#entry_quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entryQuantity"));
                                }
                                r#entry_quantity = Some(map_access.next_value()?);
                            }
                            Field::EmptyReason => {
                                if r#empty_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("emptyReason"));
                                }
                                r#empty_reason = Some(map_access.next_value()?);
                            }
                            Field::Section => {
                                if r#section.is_some() {
                                    return Err(serde::de::Error::duplicate_field("section"));
                                }
                                r#section = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "title",
                                        "focus",
                                        "focusReference",
                                        "author",
                                        "text",
                                        "mode",
                                        "orderedBy",
                                        "entryClassifier",
                                        "entryReference",
                                        "entryQuantity",
                                        "emptyReason",
                                        "section",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceReportSection {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#title,
                        r#focus,
                        r#focus_reference,
                        r#author: r#author.unwrap_or(vec![]),
                        r#text,
                        r#mode,
                        r#ordered_by,
                        r#entry_classifier: r#entry_classifier.unwrap_or(vec![]),
                        r#entry_reference: r#entry_reference.unwrap_or(vec![]),
                        r#entry_quantity: r#entry_quantity.unwrap_or(vec![]),
                        r#empty_reason,
                        r#section: r#section.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The EvidenceReport Resource is a specialized container for a collection of resources and codable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts."]
#[derive(Default, Debug, Clone)]
pub struct EvidenceReport {
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
    #[doc = "An absolute URI that is used to identify this EvidenceReport when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence report instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A formal identifier that is used to identify this EvidenceReport when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A formal identifier that is used to identify things closely related to this EvidenceReport."]
    pub r#related_identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Citation Resource or display of suggested citation for this report."]
    pub r#cite_as: Option<EvidenceReportCiteAs>,
    #[doc = "Specifies the kind of report, such as grouping of classifiers, search results, or human-compiled expression."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used for footnotes and annotations."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Link, description or reference to artifact associated with the report."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Specifies the subject or focus of the report. Answers \"What is this report about?\"."]
    pub r#subject: EvidenceReportSubject,
    #[doc = "The name of the organization or individual that published the evidence report."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individiual, organization, or device responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Relationships that this composition has with other compositions or documents that already exist."]
    pub r#relates_to: Vec<EvidenceReportRelatesTo>,
    #[doc = "The root of the sections that make up the composition."]
    pub r#section: Vec<EvidenceReportSection>,
}
impl serde::ser::Serialize for EvidenceReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "EvidenceReport")?;
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
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("url", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
            }
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
        if !self.r#use_context.is_empty() {
            state.serialize_entry("useContext", &self.r#use_context)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#related_identifier.is_empty() {
            state.serialize_entry("relatedIdentifier", &self.r#related_identifier)?;
        }
        if let Some(some) = self.r#cite_as.as_ref() {
            match some {
                EvidenceReportCiteAs::Reference(ref value) => {
                    state.serialize_entry("citeAsReference", value)?;
                }
                EvidenceReportCiteAs::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("citeAsMarkdown", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_citeAsMarkdown", &primitive_element)?;
                    }
                }
                EvidenceReportCiteAs::Invalid => {
                    return Err(serde::ser::Error::custom("cite_as is invalid"))
                }
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#related_artifact.is_empty() {
            state.serialize_entry("relatedArtifact", &self.r#related_artifact)?;
        }
        state.serialize_entry("subject", &self.r#subject)?;
        if let Some(some) = self.r#publisher.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("publisher", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_publisher", &primitive_element)?;
            }
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if !self.r#editor.is_empty() {
            state.serialize_entry("editor", &self.r#editor)?;
        }
        if !self.r#reviewer.is_empty() {
            state.serialize_entry("reviewer", &self.r#reviewer)?;
        }
        if !self.r#endorser.is_empty() {
            state.serialize_entry("endorser", &self.r#endorser)?;
        }
        if !self.r#relates_to.is_empty() {
            state.serialize_entry("relatesTo", &self.r#relates_to)?;
        }
        if !self.r#section.is_empty() {
            state.serialize_entry("section", &self.r#section)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for EvidenceReport {
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
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "relatedIdentifier")]
            RelatedIdentifier,
            #[serde(rename = "citeAsReference")]
            CiteAsReference,
            #[serde(rename = "citeAsMarkdown")]
            CiteAsMarkdown,
            #[serde(rename = "_citeAsMarkdown")]
            CiteAsMarkdownPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "relatedArtifact")]
            RelatedArtifact,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "editor")]
            Editor,
            #[serde(rename = "reviewer")]
            Reviewer,
            #[serde(rename = "endorser")]
            Endorser,
            #[serde(rename = "relatesTo")]
            RelatesTo,
            #[serde(rename = "section")]
            Section,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EvidenceReport;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReport")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<EvidenceReport, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#related_identifier: Option<Vec<Box<super::super::types::Identifier>>> =
                    None;
                let mut r#cite_as: Option<EvidenceReportCiteAs> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#related_artifact: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#subject: Option<EvidenceReportSubject> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#editor: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#reviewer: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#endorser: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#relates_to: Option<Vec<EvidenceReportRelatesTo>> = None;
                let mut r#section: Option<Vec<EvidenceReportSection>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "EvidenceReport" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"EvidenceReport",
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
                            Field::Url => {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UrlPrimitiveElement => {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                            Field::UseContext => {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::RelatedIdentifier => {
                                if r#related_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedIdentifier",
                                    ));
                                }
                                r#related_identifier = Some(map_access.next_value()?);
                            }
                            Field::CiteAsReference => {
                                if r#cite_as.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "citeAsReference",
                                    ));
                                }
                                r#cite_as =
                                    Some(EvidenceReportCiteAs::Reference(map_access.next_value()?));
                            }
                            Field::CiteAsMarkdown => {
                                let r#enum = r#cite_as.get_or_insert(
                                    EvidenceReportCiteAs::Markdown(Default::default()),
                                );
                                if let EvidenceReportCiteAs::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "citeAsMarkdown",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("citeAs[x]"));
                                }
                            }
                            Field::CiteAsMarkdownPrimitiveElement => {
                                let r#enum = r#cite_as.get_or_insert(
                                    EvidenceReportCiteAs::Markdown(Default::default()),
                                );
                                if let EvidenceReportCiteAs::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_citeAsMarkdown",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_citeAs[x]"));
                                }
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::RelatedArtifact => {
                                if r#related_artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedArtifact",
                                    ));
                                }
                                r#related_artifact = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Publisher => {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PublisherPrimitiveElement => {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Contact => {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some(map_access.next_value()?);
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Editor => {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                r#editor = Some(map_access.next_value()?);
                            }
                            Field::Reviewer => {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                r#reviewer = Some(map_access.next_value()?);
                            }
                            Field::Endorser => {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                r#endorser = Some(map_access.next_value()?);
                            }
                            Field::RelatesTo => {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to = Some(map_access.next_value()?);
                            }
                            Field::Section => {
                                if r#section.is_some() {
                                    return Err(serde::de::Error::duplicate_field("section"));
                                }
                                r#section = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
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
                                        "url",
                                        "status",
                                        "useContext",
                                        "identifier",
                                        "relatedIdentifier",
                                        "citeAsReference",
                                        "citeAsMarkdown",
                                        "type",
                                        "note",
                                        "relatedArtifact",
                                        "subject",
                                        "publisher",
                                        "contact",
                                        "author",
                                        "editor",
                                        "reviewer",
                                        "endorser",
                                        "relatesTo",
                                        "section",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(EvidenceReport {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url,
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#related_identifier: r#related_identifier.unwrap_or(vec![]),
                        r#cite_as,
                        r#type,
                        r#note: r#note.unwrap_or(vec![]),
                        r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                        r#subject: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#author: r#author.unwrap_or(vec![]),
                        r#editor: r#editor.unwrap_or(vec![]),
                        r#reviewer: r#reviewer.unwrap_or(vec![]),
                        r#endorser: r#endorser.unwrap_or(vec![]),
                        r#relates_to: r#relates_to.unwrap_or(vec![]),
                        r#section: r#section.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
