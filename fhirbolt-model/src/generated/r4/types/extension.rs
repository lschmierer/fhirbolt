// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "Value of extension - must be one of a constrained set of the data types (see [Extensibility](https://hl7.org/FHIR/extensibility.html)) for a list)."]
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
    Invalid,
}
impl Default for ExtensionValue {
    fn default() -> ExtensionValue {
        ExtensionValue::Invalid
    }
}
#[doc = "Base StructureDefinition for Extension Type: Optional Extension Element - found in all resources.\n\nThe ability to add extensions in a structured way is what keeps FHIR resources simple."]
#[derive(Default, Debug, Clone)]
pub struct Extension {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Source of the definition for the extension code - a logical name or a URL."]
    pub r#url: std::string::String,
    #[doc = "Value of extension - must be one of a constrained set of the data types (see [Extensibility](https://hl7.org/FHIR/extensibility.html)) for a list)."]
    pub r#value: Option<ExtensionValue>,
}
impl crate::AnyResource for Extension {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueBase64Binary", &some)?;
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
                ExtensionValue::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueCanonical", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueCode", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueDate", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueDateTime", &some)?;
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
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("valueDecimal", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueId", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueInstant", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueInteger", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueMarkdown", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueOid", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valuePositiveInt", &some)?;
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
                ExtensionValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueTime", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueUnsignedInt", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueUri", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueUrl", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("valueUuid", &some)?;
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
                ExtensionValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueCanonical")]
            ValueCanonical,
            #[serde(rename = "_valueCanonical")]
            ValueCanonicalPrimitiveElement,
            #[serde(rename = "valueCode")]
            ValueCode,
            #[serde(rename = "_valueCode")]
            ValueCodePrimitiveElement,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            #[serde(rename = "valueId")]
            ValueId,
            #[serde(rename = "_valueId")]
            ValueIdPrimitiveElement,
            #[serde(rename = "valueInstant")]
            ValueInstant,
            #[serde(rename = "_valueInstant")]
            ValueInstantPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueMarkdown")]
            ValueMarkdown,
            #[serde(rename = "_valueMarkdown")]
            ValueMarkdownPrimitiveElement,
            #[serde(rename = "valueOid")]
            ValueOid,
            #[serde(rename = "_valueOid")]
            ValueOidPrimitiveElement,
            #[serde(rename = "valuePositiveInt")]
            ValuePositiveInt,
            #[serde(rename = "_valuePositiveInt")]
            ValuePositiveIntPrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueUnsignedInt")]
            ValueUnsignedInt,
            #[serde(rename = "_valueUnsignedInt")]
            ValueUnsignedIntPrimitiveElement,
            #[serde(rename = "valueUri")]
            ValueUri,
            #[serde(rename = "_valueUri")]
            ValueUriPrimitiveElement,
            #[serde(rename = "valueUrl")]
            ValueUrl,
            #[serde(rename = "_valueUrl")]
            ValueUrlPrimitiveElement,
            #[serde(rename = "valueUuid")]
            ValueUuid,
            #[serde(rename = "_valueUuid")]
            ValueUuidPrimitiveElement,
            #[serde(rename = "valueAddress")]
            ValueAddress,
            #[serde(rename = "valueAge")]
            ValueAge,
            #[serde(rename = "valueAnnotation")]
            ValueAnnotation,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueContactPoint")]
            ValueContactPoint,
            #[serde(rename = "valueCount")]
            ValueCount,
            #[serde(rename = "valueDistance")]
            ValueDistance,
            #[serde(rename = "valueDuration")]
            ValueDuration,
            #[serde(rename = "valueHumanName")]
            ValueHumanName,
            #[serde(rename = "valueIdentifier")]
            ValueIdentifier,
            #[serde(rename = "valueMoney")]
            ValueMoney,
            #[serde(rename = "valuePeriod")]
            ValuePeriod,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueRatio")]
            ValueRatio,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "valueSampledData")]
            ValueSampledData,
            #[serde(rename = "valueSignature")]
            ValueSignature,
            #[serde(rename = "valueTiming")]
            ValueTiming,
            #[serde(rename = "valueContactDetail")]
            ValueContactDetail,
            #[serde(rename = "valueContributor")]
            ValueContributor,
            #[serde(rename = "valueDataRequirement")]
            ValueDataRequirement,
            #[serde(rename = "valueExpression")]
            ValueExpression,
            #[serde(rename = "valueParameterDefinition")]
            ValueParameterDefinition,
            #[serde(rename = "valueRelatedArtifact")]
            ValueRelatedArtifact,
            #[serde(rename = "valueTriggerDefinition")]
            ValueTriggerDefinition,
            #[serde(rename = "valueUsageContext")]
            ValueUsageContext,
            #[serde(rename = "valueDosage")]
            ValueDosage,
            #[serde(rename = "valueMeta")]
            ValueMeta,
            Unknown(String),
        }
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
                            Field::Url => {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                r#url = Some(map_access.next_value()?);
                            }
                            Field::ValueBase64Binary => {
                                let r#enum = r#value.get_or_insert(ExtensionValue::Base64Binary(
                                    Default::default(),
                                ));
                                if let ExtensionValue::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueBase64BinaryPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(ExtensionValue::Base64Binary(
                                    Default::default(),
                                ));
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
                            Field::ValueBoolean => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Boolean(Default::default()));
                                if let ExtensionValue::Boolean(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Boolean(Default::default()));
                                if let ExtensionValue::Boolean(variant) = r#enum {
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
                            Field::ValueCanonical => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Canonical(Default::default()));
                                if let ExtensionValue::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueCanonicalPrimitiveElement => {
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
                            Field::ValueCode => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Code(Default::default()));
                                if let ExtensionValue::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueCodePrimitiveElement => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Code(Default::default()));
                                if let ExtensionValue::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
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
                            Field::ValueDate => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Date(Default::default()));
                                if let ExtensionValue::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueDatePrimitiveElement => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Date(Default::default()));
                                if let ExtensionValue::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
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
                            Field::ValueDateTime => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::DateTime(Default::default()));
                                if let ExtensionValue::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::DateTime(Default::default()));
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
                            Field::ValueDecimal => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Decimal(Default::default()));
                                if let ExtensionValue::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueDecimalPrimitiveElement => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Decimal(Default::default()));
                                if let ExtensionValue::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
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
                            Field::ValueId => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Id(Default::default()));
                                if let ExtensionValue::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueIdPrimitiveElement => {
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
                            Field::ValueInstant => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Instant(Default::default()));
                                if let ExtensionValue::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueInstantPrimitiveElement => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Instant(Default::default()));
                                if let ExtensionValue::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
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
                            Field::ValueInteger => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Integer(Default::default()));
                                if let ExtensionValue::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Integer(Default::default()));
                                if let ExtensionValue::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
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
                            Field::ValueMarkdown => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Markdown(Default::default()));
                                if let ExtensionValue::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueMarkdownPrimitiveElement => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::Markdown(Default::default()));
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
                            Field::ValueOid => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Oid(Default::default()));
                                if let ExtensionValue::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueOidPrimitiveElement => {
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
                            Field::ValuePositiveInt => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::PositiveInt(Default::default()));
                                if let ExtensionValue::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valuePositiveInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValuePositiveIntPrimitiveElement => {
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
                            Field::ValueString => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::String(Default::default()));
                                if let ExtensionValue::String(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::String(Default::default()));
                                if let ExtensionValue::String(variant) = r#enum {
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
                            Field::ValueTime => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Time(Default::default()));
                                if let ExtensionValue::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueTimePrimitiveElement => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Time(Default::default()));
                                if let ExtensionValue::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
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
                            Field::ValueUnsignedInt => {
                                let r#enum = r#value
                                    .get_or_insert(ExtensionValue::UnsignedInt(Default::default()));
                                if let ExtensionValue::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueUnsignedInt",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueUnsignedIntPrimitiveElement => {
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
                            Field::ValueUri => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Uri(Default::default()));
                                if let ExtensionValue::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueUriPrimitiveElement => {
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
                            Field::ValueUrl => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Url(Default::default()));
                                if let ExtensionValue::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueUrlPrimitiveElement => {
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
                            Field::ValueUuid => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Uuid(Default::default()));
                                if let ExtensionValue::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            }
                            Field::ValueUuidPrimitiveElement => {
                                let r#enum =
                                    r#value.get_or_insert(ExtensionValue::Uuid(Default::default()));
                                if let ExtensionValue::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
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
                            Field::ValueAddress => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAddress"));
                                }
                                r#value = Some(ExtensionValue::Address(map_access.next_value()?));
                            }
                            Field::ValueAge => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAge"));
                                }
                                r#value = Some(ExtensionValue::Age(map_access.next_value()?));
                            }
                            Field::ValueAnnotation => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAnnotation",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::Annotation(map_access.next_value()?));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::Attachment(map_access.next_value()?));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::CodeableConcept(map_access.next_value()?));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(ExtensionValue::Coding(map_access.next_value()?));
                            }
                            Field::ValueContactPoint => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactPoint",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::ContactPoint(map_access.next_value()?));
                            }
                            Field::ValueCount => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCount"));
                                }
                                r#value = Some(ExtensionValue::Count(map_access.next_value()?));
                            }
                            Field::ValueDistance => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDistance"));
                                }
                                r#value = Some(ExtensionValue::Distance(map_access.next_value()?));
                            }
                            Field::ValueDuration => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDuration"));
                                }
                                r#value = Some(ExtensionValue::Duration(map_access.next_value()?));
                            }
                            Field::ValueHumanName => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueHumanName",
                                    ));
                                }
                                r#value = Some(ExtensionValue::HumanName(map_access.next_value()?));
                            }
                            Field::ValueIdentifier => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueIdentifier",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::Identifier(map_access.next_value()?));
                            }
                            Field::ValueMoney => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMoney"));
                                }
                                r#value = Some(ExtensionValue::Money(map_access.next_value()?));
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(ExtensionValue::Period(map_access.next_value()?));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ExtensionValue::Quantity(map_access.next_value()?));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(ExtensionValue::Range(map_access.next_value()?));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value = Some(ExtensionValue::Ratio(map_access.next_value()?));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(ExtensionValue::Reference(map_access.next_value()?));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::SampledData(map_access.next_value()?));
                            }
                            Field::ValueSignature => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSignature",
                                    ));
                                }
                                r#value = Some(ExtensionValue::Signature(map_access.next_value()?));
                            }
                            Field::ValueTiming => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTiming"));
                                }
                                r#value = Some(ExtensionValue::Timing(map_access.next_value()?));
                            }
                            Field::ValueContactDetail => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactDetail",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::ContactDetail(map_access.next_value()?));
                            }
                            Field::ValueContributor => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContributor",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::Contributor(map_access.next_value()?));
                            }
                            Field::ValueDataRequirement => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueDataRequirement",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::DataRequirement(map_access.next_value()?));
                            }
                            Field::ValueExpression => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueExpression",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::Expression(map_access.next_value()?));
                            }
                            Field::ValueParameterDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueParameterDefinition",
                                    ));
                                }
                                r#value = Some(ExtensionValue::ParameterDefinition(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRelatedArtifact => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueRelatedArtifact",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::RelatedArtifact(map_access.next_value()?));
                            }
                            Field::ValueTriggerDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueTriggerDefinition",
                                    ));
                                }
                                r#value = Some(ExtensionValue::TriggerDefinition(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueUsageContext => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUsageContext",
                                    ));
                                }
                                r#value =
                                    Some(ExtensionValue::UsageContext(map_access.next_value()?));
                            }
                            Field::ValueDosage => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDosage"));
                                }
                                r#value = Some(ExtensionValue::Dosage(map_access.next_value()?));
                            }
                            Field::ValueMeta => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMeta"));
                                }
                                r#value = Some(ExtensionValue::Meta(map_access.next_value()?));
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "url",
                                        "valueBase64Binary",
                                        "valueBoolean",
                                        "valueCanonical",
                                        "valueCode",
                                        "valueDate",
                                        "valueDateTime",
                                        "valueDecimal",
                                        "valueId",
                                        "valueInstant",
                                        "valueInteger",
                                        "valueMarkdown",
                                        "valueOid",
                                        "valuePositiveInt",
                                        "valueString",
                                        "valueTime",
                                        "valueUnsignedInt",
                                        "valueUri",
                                        "valueUrl",
                                        "valueUuid",
                                        "valueAddress",
                                        "valueAge",
                                        "valueAnnotation",
                                        "valueAttachment",
                                        "valueCodeableConcept",
                                        "valueCoding",
                                        "valueContactPoint",
                                        "valueCount",
                                        "valueDistance",
                                        "valueDuration",
                                        "valueHumanName",
                                        "valueIdentifier",
                                        "valueMoney",
                                        "valuePeriod",
                                        "valueQuantity",
                                        "valueRange",
                                        "valueRatio",
                                        "valueReference",
                                        "valueSampledData",
                                        "valueSignature",
                                        "valueTiming",
                                        "valueContactDetail",
                                        "valueContributor",
                                        "valueDataRequirement",
                                        "valueExpression",
                                        "valueParameterDefinition",
                                        "valueRelatedArtifact",
                                        "valueTriggerDefinition",
                                        "valueUsageContext",
                                        "valueDosage",
                                        "valueMeta",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Extension {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#url: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#value,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
