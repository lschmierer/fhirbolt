// Generated on 2023-04-19 by fhirbolt-codegen v0.3.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::types::Extension>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        state.serialize_entry("url", &self.value.r#url)?;
        if let Some(some) = self.value.r#value.as_ref() {
            match some {
                fhirbolt_model::r4b::types::ExtensionValue::Base64Binary(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBase64Binary", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBase64Binary", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueBase64Binary", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Canonical(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueCanonical", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueCanonical", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Code(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueCode", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueCode", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueCode", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDate", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueDate", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = some.parse::<serde_json::Number>().map_err(|_| {
                                serde::ser::Error::custom("error serializing decimal")
                            })?;
                            state.serialize_entry("valueDecimal", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueDecimal", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Id(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueId", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueId", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueId", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueInstant", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueInstant", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueInteger", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueInteger", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Markdown(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueMarkdown", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueMarkdown", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueMarkdown", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Oid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueOid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueOid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueOid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valuePositiveInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valuePositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valuePositiveInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueString", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueTime", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUnsignedInt", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueUnsignedInt", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Uri(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUri", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUri", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUrl", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUrl", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Uuid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("valueUuid", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUuid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUuid", ctx))?;
                    }
                }
                fhirbolt_model::r4b::types::ExtensionValue::Address(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Annotation(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAnnotation", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Attachment(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::CodeableReference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableReference", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Coding(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::ContactPoint(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueContactPoint", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Count(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Distance(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDistance", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Duration(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDuration", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::HumanName(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueHumanName", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Identifier(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueIdentifier", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Money(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::RatioRange(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRatioRange", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::SampledData(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueSampledData", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Signature(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueSignature", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::ContactDetail(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueContactDetail", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Contributor(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueContributor", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::DataRequirement(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueDataRequirement", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Expression(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueExpression", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::ParameterDefinition(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueParameterDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::RelatedArtifact(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueRelatedArtifact", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::TriggerDefinition(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueTriggerDefinition", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::UsageContext(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueUsageContext", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Dosage(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
                }
                fhirbolt_model::r4b::types::ExtensionValue::Invalid => {
                    return Err(serde::ser::Error::custom("value is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4b::types::Extension>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4b::types::Extension>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<Box<fhirbolt_model::r4b::types::Extension>>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::Extension>
{
    type Value = fhirbolt_model::r4b::types::Extension;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::Extension,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::Extension;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Extension")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::Extension, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "valueCodeableReference")]
                    ValueCodeableReference,
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
                    #[serde(rename = "valueRatioRange")]
                    ValueRatioRange,
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                            "valueCodeableReference",
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
                            "valueRatioRange",
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
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#url: Option<std::string::String> = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::ExtensionValue> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Url => {
                            if r#url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            r#url = Some(map_access.next_value()?);
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Base64Binary(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Base64Binary(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Boolean(
                                    variant,
                                ) = r#enum
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueCanonical => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Canonical (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Canonical > > ()) ?)) ;
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Canonical(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Canonical(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCanonical",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCanonical");
                            }
                        }
                        Field::ValueCode => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Code(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Code(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Code>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Code(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Code(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCode");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Date(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Date(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Date>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Date(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Date(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Decimal(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Decimal(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueId => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Id(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Id>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Id(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Id(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueInstant => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Instant(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Instant (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Instant > > ()) ?)) ;
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Instant(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Instant(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInstant");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Integer(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Integer(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Integer(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Markdown(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueMarkdown",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueOid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Oid(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Oid(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Oid>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Oid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Oid(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueOid"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueOid");
                            }
                        }
                        Field::ValuePositiveInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: PositiveInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: PositiveInt > > ()) ?)) ;
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::PositiveInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::PositiveInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valuePositiveInt",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valuePositiveInt");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::String(variant) =
                                    r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::String(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::String>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::String(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueTime => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Time(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Time(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Time>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Time(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Time(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
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
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::UnsignedInt(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::UnsignedInt(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUnsignedInt",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUnsignedInt");
                            }
                        }
                        Field::ValueUri => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Uri(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Uri(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Uri>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Uri(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Uri(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUri"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueUrl => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Url(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Url(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Url>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Url(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Url(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUrl");
                            }
                        }
                        Field::ValueUuid => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Uuid(variant) =
                                    r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Uuid(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Uuid>>(),
                                    )?,
                                ));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(
                                    fhirbolt_model::r4b::types::ExtensionValue::Uuid(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::ExtensionValue::Uuid(variant) =
                                    r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUuid");
                            }
                        }
                        Field::ValueAddress => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Address(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Address>>(),
                                )?,
                            ));
                        }
                        Field::ValueAge => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Age(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                )?,
                            ));
                        }
                        Field::ValueAnnotation => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Annotation(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Annotation>>(),
                                )?,
                            ));
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Attachment(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Attachment>>(),
                                )?,
                            ));
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueCodeableReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableReference",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: CodeableReference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableReference > > ()) ?)) ;
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Coding(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Coding>>(),
                                )?,
                            ));
                        }
                        Field::ValueContactPoint => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: ContactPoint (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactPoint > > ()) ?)) ;
                        }
                        Field::ValueCount => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Count(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Count>>(),
                                )?,
                            ));
                        }
                        Field::ValueDistance => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Distance(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Distance>>(),
                                )?,
                            ));
                        }
                        Field::ValueDuration => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Duration(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(),
                                )?,
                            ));
                        }
                        Field::ValueHumanName => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::HumanName(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::HumanName>>(),
                                )?,
                            ));
                        }
                        Field::ValueIdentifier => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Identifier(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Identifier>>(),
                                )?,
                            ));
                        }
                        Field::ValueMoney => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Money(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                                )?,
                            ));
                        }
                        Field::ValuePeriod => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Period(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                )?,
                            ));
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Quantity(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
                                )?,
                            ));
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Range(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                )?,
                            ));
                        }
                        Field::ValueRatio => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Ratio(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                )?,
                            ));
                        }
                        Field::ValueRatioRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatioRange"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::RatioRange(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::RatioRange>>(),
                                )?,
                            ));
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Reference(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            ));
                        }
                        Field::ValueSampledData => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: SampledData (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: SampledData > > ()) ?)) ;
                        }
                        Field::ValueSignature => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Signature(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Signature>>(),
                                )?,
                            ));
                        }
                        Field::ValueTiming => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Timing(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                )?,
                            ));
                        }
                        Field::ValueContactDetail => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: ContactDetail (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?)) ;
                        }
                        Field::ValueContributor => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: Contributor (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Contributor > > ()) ?)) ;
                        }
                        Field::ValueDataRequirement => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: DataRequirement (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DataRequirement > > ()) ?)) ;
                        }
                        Field::ValueExpression => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Expression(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Expression>>(),
                                )?,
                            ));
                        }
                        Field::ValueParameterDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value =
                                Some(
                                    fhirbolt_model::r4b::types::ExtensionValue::ParameterDefinition(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Box<
                                                fhirbolt_model::r4b::types::ParameterDefinition,
                                            >>(),
                                        )?,
                                    ),
                                );
                        }
                        Field::ValueRelatedArtifact => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: RelatedArtifact (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?)) ;
                        }
                        Field::ValueTriggerDefinition => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: TriggerDefinition (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: TriggerDefinition > > ()) ?)) ;
                        }
                        Field::ValueUsageContext => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: types :: ExtensionValue :: UsageContext (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?)) ;
                        }
                        Field::ValueDosage => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value = Some(fhirbolt_model::r4b::types::ExtensionValue::Dosage(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                )?,
                            ));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::types::Extension {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#url: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#url.unwrap_or(Default::default())
                    } else {
                        r#url.ok_or(serde::de::Error::missing_field("url"))?
                    },
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4b::types::Extension>>
{
    type Value = Box<fhirbolt_model::r4b::types::Extension>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::Extension>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4b::types::Extension>>
{
    type Value = Vec<fhirbolt_model::r4b::types::Extension>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::Extension>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::Extension>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::types::Extension>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::types::Extension>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::types::Extension>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::types::Extension>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<Box<fhirbolt_model::r4b::types::Extension>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
