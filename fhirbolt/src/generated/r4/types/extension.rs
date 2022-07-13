// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ExtensionValue {
    Base64Binary(Box<super::super::types::Base64Binary>),
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    Code(Box<super::super::types::Code>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
    Id(Box<super::super::types::Id>),
    Instant(Box<super::super::types::Instant>),
    Integer(Box<super::super::types::Integer>),
    Markdown(Box<super::super::types::Markdown>),
    Oid(Box<super::super::types::Oid>),
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
    Time(Box<super::super::types::Time>),
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    Uri(Box<super::super::types::Uri>),
    Url(Box<super::super::types::Url>),
    Uuid(Box<super::super::types::Uuid>),
    Address(Box<super::super::types::Address>),
    Age(Box<super::super::types::Age>),
    Annotation(Box<super::super::types::Annotation>),
    Attachment(Box<super::super::types::Attachment>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Coding(Box<super::super::types::Coding>),
    ContactPoint(Box<super::super::types::ContactPoint>),
    Count(Box<super::super::types::Count>),
    Distance(Box<super::super::types::Distance>),
    Duration(Box<super::super::types::Duration>),
    HumanName(Box<super::super::types::HumanName>),
    Identifier(Box<super::super::types::Identifier>),
    Money(Box<super::super::types::Money>),
    Period(Box<super::super::types::Period>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    Reference(Box<super::super::types::Reference>),
    SampledData(Box<super::super::types::SampledData>),
    Signature(Box<super::super::types::Signature>),
    Timing(Box<super::super::types::Timing>),
    ContactDetail(Box<super::super::types::ContactDetail>),
    Contributor(Box<super::super::types::Contributor>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    Expression(Box<super::super::types::Expression>),
    ParameterDefinition(Box<super::super::types::ParameterDefinition>),
    RelatedArtifact(Box<super::super::types::RelatedArtifact>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
    UsageContext(Box<super::super::types::UsageContext>),
    Dosage(Box<super::super::types::Dosage>),
    Meta(Box<super::super::types::Meta>),
}
impl Default for ExtensionValue {
    fn default() -> ExtensionValue {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct Extension {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: std::string::String,
    pub r#value: Option<ExtensionValue>,
}
impl serde::ser::Serialize for Extension {
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
        state.serialize_entry("url", &self.r#url)?;
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ExtensionValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                    }
                }
                ExtensionValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ExtensionValue::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueCanonical", &primitive_element)?;
                    }
                }
                ExtensionValue::Code(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueCode", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueCode", &primitive_element)?;
                    }
                }
                ExtensionValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDate", &primitive_element)?;
                    }
                }
                ExtensionValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDateTime", &primitive_element)?;
                    }
                }
                ExtensionValue::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDecimal", &primitive_element)?;
                    }
                }
                ExtensionValue::Id(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueId", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueId", &primitive_element)?;
                    }
                }
                ExtensionValue::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInstant", &primitive_element)?;
                    }
                }
                ExtensionValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInteger", &primitive_element)?;
                    }
                }
                ExtensionValue::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueMarkdown", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueMarkdown", &primitive_element)?;
                    }
                }
                ExtensionValue::Oid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueOid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueOid", &primitive_element)?;
                    }
                }
                ExtensionValue::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valuePositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valuePositiveInt", &primitive_element)?;
                    }
                }
                ExtensionValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ExtensionValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueTime", &primitive_element)?;
                    }
                }
                ExtensionValue::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUnsignedInt", &primitive_element)?;
                    }
                }
                ExtensionValue::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUri", &primitive_element)?;
                    }
                }
                ExtensionValue::Url(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUrl", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUrl", &primitive_element)?;
                    }
                }
                ExtensionValue::Uuid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUuid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUuid", &primitive_element)?;
                    }
                }
                ExtensionValue::Address(ref value) => {
                    state.serialize_entry("valueAddress", value)?;
                }
                ExtensionValue::Age(ref value) => {
                    state.serialize_entry("valueAge", value)?;
                }
                ExtensionValue::Annotation(ref value) => {
                    state.serialize_entry("valueAnnotation", value)?;
                }
                ExtensionValue::Attachment(ref value) => {
                    state.serialize_entry("valueAttachment", value)?;
                }
                ExtensionValue::CodeableConcept(ref value) => {
                    state.serialize_entry("valueCodeableConcept", value)?;
                }
                ExtensionValue::Coding(ref value) => {
                    state.serialize_entry("valueCoding", value)?;
                }
                ExtensionValue::ContactPoint(ref value) => {
                    state.serialize_entry("valueContactPoint", value)?;
                }
                ExtensionValue::Count(ref value) => {
                    state.serialize_entry("valueCount", value)?;
                }
                ExtensionValue::Distance(ref value) => {
                    state.serialize_entry("valueDistance", value)?;
                }
                ExtensionValue::Duration(ref value) => {
                    state.serialize_entry("valueDuration", value)?;
                }
                ExtensionValue::HumanName(ref value) => {
                    state.serialize_entry("valueHumanName", value)?;
                }
                ExtensionValue::Identifier(ref value) => {
                    state.serialize_entry("valueIdentifier", value)?;
                }
                ExtensionValue::Money(ref value) => {
                    state.serialize_entry("valueMoney", value)?;
                }
                ExtensionValue::Period(ref value) => {
                    state.serialize_entry("valuePeriod", value)?;
                }
                ExtensionValue::Quantity(ref value) => {
                    state.serialize_entry("valueQuantity", value)?;
                }
                ExtensionValue::Range(ref value) => {
                    state.serialize_entry("valueRange", value)?;
                }
                ExtensionValue::Ratio(ref value) => {
                    state.serialize_entry("valueRatio", value)?;
                }
                ExtensionValue::Reference(ref value) => {
                    state.serialize_entry("valueReference", value)?;
                }
                ExtensionValue::SampledData(ref value) => {
                    state.serialize_entry("valueSampledData", value)?;
                }
                ExtensionValue::Signature(ref value) => {
                    state.serialize_entry("valueSignature", value)?;
                }
                ExtensionValue::Timing(ref value) => {
                    state.serialize_entry("valueTiming", value)?;
                }
                ExtensionValue::ContactDetail(ref value) => {
                    state.serialize_entry("valueContactDetail", value)?;
                }
                ExtensionValue::Contributor(ref value) => {
                    state.serialize_entry("valueContributor", value)?;
                }
                ExtensionValue::DataRequirement(ref value) => {
                    state.serialize_entry("valueDataRequirement", value)?;
                }
                ExtensionValue::Expression(ref value) => {
                    state.serialize_entry("valueExpression", value)?;
                }
                ExtensionValue::ParameterDefinition(ref value) => {
                    state.serialize_entry("valueParameterDefinition", value)?;
                }
                ExtensionValue::RelatedArtifact(ref value) => {
                    state.serialize_entry("valueRelatedArtifact", value)?;
                }
                ExtensionValue::TriggerDefinition(ref value) => {
                    state.serialize_entry("valueTriggerDefinition", value)?;
                }
                ExtensionValue::UsageContext(ref value) => {
                    state.serialize_entry("valueUsageContext", value)?;
                }
                ExtensionValue::Dosage(ref value) => {
                    state.serialize_entry("valueDosage", value)?;
                }
                ExtensionValue::Meta(ref value) => {
                    state.serialize_entry("valueMeta", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Extension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Extension;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Extension")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Extension, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#url: Option<std::string::String> = None;
                let mut r#value: Option<ExtensionValue> = None;
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
                        "url" => {
                            if r#url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            r#url = Some(map_access.next_value()?);
                        }
                        "valueBase64Binary" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::Base64Binary(Default::default()));
                            if let ExtensionValue::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBase64Binary" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::Base64Binary(Default::default()));
                            if let ExtensionValue::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueBase64Binary",
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
                        "valueBoolean" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Boolean(Default::default()));
                            if let ExtensionValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Boolean(Default::default()));
                            if let ExtensionValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
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
                        "valueCanonical" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::Canonical(Default::default()));
                            if let ExtensionValue::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueCanonical" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::Canonical(Default::default()));
                            if let ExtensionValue::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueCanonical",
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
                        "valueCode" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Code(Default::default()));
                            if let ExtensionValue::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueCode" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Code(Default::default()));
                            if let ExtensionValue::Code(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueCode"));
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
                        "valueDate" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Date(Default::default()));
                            if let ExtensionValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDate" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Date(Default::default()));
                            if let ExtensionValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDate"));
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
                        "valueDateTime" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::DateTime(Default::default()));
                            if let ExtensionValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::DateTime(Default::default()));
                            if let ExtensionValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
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
                        "valueDecimal" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Decimal(Default::default()));
                            if let ExtensionValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDecimal" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Decimal(Default::default()));
                            if let ExtensionValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDecimal"));
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
                        "valueId" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Id(Default::default()));
                            if let ExtensionValue::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueId" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Id(Default::default()));
                            if let ExtensionValue::Id(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueId"));
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
                        "valueInstant" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Instant(Default::default()));
                            if let ExtensionValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInstant" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Instant(Default::default()));
                            if let ExtensionValue::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInstant"));
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
                        "valueInteger" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Integer(Default::default()));
                            if let ExtensionValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInteger" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Integer(Default::default()));
                            if let ExtensionValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
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
                        "valueMarkdown" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Markdown(Default::default()));
                            if let ExtensionValue::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueMarkdown" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Markdown(Default::default()));
                            if let ExtensionValue::Markdown(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueMarkdown",
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
                        "valueOid" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Oid(Default::default()));
                            if let ExtensionValue::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueOid" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Oid(Default::default()));
                            if let ExtensionValue::Oid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueOid"));
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
                        "valuePositiveInt" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::PositiveInt(Default::default()));
                            if let ExtensionValue::PositiveInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valuePositiveInt" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::PositiveInt(Default::default()));
                            if let ExtensionValue::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valuePositiveInt",
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
                        "valueString" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::String(Default::default()));
                            if let ExtensionValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::String(Default::default()));
                            if let ExtensionValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
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
                        "valueTime" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Time(Default::default()));
                            if let ExtensionValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueTime" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Time(Default::default()));
                            if let ExtensionValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
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
                        "valueUnsignedInt" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::UnsignedInt(Default::default()));
                            if let ExtensionValue::UnsignedInt(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUnsignedInt" => {
                            let r#enum = r#value
                                .get_or_insert(ExtensionValue::UnsignedInt(Default::default()));
                            if let ExtensionValue::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueUnsignedInt",
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
                        "valueUri" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Uri(Default::default()));
                            if let ExtensionValue::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUri" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Uri(Default::default()));
                            if let ExtensionValue::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUri"));
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
                        "valueUrl" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Url(Default::default()));
                            if let ExtensionValue::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUrl" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Url(Default::default()));
                            if let ExtensionValue::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUrl"));
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
                        "valueUuid" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Uuid(Default::default()));
                            if let ExtensionValue::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUuid" => {
                            let r#enum =
                                r#value.get_or_insert(ExtensionValue::Uuid(Default::default()));
                            if let ExtensionValue::Uuid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUuid"));
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
                        "valueAddress" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value = Some(ExtensionValue::Address(map_access.next_value()?));
                        }
                        "valueAge" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(ExtensionValue::Age(map_access.next_value()?));
                        }
                        "valueAnnotation" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some(ExtensionValue::Annotation(map_access.next_value()?));
                        }
                        "valueAttachment" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ExtensionValue::Attachment(map_access.next_value()?));
                        }
                        "valueCodeableConcept" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value =
                                Some(ExtensionValue::CodeableConcept(map_access.next_value()?));
                        }
                        "valueCoding" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(ExtensionValue::Coding(map_access.next_value()?));
                        }
                        "valueContactPoint" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some(ExtensionValue::ContactPoint(map_access.next_value()?));
                        }
                        "valueCount" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(ExtensionValue::Count(map_access.next_value()?));
                        }
                        "valueDistance" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value = Some(ExtensionValue::Distance(map_access.next_value()?));
                        }
                        "valueDuration" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value = Some(ExtensionValue::Duration(map_access.next_value()?));
                        }
                        "valueHumanName" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some(ExtensionValue::HumanName(map_access.next_value()?));
                        }
                        "valueIdentifier" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some(ExtensionValue::Identifier(map_access.next_value()?));
                        }
                        "valueMoney" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(ExtensionValue::Money(map_access.next_value()?));
                        }
                        "valuePeriod" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(ExtensionValue::Period(map_access.next_value()?));
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ExtensionValue::Quantity(map_access.next_value()?));
                        }
                        "valueRange" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(ExtensionValue::Range(map_access.next_value()?));
                        }
                        "valueRatio" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(ExtensionValue::Ratio(map_access.next_value()?));
                        }
                        "valueReference" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ExtensionValue::Reference(map_access.next_value()?));
                        }
                        "valueSampledData" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some(ExtensionValue::SampledData(map_access.next_value()?));
                        }
                        "valueSignature" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some(ExtensionValue::Signature(map_access.next_value()?));
                        }
                        "valueTiming" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value = Some(ExtensionValue::Timing(map_access.next_value()?));
                        }
                        "valueContactDetail" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some(ExtensionValue::ContactDetail(map_access.next_value()?));
                        }
                        "valueContributor" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some(ExtensionValue::Contributor(map_access.next_value()?));
                        }
                        "valueDataRequirement" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value =
                                Some(ExtensionValue::DataRequirement(map_access.next_value()?));
                        }
                        "valueExpression" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some(ExtensionValue::Expression(map_access.next_value()?));
                        }
                        "valueParameterDefinition" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some(ExtensionValue::ParameterDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        "valueRelatedArtifact" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value =
                                Some(ExtensionValue::RelatedArtifact(map_access.next_value()?));
                        }
                        "valueTriggerDefinition" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value =
                                Some(ExtensionValue::TriggerDefinition(map_access.next_value()?));
                        }
                        "valueUsageContext" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some(ExtensionValue::UsageContext(map_access.next_value()?));
                        }
                        "valueDosage" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value = Some(ExtensionValue::Dosage(map_access.next_value()?));
                        }
                        "valueMeta" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            r#value = Some(ExtensionValue::Meta(map_access.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "url", "value"],
                            ))
                        }
                    }
                }
                Ok(Extension {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#url: r#url.ok_or(serde::de::Error::missing_field("url"))?,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
