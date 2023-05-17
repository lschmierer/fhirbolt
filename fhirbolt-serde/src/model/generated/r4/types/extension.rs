// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::Extension;
impl serde::ser::Serialize for SerializationContext<&Extension> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Extension", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        state.serialize_entry("url", &self.value.r#url)?;
        {
            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
            if let Some(some) = self.value.r#value.as_ref() {
                match some {
                    _Enum::Base64Binary(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueBase64Binary", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Boolean(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueBoolean", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::Canonical(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueCanonical", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Code(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueCode", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueCode", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueCode", ctx)
                            })?;
                        }
                    }
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Decimal(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("valueDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Id(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueId", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Instant(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::Markdown(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueMarkdown", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Oid(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueOid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::PositiveInt(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valuePositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::String(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueString", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueTime", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Uri(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueUri", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Url(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueUrl", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
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
                    _Enum::Uuid(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("valueUuid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_valueUuid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueUuid", ctx)
                            })?;
                        }
                    }
                    _Enum::Address(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
                    }
                    _Enum::Age(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
                    }
                    _Enum::Annotation(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueAnnotation", ctx)
                        })?;
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueAttachment", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Coding(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
                    }
                    _Enum::ContactPoint(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueContactPoint", ctx)
                        })?;
                    }
                    _Enum::Count(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
                    }
                    _Enum::Distance(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueDistance", ctx)
                        })?;
                    }
                    _Enum::Duration(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueDuration", ctx)
                        })?;
                    }
                    _Enum::HumanName(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueHumanName", ctx)
                        })?;
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueIdentifier", ctx)
                        })?;
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
                    }
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueReference", ctx)
                        })?;
                    }
                    _Enum::SampledData(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueSampledData", ctx)
                        })?;
                    }
                    _Enum::Signature(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueSignature", ctx)
                        })?;
                    }
                    _Enum::Timing(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
                    }
                    _Enum::ContactDetail(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueContactDetail", ctx)
                        })?;
                    }
                    _Enum::Contributor(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueContributor", ctx)
                        })?;
                    }
                    _Enum::DataRequirement(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueDataRequirement", ctx)
                        })?;
                    }
                    _Enum::Expression(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueExpression", ctx)
                        })?;
                    }
                    _Enum::ParameterDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueParameterDefinition", ctx)
                        })?;
                    }
                    _Enum::RelatedArtifact(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueRelatedArtifact", ctx)
                        })?;
                    }
                    _Enum::TriggerDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueTriggerDefinition", ctx)
                        })?;
                    }
                    _Enum::UsageContext(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueUsageContext", ctx)
                        })?;
                    }
                    _Enum::Dosage(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
                    }
                    _Enum::Meta(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("valueMeta", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("value is invalid")),
                }
            }
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Extension>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Extension>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Extension> {
    type Value = Extension;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Extension>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Extension;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Extension")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Extension, V::Error>
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
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#url: Option<std::string::String> = None;
                let mut r#value: Option<fhirbolt_model::r4::types::ExtensionValue> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Url => {
                            if r#url.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            r#url = Some(map_access.next_value()?);
                        }
                        Field::ValueBase64Binary => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Base64Binary,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Base64Binary(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Canonical,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Code(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Date,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DateTime,
                                > = self.0.transmute();
                                r#value = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
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
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Id,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Id(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Instant,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Markdown,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Markdown(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueMarkdown",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Oid,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Oid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueOid"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valuePositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::PositiveInt,
                                > = self.0.transmute();
                                r#value = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valuePositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#value = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Time,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::UnsignedInt,
                                > = self.0.transmute();
                                r#value = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUri"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Url,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Url(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uuid,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Uuid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum = r#value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
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
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Address>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAge => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Age>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Age(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAnnotation => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Annotation>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Annotation(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueAttachment => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Attachment>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCodeableConcept => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCoding => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Coding>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueContactPoint => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::ContactPoint>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ContactPoint(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCount => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Count>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Count(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueDistance => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Distance>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Distance(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueDuration => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Duration>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueHumanName => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::HumanName>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::HumanName(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueIdentifier => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Identifier>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueMoney => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Money>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValuePeriod => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Period>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueQuantity => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueRange => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueRatio => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Ratio>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueReference => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueSampledData => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::SampledData>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::SampledData(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueSignature => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Signature>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Signature(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueTiming => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Timing>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueContactDetail => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::ContactDetail>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ContactDetail(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueContributor => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Contributor>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Contributor(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueDataRequirement => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::DataRequirement>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::DataRequirement(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueExpression => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Expression>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Expression(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueParameterDefinition => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::ParameterDefinition>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ParameterDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueRelatedArtifact => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::RelatedArtifact>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::RelatedArtifact(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueTriggerDefinition => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::TriggerDefinition>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::TriggerDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueUsageContext => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::UsageContext>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::UsageContext(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueDosage => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Dosage>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Dosage(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueMeta => {
                            use fhirbolt_model::r4::types::ExtensionValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Meta>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Meta(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Extension {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Extension>> {
    type Value = Box<Extension>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Extension>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Extension>> {
    type Value = Vec<Extension>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Extension>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Extension>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Extension> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
