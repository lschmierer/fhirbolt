// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "If the parameter is a data type."]
#[derive(Debug, Clone)]
pub enum ParametersParameterValue {
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
impl Default for ParametersParameterValue {
    fn default() -> ParametersParameterValue {
        ParametersParameterValue::Invalid
    }
}
#[doc = "A parameter passed to or received from the operation."]
#[derive(Default, Debug, Clone)]
pub struct ParametersParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the parameter (reference to the operation definition)."]
    pub r#name: super::super::types::String,
    #[doc = "If the parameter is a data type."]
    pub r#value: Option<ParametersParameterValue>,
    #[doc = "If the parameter is a whole resource."]
    pub r#resource: Option<Box<super::super::Resource>>,
    #[doc = "A named part of a multi-part parameter."]
    pub r#part: Vec<ParametersParameter>,
}
impl serde::ser::Serialize for ParametersParameter {
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
            if _ctx.output_json {
                if let Some(some) = self.r#name.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#name.id.as_ref(),
                        extension: &self.r#name.extension,
                    };
                    state.serialize_entry("_name", &primitive_element)?;
                }
            } else {
                state.serialize_entry("name", &self.r#name)?;
            }
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    ParametersParameterValue::Base64Binary(ref value) => {
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
                    ParametersParameterValue::Boolean(ref value) => {
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
                    ParametersParameterValue::Canonical(ref value) => {
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
                    ParametersParameterValue::Code(ref value) => {
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
                    ParametersParameterValue::Date(ref value) => {
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
                    ParametersParameterValue::DateTime(ref value) => {
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
                    ParametersParameterValue::Decimal(ref value) => {
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
                    ParametersParameterValue::Id(ref value) => {
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
                    ParametersParameterValue::Instant(ref value) => {
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
                    ParametersParameterValue::Integer(ref value) => {
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
                    ParametersParameterValue::Markdown(ref value) => {
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
                    ParametersParameterValue::Oid(ref value) => {
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
                    ParametersParameterValue::PositiveInt(ref value) => {
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
                    ParametersParameterValue::String(ref value) => {
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
                    ParametersParameterValue::Time(ref value) => {
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
                    ParametersParameterValue::UnsignedInt(ref value) => {
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
                    ParametersParameterValue::Uri(ref value) => {
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
                    ParametersParameterValue::Url(ref value) => {
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
                    ParametersParameterValue::Uuid(ref value) => {
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
                    ParametersParameterValue::Address(ref value) => {
                        state.serialize_entry("valueAddress", value)?;
                    }
                    ParametersParameterValue::Age(ref value) => {
                        state.serialize_entry("valueAge", value)?;
                    }
                    ParametersParameterValue::Annotation(ref value) => {
                        state.serialize_entry("valueAnnotation", value)?;
                    }
                    ParametersParameterValue::Attachment(ref value) => {
                        state.serialize_entry("valueAttachment", value)?;
                    }
                    ParametersParameterValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    ParametersParameterValue::Coding(ref value) => {
                        state.serialize_entry("valueCoding", value)?;
                    }
                    ParametersParameterValue::ContactPoint(ref value) => {
                        state.serialize_entry("valueContactPoint", value)?;
                    }
                    ParametersParameterValue::Count(ref value) => {
                        state.serialize_entry("valueCount", value)?;
                    }
                    ParametersParameterValue::Distance(ref value) => {
                        state.serialize_entry("valueDistance", value)?;
                    }
                    ParametersParameterValue::Duration(ref value) => {
                        state.serialize_entry("valueDuration", value)?;
                    }
                    ParametersParameterValue::HumanName(ref value) => {
                        state.serialize_entry("valueHumanName", value)?;
                    }
                    ParametersParameterValue::Identifier(ref value) => {
                        state.serialize_entry("valueIdentifier", value)?;
                    }
                    ParametersParameterValue::Money(ref value) => {
                        state.serialize_entry("valueMoney", value)?;
                    }
                    ParametersParameterValue::Period(ref value) => {
                        state.serialize_entry("valuePeriod", value)?;
                    }
                    ParametersParameterValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    ParametersParameterValue::Range(ref value) => {
                        state.serialize_entry("valueRange", value)?;
                    }
                    ParametersParameterValue::Ratio(ref value) => {
                        state.serialize_entry("valueRatio", value)?;
                    }
                    ParametersParameterValue::Reference(ref value) => {
                        state.serialize_entry("valueReference", value)?;
                    }
                    ParametersParameterValue::SampledData(ref value) => {
                        state.serialize_entry("valueSampledData", value)?;
                    }
                    ParametersParameterValue::Signature(ref value) => {
                        state.serialize_entry("valueSignature", value)?;
                    }
                    ParametersParameterValue::Timing(ref value) => {
                        state.serialize_entry("valueTiming", value)?;
                    }
                    ParametersParameterValue::ContactDetail(ref value) => {
                        state.serialize_entry("valueContactDetail", value)?;
                    }
                    ParametersParameterValue::Contributor(ref value) => {
                        state.serialize_entry("valueContributor", value)?;
                    }
                    ParametersParameterValue::DataRequirement(ref value) => {
                        state.serialize_entry("valueDataRequirement", value)?;
                    }
                    ParametersParameterValue::Expression(ref value) => {
                        state.serialize_entry("valueExpression", value)?;
                    }
                    ParametersParameterValue::ParameterDefinition(ref value) => {
                        state.serialize_entry("valueParameterDefinition", value)?;
                    }
                    ParametersParameterValue::RelatedArtifact(ref value) => {
                        state.serialize_entry("valueRelatedArtifact", value)?;
                    }
                    ParametersParameterValue::TriggerDefinition(ref value) => {
                        state.serialize_entry("valueTriggerDefinition", value)?;
                    }
                    ParametersParameterValue::UsageContext(ref value) => {
                        state.serialize_entry("valueUsageContext", value)?;
                    }
                    ParametersParameterValue::Dosage(ref value) => {
                        state.serialize_entry("valueDosage", value)?;
                    }
                    ParametersParameterValue::Meta(ref value) => {
                        state.serialize_entry("valueMeta", value)?;
                    }
                    ParametersParameterValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#resource.as_ref() {
                state.serialize_entry("resource", some)?;
            }
            if !self.r#part.is_empty() {
                state.serialize_entry("part", &self.r#part)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ParametersParameter {
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
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
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
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "part")]
            Part,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ParametersParameter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ParametersParameter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ParametersParameter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#value: Option<ParametersParameterValue> = None;
                let mut r#resource: Option<Box<super::super::Resource>> = None;
                let mut r#part: Option<Vec<ParametersParameter>> = None;
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
                            Field::Name => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#name.is_some() {
                                        return Err(serde::de::Error::duplicate_field("name"));
                                    }
                                    r#name = Some(map_access.next_value()?);
                                }
                            }
                            Field::NamePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#name.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_name"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "name",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBase64Binary => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Base64Binary(Default::default()),
                                    );
                                    if let ParametersParameterValue::Base64Binary(variant) = r#enum
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
                                    r#value = Some(ParametersParameterValue::Base64Binary(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBase64BinaryPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Base64Binary(Default::default()),
                                    );
                                    if let ParametersParameterValue::Base64Binary(variant) = r#enum
                                    {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBase64Binary",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Boolean(Default::default()),
                                    );
                                    if let ParametersParameterValue::Boolean(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Boolean(Default::default()),
                                    );
                                    if let ParametersParameterValue::Boolean(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueCanonical => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Canonical(Default::default()),
                                    );
                                    if let ParametersParameterValue::Canonical(variant) = r#enum {
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
                                    r#value = Some(ParametersParameterValue::Canonical(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueCanonicalPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Canonical(Default::default()),
                                    );
                                    if let ParametersParameterValue::Canonical(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueCanonical",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueCode => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Code(Default::default()),
                                    );
                                    if let ParametersParameterValue::Code(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueCode",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Code(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueCodePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Code(Default::default()),
                                    );
                                    if let ParametersParameterValue::Code(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueCode",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDate => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Date(Default::default()),
                                    );
                                    if let ParametersParameterValue::Date(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDate",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Date(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Date(Default::default()),
                                    );
                                    if let ParametersParameterValue::Date(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::DateTime(Default::default()),
                                    );
                                    if let ParametersParameterValue::DateTime(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::DateTime(Default::default()),
                                    );
                                    if let ParametersParameterValue::DateTime(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDateTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDecimal => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Decimal(Default::default()),
                                    );
                                    if let ParametersParameterValue::Decimal(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::Decimal(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDecimalPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Decimal(Default::default()),
                                    );
                                    if let ParametersParameterValue::Decimal(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDecimal",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueId => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Id(Default::default()),
                                    );
                                    if let ParametersParameterValue::Id(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueId",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Id(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueIdPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Id(Default::default()),
                                    );
                                    if let ParametersParameterValue::Id(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueId",
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueId",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInstant => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Instant(Default::default()),
                                    );
                                    if let ParametersParameterValue::Instant(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::Instant(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueInstantPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Instant(Default::default()),
                                    );
                                    if let ParametersParameterValue::Instant(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueInstant",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Integer(Default::default()),
                                    );
                                    if let ParametersParameterValue::Integer(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::Integer(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Integer(Default::default()),
                                    );
                                    if let ParametersParameterValue::Integer(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueInteger",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueMarkdown => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Markdown(Default::default()),
                                    );
                                    if let ParametersParameterValue::Markdown(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::Markdown(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueMarkdownPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Markdown(Default::default()),
                                    );
                                    if let ParametersParameterValue::Markdown(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueMarkdown",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueOid => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Oid(Default::default()),
                                    );
                                    if let ParametersParameterValue::Oid(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueOid",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Oid(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueOidPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Oid(Default::default()),
                                    );
                                    if let ParametersParameterValue::Oid(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueOid",
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueOid",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValuePositiveInt => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::PositiveInt(Default::default()),
                                    );
                                    if let ParametersParameterValue::PositiveInt(variant) = r#enum {
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
                                    r#value = Some(ParametersParameterValue::PositiveInt(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValuePositiveIntPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::PositiveInt(Default::default()),
                                    );
                                    if let ParametersParameterValue::PositiveInt(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valuePositiveInt",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::String(Default::default()),
                                    );
                                    if let ParametersParameterValue::String(variant) = r#enum {
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
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    r#value = Some(ParametersParameterValue::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::String(Default::default()),
                                    );
                                    if let ParametersParameterValue::String(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Time(Default::default()),
                                    );
                                    if let ParametersParameterValue::Time(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueTime",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Time(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Time(Default::default()),
                                    );
                                    if let ParametersParameterValue::Time(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueTime",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueUnsignedInt => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::UnsignedInt(Default::default()),
                                    );
                                    if let ParametersParameterValue::UnsignedInt(variant) = r#enum {
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
                                    r#value = Some(ParametersParameterValue::UnsignedInt(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueUnsignedIntPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::UnsignedInt(Default::default()),
                                    );
                                    if let ParametersParameterValue::UnsignedInt(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueUnsignedInt",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueUri => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Uri(Default::default()),
                                    );
                                    if let ParametersParameterValue::Uri(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueUri",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Uri(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueUriPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Uri(Default::default()),
                                    );
                                    if let ParametersParameterValue::Uri(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueUri",
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueUri",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueUrl => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Url(Default::default()),
                                    );
                                    if let ParametersParameterValue::Url(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueUrl",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Url(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueUrlPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Url(Default::default()),
                                    );
                                    if let ParametersParameterValue::Url(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueUrl",
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueUrl",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueUuid => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Uuid(Default::default()),
                                    );
                                    if let ParametersParameterValue::Uuid(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueUuid",
                                            ));
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
                                    r#value = Some(ParametersParameterValue::Uuid(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueUuidPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ParametersParameterValue::Uuid(Default::default()),
                                    );
                                    if let ParametersParameterValue::Uuid(variant) = r#enum {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueUuid",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "name",
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
                                            "resource",
                                            "part",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueAddress => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAddress"));
                                }
                                r#value = Some(ParametersParameterValue::Address(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueAge => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAge"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Age(map_access.next_value()?));
                            }
                            Field::ValueAnnotation => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAnnotation",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Annotation(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(ParametersParameterValue::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueContactPoint => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactPoint",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::ContactPoint(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCount => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCount"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Count(map_access.next_value()?));
                            }
                            Field::ValueDistance => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDistance"));
                                }
                                r#value = Some(ParametersParameterValue::Distance(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueDuration => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDuration"));
                                }
                                r#value = Some(ParametersParameterValue::Duration(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueHumanName => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueHumanName",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::HumanName(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueIdentifier => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueIdentifier",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Identifier(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueMoney => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMoney"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Money(map_access.next_value()?));
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(ParametersParameterValue::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ParametersParameterValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Range(map_access.next_value()?));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Ratio(map_access.next_value()?));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::SampledData(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueSignature => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSignature",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Signature(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueTiming => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTiming"));
                                }
                                r#value = Some(ParametersParameterValue::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueContactDetail => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactDetail",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::ContactDetail(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueContributor => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContributor",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Contributor(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueDataRequirement => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueDataRequirement",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::DataRequirement(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueExpression => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueExpression",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::Expression(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueParameterDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueParameterDefinition",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::ParameterDefinition(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRelatedArtifact => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueRelatedArtifact",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::RelatedArtifact(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueTriggerDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueTriggerDefinition",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::TriggerDefinition(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueUsageContext => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUsageContext",
                                    ));
                                }
                                r#value = Some(ParametersParameterValue::UsageContext(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueDosage => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDosage"));
                                }
                                r#value = Some(ParametersParameterValue::Dosage(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueMeta => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMeta"));
                                }
                                r#value =
                                    Some(ParametersParameterValue::Meta(map_access.next_value()?));
                            }
                            Field::Resource => {
                                if r#resource.is_some() {
                                    return Err(serde::de::Error::duplicate_field("resource"));
                                }
                                r#resource = Some(map_access.next_value()?);
                            }
                            Field::Part => {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                r#part = Some(map_access.next_value()?);
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
                                        "name",
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
                                        "resource",
                                        "part",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ParametersParameter {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#name: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#name.unwrap_or(Default::default())
                        } else {
                            r#name.ok_or(serde::de::Error::missing_field("name"))?
                        },
                        r#value,
                        r#resource,
                        r#part: r#part.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This resource is a non-persisted resource used to pass information into and back from an [operation](https://hl7.org/FHIR/operations.html)). It has no other use, and there is no RESTful endpoint associated with it."]
#[derive(Default, Debug, Clone)]
pub struct Parameters {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A parameter passed to or received from the operation."]
    pub r#parameter: Vec<ParametersParameter>,
}
impl crate::AnyResource for Parameters {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for Parameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Parameters")?;
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
            if !self.r#parameter.is_empty() {
                state.serialize_entry("parameter", &self.r#parameter)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Parameters {
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
            #[serde(rename = "parameter")]
            Parameter,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Parameters;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Parameters")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Parameters, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#parameter: Option<Vec<ParametersParameter>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Parameters" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Parameters",
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
                                        &["id", "meta", "implicitRules", "language", "parameter"],
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
                                        &["id", "meta", "implicitRules", "language", "parameter"],
                                    ));
                                }
                            }
                            Field::Parameter => {
                                if r#parameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("parameter"));
                                }
                                r#parameter = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "meta", "implicitRules", "language", "parameter"],
                                ));
                            },
                        }
                    }
                    Ok(Parameters {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#parameter: r#parameter.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
