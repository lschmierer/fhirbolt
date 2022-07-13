// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
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
}
impl Default for ParametersParameterValue {
    fn default() -> ParametersParameterValue {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ParametersParameter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#value: Option<ParametersParameterValue>,
    pub r#resource: Option<Box<super::Resource>>,
    pub r#part: Vec<ParametersParameter>,
}
impl serde::ser::Serialize for ParametersParameter {
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
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            match some {
                ParametersParameterValue::Base64Binary(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBase64Binary", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueBoolean", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueCanonical", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Code(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueCode", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueCode", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDate", &primitive_element)?;
                    }
                }
                ParametersParameterValue::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDateTime", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Decimal(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueDecimal", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueDecimal", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Id(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueId", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueId", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Instant(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInstant", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInstant", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueInteger", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Markdown(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueMarkdown", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueMarkdown", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Oid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueOid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueOid", &primitive_element)?;
                    }
                }
                ParametersParameterValue::PositiveInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valuePositiveInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valuePositiveInt", &primitive_element)?;
                    }
                }
                ParametersParameterValue::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueString", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Time(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueTime", &primitive_element)?;
                    }
                }
                ParametersParameterValue::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUnsignedInt", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUnsignedInt", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUri", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Url(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUrl", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUrl", &primitive_element)?;
                    }
                }
                ParametersParameterValue::Uuid(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("valueUuid", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_valueUuid", &primitive_element)?;
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
            }
        }
        if let Some(some) = self.r#resource.as_ref() {
            state.serialize_entry("resource", some)?;
        }
        if !self.r#part.is_empty() {
            state.serialize_entry("part", &self.r#part)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ParametersParameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                let mut r#resource: Option<Box<super::Resource>> = None;
                let mut r#part: Option<Vec<ParametersParameter>> = None;
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
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "valueBase64Binary" => {
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::Base64Binary(Default::default()),
                            );
                            if let ParametersParameterValue::Base64Binary(variant) = r#enum {
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
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::Base64Binary(Default::default()),
                            );
                            if let ParametersParameterValue::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueBase64Binary",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueBoolean" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Boolean(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Boolean(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueCanonical" => {
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::Canonical(Default::default()),
                            );
                            if let ParametersParameterValue::Canonical(variant) = r#enum {
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
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::Canonical(Default::default()),
                            );
                            if let ParametersParameterValue::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueCanonical",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueCode" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Code(Default::default()));
                            if let ParametersParameterValue::Code(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueCode" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Code(Default::default()));
                            if let ParametersParameterValue::Code(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueCode"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueDate" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Date(Default::default()));
                            if let ParametersParameterValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Date(Default::default()));
                            if let ParametersParameterValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDate"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueDateTime" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::DateTime(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::DateTime(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueDecimal" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Decimal(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDecimal" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Decimal(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDecimal"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueId" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Id(Default::default()));
                            if let ParametersParameterValue::Id(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueId" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Id(Default::default()));
                            if let ParametersParameterValue::Id(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueId"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueInstant" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Instant(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Instant(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInstant" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Instant(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Instant(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInstant"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueInteger" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Integer(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInteger" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Integer(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueMarkdown" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Markdown(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Markdown(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueMarkdown" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::Markdown(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::Markdown(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueMarkdown",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueOid" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Oid(Default::default()));
                            if let ParametersParameterValue::Oid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueOid" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Oid(Default::default()));
                            if let ParametersParameterValue::Oid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueOid"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valuePositiveInt" => {
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::PositiveInt(Default::default()),
                            );
                            if let ParametersParameterValue::PositiveInt(variant) = r#enum {
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
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::PositiveInt(Default::default()),
                            );
                            if let ParametersParameterValue::PositiveInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valuePositiveInt",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueString" => {
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::String(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(ParametersParameterValue::String(
                                Default::default(),
                            ));
                            if let ParametersParameterValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueTime" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Time(Default::default()));
                            if let ParametersParameterValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Time(Default::default()));
                            if let ParametersParameterValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueUnsignedInt" => {
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::UnsignedInt(Default::default()),
                            );
                            if let ParametersParameterValue::UnsignedInt(variant) = r#enum {
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
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value.get_or_insert(
                                ParametersParameterValue::UnsignedInt(Default::default()),
                            );
                            if let ParametersParameterValue::UnsignedInt(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueUnsignedInt",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueUri" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Uri(Default::default()));
                            if let ParametersParameterValue::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUri" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Uri(Default::default()));
                            if let ParametersParameterValue::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUri"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueUrl" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Url(Default::default()));
                            if let ParametersParameterValue::Url(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUrl" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Url(Default::default()));
                            if let ParametersParameterValue::Url(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        "valueUuid" => {
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Uuid(Default::default()));
                            if let ParametersParameterValue::Uuid(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUuid" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#value
                                .get_or_insert(ParametersParameterValue::Uuid(Default::default()));
                            if let ParametersParameterValue::Uuid(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUuid"));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
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
                            r#value =
                                Some(ParametersParameterValue::Address(map_access.next_value()?));
                        }
                        "valueAge" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            r#value = Some(ParametersParameterValue::Age(map_access.next_value()?));
                        }
                        "valueAnnotation" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            r#value = Some(ParametersParameterValue::Annotation(
                                map_access.next_value()?,
                            ));
                        }
                        "valueAttachment" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ParametersParameterValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        "valueCodeableConcept" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "valueCoding" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Coding(map_access.next_value()?));
                        }
                        "valueContactPoint" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            r#value = Some(ParametersParameterValue::ContactPoint(
                                map_access.next_value()?,
                            ));
                        }
                        "valueCount" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Count(map_access.next_value()?));
                        }
                        "valueDistance" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Distance(map_access.next_value()?));
                        }
                        "valueDuration" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Duration(map_access.next_value()?));
                        }
                        "valueHumanName" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            r#value = Some(ParametersParameterValue::HumanName(
                                map_access.next_value()?,
                            ));
                        }
                        "valueIdentifier" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            r#value = Some(ParametersParameterValue::Identifier(
                                map_access.next_value()?,
                            ));
                        }
                        "valueMoney" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Money(map_access.next_value()?));
                        }
                        "valuePeriod" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Period(map_access.next_value()?));
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Quantity(map_access.next_value()?));
                        }
                        "valueRange" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Range(map_access.next_value()?));
                        }
                        "valueRatio" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Ratio(map_access.next_value()?));
                        }
                        "valueReference" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ParametersParameterValue::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "valueSampledData" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            r#value = Some(ParametersParameterValue::SampledData(
                                map_access.next_value()?,
                            ));
                        }
                        "valueSignature" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            r#value = Some(ParametersParameterValue::Signature(
                                map_access.next_value()?,
                            ));
                        }
                        "valueTiming" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Timing(map_access.next_value()?));
                        }
                        "valueContactDetail" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::ContactDetail(
                                map_access.next_value()?,
                            ));
                        }
                        "valueContributor" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            r#value = Some(ParametersParameterValue::Contributor(
                                map_access.next_value()?,
                            ));
                        }
                        "valueDataRequirement" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::DataRequirement(
                                map_access.next_value()?,
                            ));
                        }
                        "valueExpression" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            r#value = Some(ParametersParameterValue::Expression(
                                map_access.next_value()?,
                            ));
                        }
                        "valueParameterDefinition" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::ParameterDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        "valueRelatedArtifact" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::RelatedArtifact(
                                map_access.next_value()?,
                            ));
                        }
                        "valueTriggerDefinition" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            r#value = Some(ParametersParameterValue::TriggerDefinition(
                                map_access.next_value()?,
                            ));
                        }
                        "valueUsageContext" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            r#value = Some(ParametersParameterValue::UsageContext(
                                map_access.next_value()?,
                            ));
                        }
                        "valueDosage" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Dosage(map_access.next_value()?));
                        }
                        "valueMeta" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMeta"));
                            }
                            r#value =
                                Some(ParametersParameterValue::Meta(map_access.next_value()?));
                        }
                        "resource" => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(map_access.next_value()?);
                        }
                        "part" => {
                            if r#part.is_some() {
                                return Err(serde::de::Error::duplicate_field("part"));
                            }
                            r#part = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "name",
                                    "value",
                                    "resource",
                                    "part",
                                ],
                            ))
                        }
                    }
                }
                Ok(ParametersParameter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#value,
                    r#resource,
                    r#part: r#part.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Parameters {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#parameter: Vec<ParametersParameter>,
}
impl serde::ser::Serialize for Parameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Parameters")?;
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if !self.r#parameter.is_empty() {
            state.serialize_entry("parameter", &self.r#parameter)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Parameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "parameter" => {
                            if r#parameter.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            r#parameter = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "meta", "implicit_rules", "language", "parameter"],
                            ))
                        }
                    }
                }
                Ok(Parameters {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#parameter: r#parameter.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
