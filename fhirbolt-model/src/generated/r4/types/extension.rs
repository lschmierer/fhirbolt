// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "Value of extension - must be one of a constrained set of the data types (see [Extensibility](https://hl7.org/FHIR/extensibility.html)) for a list)."]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Default, Debug, Clone, PartialEq)]
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
impl serde::ser::Serialize for Extension {
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
            state.serialize_entry("url", &self.r#url)?;
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    ExtensionValue::Base64Binary(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBase64Binary", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBase64Binary", value)?;
                        }
                    }
                    ExtensionValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBoolean", value)?;
                        }
                    }
                    ExtensionValue::Canonical(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueCanonical", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueCanonical", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueCanonical", value)?;
                        }
                    }
                    ExtensionValue::Code(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueCode", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueCode", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueCode", value)?;
                        }
                    }
                    ExtensionValue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDate", value)?;
                        }
                    }
                    ExtensionValue::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDateTime", value)?;
                        }
                    }
                    ExtensionValue::Decimal(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = some.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })?;
                                state.serialize_entry("valueDecimal", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDecimal", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDecimal", value)?;
                        }
                    }
                    ExtensionValue::Id(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueId", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueId", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueId", value)?;
                        }
                    }
                    ExtensionValue::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueInstant", value)?;
                        }
                    }
                    ExtensionValue::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueInteger", value)?;
                        }
                    }
                    ExtensionValue::Markdown(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueMarkdown", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueMarkdown", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueMarkdown", value)?;
                        }
                    }
                    ExtensionValue::Oid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueOid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueOid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueOid", value)?;
                        }
                    }
                    ExtensionValue::PositiveInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valuePositiveInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valuePositiveInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valuePositiveInt", value)?;
                        }
                    }
                    ExtensionValue::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueString", value)?;
                        }
                    }
                    ExtensionValue::Time(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueTime", value)?;
                        }
                    }
                    ExtensionValue::UnsignedInt(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueUnsignedInt", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueUnsignedInt", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueUnsignedInt", value)?;
                        }
                    }
                    ExtensionValue::Uri(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueUri", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueUri", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueUri", value)?;
                        }
                    }
                    ExtensionValue::Url(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueUrl", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueUrl", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueUrl", value)?;
                        }
                    }
                    ExtensionValue::Uuid(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueUuid", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueUuid", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueUuid", value)?;
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
        })
    }
}
