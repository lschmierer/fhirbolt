// Generated on 2022-10-11 by fhirbolt-codegen v0.1.0
#[doc = "The value of the input parameter as a basic type."]
#[derive(Debug, Clone)]
pub enum TaskInputValue {
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
impl crate::model::ResourceOrElement for TaskInputValue {}
impl Default for TaskInputValue {
    fn default() -> TaskInputValue {
        TaskInputValue::Invalid
    }
}
#[doc = "The value of the Output parameter as a basic type."]
#[derive(Debug, Clone)]
pub enum TaskOutputValue {
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
impl crate::model::ResourceOrElement for TaskOutputValue {}
impl Default for TaskOutputValue {
    fn default() -> TaskOutputValue {
        TaskOutputValue::Invalid
    }
}
#[doc = "If the Task.focus is a request resource and the task is seeking fulfillment (i.e. is asking for the request to be actioned), this element identifies any limitations on what parts of the referenced request should be actioned."]
#[derive(Default, Debug, Clone)]
pub struct TaskRestriction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the number of times the requested action should occur."]
    pub r#repetitions: Option<super::super::types::PositiveInt>,
    #[doc = "Over what time-period is fulfillment sought."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "For requests that are targeted to more than on potential recipient/target, for whom is fulfillment sought?"]
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
}
impl crate::model::ResourceOrElement for TaskRestriction {}
impl serde::ser::Serialize for TaskRestriction {
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
        if let Some(some) = self.r#repetitions.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("repetitions", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_repetitions", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if !self.r#recipient.is_empty() {
            state.serialize_entry("recipient", &self.r#recipient)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for TaskRestriction {
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
            #[serde(rename = "repetitions")]
            Repetitions,
            #[serde(rename = "_repetitions")]
            RepetitionsPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "recipient")]
            Recipient,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = TaskRestriction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskRestriction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TaskRestriction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#repetitions: Option<super::super::types::PositiveInt> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#recipient: Option<Vec<Box<super::super::types::Reference>>> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::Repetitions => {
                                let some = r#repetitions.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repetitions"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RepetitionsPrimitiveElement => {
                                let some = r#repetitions.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_repetitions"));
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
                            Field::Recipient => {
                                if r#recipient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recipient"));
                                }
                                r#recipient = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "repetitions",
                                            "period",
                                            "recipient",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(TaskRestriction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#repetitions,
                        r#period,
                        r#recipient: r#recipient.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Additional information that may be needed in the execution of the task."]
#[derive(Default, Debug, Clone)]
pub struct TaskInput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code or description indicating how the input is intended to be used as part of the task execution."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the input parameter as a basic type."]
    pub r#value: TaskInputValue,
}
impl crate::model::ResourceOrElement for TaskInput {}
impl serde::ser::Serialize for TaskInput {
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
        match self.r#value {
            TaskInputValue::Base64Binary(ref value) => {
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
            TaskInputValue::Boolean(ref value) => {
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
            TaskInputValue::Canonical(ref value) => {
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
            TaskInputValue::Code(ref value) => {
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
            TaskInputValue::Date(ref value) => {
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
            TaskInputValue::DateTime(ref value) => {
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
            TaskInputValue::Decimal(ref value) => {
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
            TaskInputValue::Id(ref value) => {
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
            TaskInputValue::Instant(ref value) => {
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
            TaskInputValue::Integer(ref value) => {
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
            TaskInputValue::Markdown(ref value) => {
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
            TaskInputValue::Oid(ref value) => {
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
            TaskInputValue::PositiveInt(ref value) => {
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
            TaskInputValue::String(ref value) => {
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
            TaskInputValue::Time(ref value) => {
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
            TaskInputValue::UnsignedInt(ref value) => {
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
            TaskInputValue::Uri(ref value) => {
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
            TaskInputValue::Url(ref value) => {
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
            TaskInputValue::Uuid(ref value) => {
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
            TaskInputValue::Address(ref value) => {
                state.serialize_entry("valueAddress", value)?;
            }
            TaskInputValue::Age(ref value) => {
                state.serialize_entry("valueAge", value)?;
            }
            TaskInputValue::Annotation(ref value) => {
                state.serialize_entry("valueAnnotation", value)?;
            }
            TaskInputValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            TaskInputValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            TaskInputValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            TaskInputValue::ContactPoint(ref value) => {
                state.serialize_entry("valueContactPoint", value)?;
            }
            TaskInputValue::Count(ref value) => {
                state.serialize_entry("valueCount", value)?;
            }
            TaskInputValue::Distance(ref value) => {
                state.serialize_entry("valueDistance", value)?;
            }
            TaskInputValue::Duration(ref value) => {
                state.serialize_entry("valueDuration", value)?;
            }
            TaskInputValue::HumanName(ref value) => {
                state.serialize_entry("valueHumanName", value)?;
            }
            TaskInputValue::Identifier(ref value) => {
                state.serialize_entry("valueIdentifier", value)?;
            }
            TaskInputValue::Money(ref value) => {
                state.serialize_entry("valueMoney", value)?;
            }
            TaskInputValue::Period(ref value) => {
                state.serialize_entry("valuePeriod", value)?;
            }
            TaskInputValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            TaskInputValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            TaskInputValue::Ratio(ref value) => {
                state.serialize_entry("valueRatio", value)?;
            }
            TaskInputValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            TaskInputValue::SampledData(ref value) => {
                state.serialize_entry("valueSampledData", value)?;
            }
            TaskInputValue::Signature(ref value) => {
                state.serialize_entry("valueSignature", value)?;
            }
            TaskInputValue::Timing(ref value) => {
                state.serialize_entry("valueTiming", value)?;
            }
            TaskInputValue::ContactDetail(ref value) => {
                state.serialize_entry("valueContactDetail", value)?;
            }
            TaskInputValue::Contributor(ref value) => {
                state.serialize_entry("valueContributor", value)?;
            }
            TaskInputValue::DataRequirement(ref value) => {
                state.serialize_entry("valueDataRequirement", value)?;
            }
            TaskInputValue::Expression(ref value) => {
                state.serialize_entry("valueExpression", value)?;
            }
            TaskInputValue::ParameterDefinition(ref value) => {
                state.serialize_entry("valueParameterDefinition", value)?;
            }
            TaskInputValue::RelatedArtifact(ref value) => {
                state.serialize_entry("valueRelatedArtifact", value)?;
            }
            TaskInputValue::TriggerDefinition(ref value) => {
                state.serialize_entry("valueTriggerDefinition", value)?;
            }
            TaskInputValue::UsageContext(ref value) => {
                state.serialize_entry("valueUsageContext", value)?;
            }
            TaskInputValue::Dosage(ref value) => {
                state.serialize_entry("valueDosage", value)?;
            }
            TaskInputValue::Meta(ref value) => {
                state.serialize_entry("valueMeta", value)?;
            }
            TaskInputValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for TaskInput {
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
            type Value = TaskInput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskInput")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TaskInput, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<TaskInputValue> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::ValueBase64Binary => {
                                let r#enum = r#value.get_or_insert(TaskInputValue::Base64Binary(
                                    Default::default(),
                                ));
                                if let TaskInputValue::Base64Binary(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskInputValue::Base64Binary(
                                    Default::default(),
                                ));
                                if let TaskInputValue::Base64Binary(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Boolean(Default::default()));
                                if let TaskInputValue::Boolean(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Boolean(Default::default()));
                                if let TaskInputValue::Boolean(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Canonical(Default::default()));
                                if let TaskInputValue::Canonical(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Canonical(Default::default()));
                                if let TaskInputValue::Canonical(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Code(Default::default()));
                                if let TaskInputValue::Code(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Code(Default::default()));
                                if let TaskInputValue::Code(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Date(Default::default()));
                                if let TaskInputValue::Date(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Date(Default::default()));
                                if let TaskInputValue::Date(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::DateTime(Default::default()));
                                if let TaskInputValue::DateTime(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::DateTime(Default::default()));
                                if let TaskInputValue::DateTime(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Decimal(Default::default()));
                                if let TaskInputValue::Decimal(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Decimal(Default::default()));
                                if let TaskInputValue::Decimal(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Id(Default::default()));
                                if let TaskInputValue::Id(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Id(Default::default()));
                                if let TaskInputValue::Id(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Instant(Default::default()));
                                if let TaskInputValue::Instant(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Instant(Default::default()));
                                if let TaskInputValue::Instant(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Integer(Default::default()));
                                if let TaskInputValue::Integer(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Integer(Default::default()));
                                if let TaskInputValue::Integer(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Markdown(Default::default()));
                                if let TaskInputValue::Markdown(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::Markdown(Default::default()));
                                if let TaskInputValue::Markdown(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Oid(Default::default()));
                                if let TaskInputValue::Oid(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Oid(Default::default()));
                                if let TaskInputValue::Oid(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::PositiveInt(Default::default()));
                                if let TaskInputValue::PositiveInt(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::PositiveInt(Default::default()));
                                if let TaskInputValue::PositiveInt(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::String(Default::default()));
                                if let TaskInputValue::String(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::String(Default::default()));
                                if let TaskInputValue::String(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Time(Default::default()));
                                if let TaskInputValue::Time(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Time(Default::default()));
                                if let TaskInputValue::Time(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::UnsignedInt(Default::default()));
                                if let TaskInputValue::UnsignedInt(variant) = r#enum {
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
                                    .get_or_insert(TaskInputValue::UnsignedInt(Default::default()));
                                if let TaskInputValue::UnsignedInt(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Uri(Default::default()));
                                if let TaskInputValue::Uri(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Uri(Default::default()));
                                if let TaskInputValue::Uri(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Url(Default::default()));
                                if let TaskInputValue::Url(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Url(Default::default()));
                                if let TaskInputValue::Url(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Uuid(Default::default()));
                                if let TaskInputValue::Uuid(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskInputValue::Uuid(Default::default()));
                                if let TaskInputValue::Uuid(variant) = r#enum {
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
                                r#value = Some(TaskInputValue::Address(map_access.next_value()?));
                            }
                            Field::ValueAge => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAge"));
                                }
                                r#value = Some(TaskInputValue::Age(map_access.next_value()?));
                            }
                            Field::ValueAnnotation => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAnnotation",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::Annotation(map_access.next_value()?));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::Attachment(map_access.next_value()?));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::CodeableConcept(map_access.next_value()?));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(TaskInputValue::Coding(map_access.next_value()?));
                            }
                            Field::ValueContactPoint => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactPoint",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::ContactPoint(map_access.next_value()?));
                            }
                            Field::ValueCount => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCount"));
                                }
                                r#value = Some(TaskInputValue::Count(map_access.next_value()?));
                            }
                            Field::ValueDistance => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDistance"));
                                }
                                r#value = Some(TaskInputValue::Distance(map_access.next_value()?));
                            }
                            Field::ValueDuration => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDuration"));
                                }
                                r#value = Some(TaskInputValue::Duration(map_access.next_value()?));
                            }
                            Field::ValueHumanName => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueHumanName",
                                    ));
                                }
                                r#value = Some(TaskInputValue::HumanName(map_access.next_value()?));
                            }
                            Field::ValueIdentifier => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueIdentifier",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::Identifier(map_access.next_value()?));
                            }
                            Field::ValueMoney => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMoney"));
                                }
                                r#value = Some(TaskInputValue::Money(map_access.next_value()?));
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(TaskInputValue::Period(map_access.next_value()?));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(TaskInputValue::Quantity(map_access.next_value()?));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(TaskInputValue::Range(map_access.next_value()?));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value = Some(TaskInputValue::Ratio(map_access.next_value()?));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(TaskInputValue::Reference(map_access.next_value()?));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::SampledData(map_access.next_value()?));
                            }
                            Field::ValueSignature => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSignature",
                                    ));
                                }
                                r#value = Some(TaskInputValue::Signature(map_access.next_value()?));
                            }
                            Field::ValueTiming => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTiming"));
                                }
                                r#value = Some(TaskInputValue::Timing(map_access.next_value()?));
                            }
                            Field::ValueContactDetail => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactDetail",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::ContactDetail(map_access.next_value()?));
                            }
                            Field::ValueContributor => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContributor",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::Contributor(map_access.next_value()?));
                            }
                            Field::ValueDataRequirement => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueDataRequirement",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::DataRequirement(map_access.next_value()?));
                            }
                            Field::ValueExpression => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueExpression",
                                    ));
                                }
                                r#value =
                                    Some(TaskInputValue::Expression(map_access.next_value()?));
                            }
                            Field::ValueParameterDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueParameterDefinition",
                                    ));
                                }
                                r#value = Some(TaskInputValue::ParameterDefinition(
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
                                    Some(TaskInputValue::RelatedArtifact(map_access.next_value()?));
                            }
                            Field::ValueTriggerDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueTriggerDefinition",
                                    ));
                                }
                                r#value = Some(TaskInputValue::TriggerDefinition(
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
                                    Some(TaskInputValue::UsageContext(map_access.next_value()?));
                            }
                            Field::ValueDosage => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDosage"));
                                }
                                r#value = Some(TaskInputValue::Dosage(map_access.next_value()?));
                            }
                            Field::ValueMeta => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMeta"));
                                }
                                r#value = Some(TaskInputValue::Meta(map_access.next_value()?));
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
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
                                }
                            }
                        }
                    }
                    Ok(TaskInput {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode == crate::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value: if config.mode == crate::DeserializationMode::Lax {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Outputs produced by the Task."]
#[derive(Default, Debug, Clone)]
pub struct TaskOutput {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The name of the Output parameter."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the Output parameter as a basic type."]
    pub r#value: TaskOutputValue,
}
impl crate::model::ResourceOrElement for TaskOutput {}
impl serde::ser::Serialize for TaskOutput {
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
        match self.r#value {
            TaskOutputValue::Base64Binary(ref value) => {
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
            TaskOutputValue::Boolean(ref value) => {
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
            TaskOutputValue::Canonical(ref value) => {
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
            TaskOutputValue::Code(ref value) => {
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
            TaskOutputValue::Date(ref value) => {
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
            TaskOutputValue::DateTime(ref value) => {
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
            TaskOutputValue::Decimal(ref value) => {
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
            TaskOutputValue::Id(ref value) => {
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
            TaskOutputValue::Instant(ref value) => {
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
            TaskOutputValue::Integer(ref value) => {
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
            TaskOutputValue::Markdown(ref value) => {
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
            TaskOutputValue::Oid(ref value) => {
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
            TaskOutputValue::PositiveInt(ref value) => {
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
            TaskOutputValue::String(ref value) => {
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
            TaskOutputValue::Time(ref value) => {
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
            TaskOutputValue::UnsignedInt(ref value) => {
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
            TaskOutputValue::Uri(ref value) => {
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
            TaskOutputValue::Url(ref value) => {
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
            TaskOutputValue::Uuid(ref value) => {
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
            TaskOutputValue::Address(ref value) => {
                state.serialize_entry("valueAddress", value)?;
            }
            TaskOutputValue::Age(ref value) => {
                state.serialize_entry("valueAge", value)?;
            }
            TaskOutputValue::Annotation(ref value) => {
                state.serialize_entry("valueAnnotation", value)?;
            }
            TaskOutputValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            TaskOutputValue::CodeableConcept(ref value) => {
                state.serialize_entry("valueCodeableConcept", value)?;
            }
            TaskOutputValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            TaskOutputValue::ContactPoint(ref value) => {
                state.serialize_entry("valueContactPoint", value)?;
            }
            TaskOutputValue::Count(ref value) => {
                state.serialize_entry("valueCount", value)?;
            }
            TaskOutputValue::Distance(ref value) => {
                state.serialize_entry("valueDistance", value)?;
            }
            TaskOutputValue::Duration(ref value) => {
                state.serialize_entry("valueDuration", value)?;
            }
            TaskOutputValue::HumanName(ref value) => {
                state.serialize_entry("valueHumanName", value)?;
            }
            TaskOutputValue::Identifier(ref value) => {
                state.serialize_entry("valueIdentifier", value)?;
            }
            TaskOutputValue::Money(ref value) => {
                state.serialize_entry("valueMoney", value)?;
            }
            TaskOutputValue::Period(ref value) => {
                state.serialize_entry("valuePeriod", value)?;
            }
            TaskOutputValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            TaskOutputValue::Range(ref value) => {
                state.serialize_entry("valueRange", value)?;
            }
            TaskOutputValue::Ratio(ref value) => {
                state.serialize_entry("valueRatio", value)?;
            }
            TaskOutputValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            TaskOutputValue::SampledData(ref value) => {
                state.serialize_entry("valueSampledData", value)?;
            }
            TaskOutputValue::Signature(ref value) => {
                state.serialize_entry("valueSignature", value)?;
            }
            TaskOutputValue::Timing(ref value) => {
                state.serialize_entry("valueTiming", value)?;
            }
            TaskOutputValue::ContactDetail(ref value) => {
                state.serialize_entry("valueContactDetail", value)?;
            }
            TaskOutputValue::Contributor(ref value) => {
                state.serialize_entry("valueContributor", value)?;
            }
            TaskOutputValue::DataRequirement(ref value) => {
                state.serialize_entry("valueDataRequirement", value)?;
            }
            TaskOutputValue::Expression(ref value) => {
                state.serialize_entry("valueExpression", value)?;
            }
            TaskOutputValue::ParameterDefinition(ref value) => {
                state.serialize_entry("valueParameterDefinition", value)?;
            }
            TaskOutputValue::RelatedArtifact(ref value) => {
                state.serialize_entry("valueRelatedArtifact", value)?;
            }
            TaskOutputValue::TriggerDefinition(ref value) => {
                state.serialize_entry("valueTriggerDefinition", value)?;
            }
            TaskOutputValue::UsageContext(ref value) => {
                state.serialize_entry("valueUsageContext", value)?;
            }
            TaskOutputValue::Dosage(ref value) => {
                state.serialize_entry("valueDosage", value)?;
            }
            TaskOutputValue::Meta(ref value) => {
                state.serialize_entry("valueMeta", value)?;
            }
            TaskOutputValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for TaskOutput {
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
            type Value = TaskOutput;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("TaskOutput")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<TaskOutput, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<TaskOutputValue> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
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
                            Field::ValueBase64Binary => {
                                let r#enum = r#value.get_or_insert(TaskOutputValue::Base64Binary(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::Base64Binary(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskOutputValue::Base64Binary(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::Base64Binary(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Boolean(Default::default()));
                                if let TaskOutputValue::Boolean(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Boolean(Default::default()));
                                if let TaskOutputValue::Boolean(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Canonical(Default::default()));
                                if let TaskOutputValue::Canonical(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Canonical(Default::default()));
                                if let TaskOutputValue::Canonical(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Code(Default::default()));
                                if let TaskOutputValue::Code(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Code(Default::default()));
                                if let TaskOutputValue::Code(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Date(Default::default()));
                                if let TaskOutputValue::Date(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Date(Default::default()));
                                if let TaskOutputValue::Date(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::DateTime(Default::default()));
                                if let TaskOutputValue::DateTime(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::DateTime(Default::default()));
                                if let TaskOutputValue::DateTime(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Decimal(Default::default()));
                                if let TaskOutputValue::Decimal(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Decimal(Default::default()));
                                if let TaskOutputValue::Decimal(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Id(Default::default()));
                                if let TaskOutputValue::Id(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Id(Default::default()));
                                if let TaskOutputValue::Id(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Instant(Default::default()));
                                if let TaskOutputValue::Instant(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Instant(Default::default()));
                                if let TaskOutputValue::Instant(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Integer(Default::default()));
                                if let TaskOutputValue::Integer(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Integer(Default::default()));
                                if let TaskOutputValue::Integer(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Markdown(Default::default()));
                                if let TaskOutputValue::Markdown(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::Markdown(Default::default()));
                                if let TaskOutputValue::Markdown(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Oid(Default::default()));
                                if let TaskOutputValue::Oid(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Oid(Default::default()));
                                if let TaskOutputValue::Oid(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskOutputValue::PositiveInt(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::PositiveInt(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskOutputValue::PositiveInt(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::PositiveInt(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::String(Default::default()));
                                if let TaskOutputValue::String(variant) = r#enum {
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
                                    .get_or_insert(TaskOutputValue::String(Default::default()));
                                if let TaskOutputValue::String(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Time(Default::default()));
                                if let TaskOutputValue::Time(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Time(Default::default()));
                                if let TaskOutputValue::Time(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskOutputValue::UnsignedInt(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::UnsignedInt(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(TaskOutputValue::UnsignedInt(
                                    Default::default(),
                                ));
                                if let TaskOutputValue::UnsignedInt(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Uri(Default::default()));
                                if let TaskOutputValue::Uri(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Uri(Default::default()));
                                if let TaskOutputValue::Uri(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Url(Default::default()));
                                if let TaskOutputValue::Url(variant) = r#enum {
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
                                    r#value.get_or_insert(TaskOutputValue::Url(Default::default()));
                                if let TaskOutputValue::Url(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Uuid(Default::default()));
                                if let TaskOutputValue::Uuid(variant) = r#enum {
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
                                let r#enum = r#value
                                    .get_or_insert(TaskOutputValue::Uuid(Default::default()));
                                if let TaskOutputValue::Uuid(variant) = r#enum {
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
                                r#value = Some(TaskOutputValue::Address(map_access.next_value()?));
                            }
                            Field::ValueAge => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueAge"));
                                }
                                r#value = Some(TaskOutputValue::Age(map_access.next_value()?));
                            }
                            Field::ValueAnnotation => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAnnotation",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Annotation(map_access.next_value()?));
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Attachment(map_access.next_value()?));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(TaskOutputValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(TaskOutputValue::Coding(map_access.next_value()?));
                            }
                            Field::ValueContactPoint => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactPoint",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::ContactPoint(map_access.next_value()?));
                            }
                            Field::ValueCount => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCount"));
                                }
                                r#value = Some(TaskOutputValue::Count(map_access.next_value()?));
                            }
                            Field::ValueDistance => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDistance"));
                                }
                                r#value = Some(TaskOutputValue::Distance(map_access.next_value()?));
                            }
                            Field::ValueDuration => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDuration"));
                                }
                                r#value = Some(TaskOutputValue::Duration(map_access.next_value()?));
                            }
                            Field::ValueHumanName => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueHumanName",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::HumanName(map_access.next_value()?));
                            }
                            Field::ValueIdentifier => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueIdentifier",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Identifier(map_access.next_value()?));
                            }
                            Field::ValueMoney => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMoney"));
                                }
                                r#value = Some(TaskOutputValue::Money(map_access.next_value()?));
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(TaskOutputValue::Period(map_access.next_value()?));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(TaskOutputValue::Quantity(map_access.next_value()?));
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(TaskOutputValue::Range(map_access.next_value()?));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value = Some(TaskOutputValue::Ratio(map_access.next_value()?));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Reference(map_access.next_value()?));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::SampledData(map_access.next_value()?));
                            }
                            Field::ValueSignature => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSignature",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Signature(map_access.next_value()?));
                            }
                            Field::ValueTiming => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTiming"));
                                }
                                r#value = Some(TaskOutputValue::Timing(map_access.next_value()?));
                            }
                            Field::ValueContactDetail => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContactDetail",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::ContactDetail(map_access.next_value()?));
                            }
                            Field::ValueContributor => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueContributor",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Contributor(map_access.next_value()?));
                            }
                            Field::ValueDataRequirement => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueDataRequirement",
                                    ));
                                }
                                r#value = Some(TaskOutputValue::DataRequirement(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueExpression => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueExpression",
                                    ));
                                }
                                r#value =
                                    Some(TaskOutputValue::Expression(map_access.next_value()?));
                            }
                            Field::ValueParameterDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueParameterDefinition",
                                    ));
                                }
                                r#value = Some(TaskOutputValue::ParameterDefinition(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRelatedArtifact => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueRelatedArtifact",
                                    ));
                                }
                                r#value = Some(TaskOutputValue::RelatedArtifact(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueTriggerDefinition => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueTriggerDefinition",
                                    ));
                                }
                                r#value = Some(TaskOutputValue::TriggerDefinition(
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
                                    Some(TaskOutputValue::UsageContext(map_access.next_value()?));
                            }
                            Field::ValueDosage => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDosage"));
                                }
                                r#value = Some(TaskOutputValue::Dosage(map_access.next_value()?));
                            }
                            Field::ValueMeta => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMeta"));
                                }
                                r#value = Some(TaskOutputValue::Meta(map_access.next_value()?));
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
                                    return Err(serde::de::Error::unknown_field(
                                        &key,
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
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
                                }
                            }
                        }
                    }
                    Ok(TaskOutput {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode == crate::DeserializationMode::Lax {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value: if config.mode == crate::DeserializationMode::Lax {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A task to be performed."]
#[derive(Default, Debug, Clone)]
pub struct Task {
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
    #[doc = "The business identifier for this task."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Task."]
    pub r#instantiates_canonical: Option<super::super::types::Canonical>,
    #[doc = "The URL pointing to an *externally* maintained  protocol, guideline, orderset or other definition that is adhered to in whole or in part by this Task."]
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    #[doc = "BasedOn refers to a higher-level authorization that triggered the creation of the task.  It references a \"request\" resource such as a ServiceRequest, MedicationRequest, ServiceRequest, CarePlan, etc. which is distinct from the \"request\" resource the task is seeking to fulfill.  This latter resource is referenced by FocusOn.  For example, based on a ServiceRequest (= BasedOn), a task is created to fulfill a procedureRequest ( = FocusOn ) to collect a specimen from a patient."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "An identifier that links together multiple tasks and other requests that were created in the same context."]
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Task that this particular task is part of."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "The current status of the task."]
    pub r#status: super::super::types::Code,
    #[doc = "An explanation as to why this task is held, failed, was refused, etc."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Contains business-specific nuances of the business state."]
    pub r#business_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the \"level\" of actionability associated with the Task, i.e. i+R`9`s this a proposed task, a planned task, an actionable task, etc."]
    pub r#intent: super::super::types::Code,
    #[doc = "Indicates how quickly the Task should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A name or code (or both) briefly describing what the task involves."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A free-text description of what is to be performed."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The request being actioned or the resource being manipulated by this task."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "The entity who benefits from the performance of the service specified in the task (e.g., the patient)."]
    pub r#for: Option<Box<super::super::types::Reference>>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) during which this task was created."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the time action was first taken against the task (start) and/or the time final action was taken against the task prior to marking it as completed (end)."]
    pub r#execution_period: Option<Box<super::super::types::Period>>,
    #[doc = "The date and time this task was created."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The date and time of last modification to this task."]
    pub r#last_modified: Option<super::super::types::DateTime>,
    #[doc = "The creator of the task."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "The kind of participant that should perform the task."]
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Individual organization or Device currently responsible for task execution."]
    pub r#owner: Option<Box<super::super::types::Reference>>,
    #[doc = "Principal physical location where the this task is performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "A description or code indicating why this task needs to be performed."]
    pub r#reason_code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A resource reference indicating why this task needs to be performed."]
    pub r#reason_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Insurance plans, coverage extensions, pre-authorizations and/or pre-determinations that may be relevant to the Task."]
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    #[doc = "Free-text information captured about the task as it progresses."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Links to Provenance records for past versions of this Task that identify key state transitions or updates that are likely to be relevant to a user looking at the current version of the task."]
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    #[doc = "If the Task.focus is a request resource and the task is seeking fulfillment (i.e. is asking for the request to be actioned), this element identifies any limitations on what parts of the referenced request should be actioned."]
    pub r#restriction: Option<TaskRestriction>,
    #[doc = "Additional information that may be needed in the execution of the task."]
    pub r#input: Vec<TaskInput>,
    #[doc = "Outputs produced by the Task."]
    pub r#output: Vec<TaskOutput>,
}
impl crate::model::ResourceOrElement for Task {}
impl serde::ser::Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Task")?;
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
        if let Some(some) = self.r#instantiates_canonical.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("instantiatesCanonical", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_instantiatesCanonical", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#instantiates_uri.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("instantiatesUri", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_instantiatesUri", &primitive_element)?;
            }
        }
        if !self.r#based_on.is_empty() {
            state.serialize_entry("basedOn", &self.r#based_on)?;
        }
        if let Some(some) = self.r#group_identifier.as_ref() {
            state.serialize_entry("groupIdentifier", some)?;
        }
        if !self.r#part_of.is_empty() {
            state.serialize_entry("partOf", &self.r#part_of)?;
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
        if let Some(some) = self.r#status_reason.as_ref() {
            state.serialize_entry("statusReason", some)?;
        }
        if let Some(some) = self.r#business_status.as_ref() {
            state.serialize_entry("businessStatus", some)?;
        }
        if let Some(some) = self.r#intent.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("intent", &some)?;
        }
        if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#intent.id,
                extension: &self.r#intent.extension,
            };
            state.serialize_entry("_intent", &primitive_element)?;
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("priority", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#code.as_ref() {
            state.serialize_entry("code", some)?;
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("description", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#focus.as_ref() {
            state.serialize_entry("focus", some)?;
        }
        if let Some(some) = self.r#for.as_ref() {
            state.serialize_entry("for", some)?;
        }
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#execution_period.as_ref() {
            state.serialize_entry("executionPeriod", some)?;
        }
        if let Some(some) = self.r#authored_on.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("authoredOn", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authoredOn", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_modified.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("lastModified", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastModified", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requester.as_ref() {
            state.serialize_entry("requester", some)?;
        }
        if !self.r#performer_type.is_empty() {
            state.serialize_entry("performerType", &self.r#performer_type)?;
        }
        if let Some(some) = self.r#owner.as_ref() {
            state.serialize_entry("owner", some)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if let Some(some) = self.r#reason_code.as_ref() {
            state.serialize_entry("reasonCode", some)?;
        }
        if let Some(some) = self.r#reason_reference.as_ref() {
            state.serialize_entry("reasonReference", some)?;
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#relevant_history.is_empty() {
            state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
        }
        if let Some(some) = self.r#restriction.as_ref() {
            state.serialize_entry("restriction", some)?;
        }
        if !self.r#input.is_empty() {
            state.serialize_entry("input", &self.r#input)?;
        }
        if !self.r#output.is_empty() {
            state.serialize_entry("output", &self.r#output)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Task {
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "groupIdentifier")]
            GroupIdentifier,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "businessStatus")]
            BusinessStatus,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "_intent")]
            IntentPrimitiveElement,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "focus")]
            Focus,
            #[serde(rename = "for")]
            For,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "executionPeriod")]
            ExecutionPeriod,
            #[serde(rename = "authoredOn")]
            AuthoredOn,
            #[serde(rename = "_authoredOn")]
            AuthoredOnPrimitiveElement,
            #[serde(rename = "lastModified")]
            LastModified,
            #[serde(rename = "_lastModified")]
            LastModifiedPrimitiveElement,
            #[serde(rename = "requester")]
            Requester,
            #[serde(rename = "performerType")]
            PerformerType,
            #[serde(rename = "owner")]
            Owner,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "relevantHistory")]
            RelevantHistory,
            #[serde(rename = "restriction")]
            Restriction,
            #[serde(rename = "input")]
            Input,
            #[serde(rename = "output")]
            Output,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Task;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Task")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Task, V::Error>
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
                let mut r#instantiates_canonical: Option<super::super::types::Canonical> = None;
                let mut r#instantiates_uri: Option<super::super::types::Uri> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#group_identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#business_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#focus: Option<Box<super::super::types::Reference>> = None;
                let mut r#for: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#execution_period: Option<Box<super::super::types::Period>> = None;
                let mut r#authored_on: Option<super::super::types::DateTime> = None;
                let mut r#last_modified: Option<super::super::types::DateTime> = None;
                let mut r#requester: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#owner: Option<Box<super::super::types::Reference>> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#reason_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reason_reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#insurance: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#relevant_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#restriction: Option<TaskRestriction> = None;
                let mut r#input: Option<Vec<TaskInput>> = None;
                let mut r#output: Option<Vec<TaskOutput>> = None;
                crate::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Task" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Task",
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
                            Field::InstantiatesCanonical => {
                                let some =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                let some =
                                    r#instantiates_canonical.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::InstantiatesUri => {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::GroupIdentifier => {
                                if r#group_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupIdentifier",
                                    ));
                                }
                                r#group_identifier = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
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
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::BusinessStatus => {
                                if r#business_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "businessStatus",
                                    ));
                                }
                                r#business_status = Some(map_access.next_value()?);
                            }
                            Field::Intent => {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::IntentPrimitiveElement => {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_intent"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Priority => {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PriorityPrimitiveElement => {
                                let some = r#priority.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_priority"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DescriptionPrimitiveElement => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
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
                            Field::For => {
                                if r#for.is_some() {
                                    return Err(serde::de::Error::duplicate_field("for"));
                                }
                                r#for = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::ExecutionPeriod => {
                                if r#execution_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "executionPeriod",
                                    ));
                                }
                                r#execution_period = Some(map_access.next_value()?);
                            }
                            Field::AuthoredOn => {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authoredOn"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AuthoredOnPrimitiveElement => {
                                let some = r#authored_on.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_authoredOn"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::LastModified => {
                                let some = r#last_modified.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lastModified"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LastModifiedPrimitiveElement => {
                                let some = r#last_modified.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_lastModified"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Requester => {
                                if r#requester.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requester"));
                                }
                                r#requester = Some(map_access.next_value()?);
                            }
                            Field::PerformerType => {
                                if r#performer_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerType"));
                                }
                                r#performer_type = Some(map_access.next_value()?);
                            }
                            Field::Owner => {
                                if r#owner.is_some() {
                                    return Err(serde::de::Error::duplicate_field("owner"));
                                }
                                r#owner = Some(map_access.next_value()?);
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code = Some(map_access.next_value()?);
                            }
                            Field::ReasonReference => {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::Insurance => {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::RelevantHistory => {
                                if r#relevant_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relevantHistory",
                                    ));
                                }
                                r#relevant_history = Some(map_access.next_value()?);
                            }
                            Field::Restriction => {
                                if r#restriction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("restriction"));
                                }
                                r#restriction = Some(map_access.next_value()?);
                            }
                            Field::Input => {
                                if r#input.is_some() {
                                    return Err(serde::de::Error::duplicate_field("input"));
                                }
                                r#input = Some(map_access.next_value()?);
                            }
                            Field::Output => {
                                if r#output.is_some() {
                                    return Err(serde::de::Error::duplicate_field("output"));
                                }
                                r#output = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => {
                                if config.mode == crate::DeserializationMode::Strict {
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
                                            "instantiatesCanonical",
                                            "instantiatesUri",
                                            "basedOn",
                                            "groupIdentifier",
                                            "partOf",
                                            "status",
                                            "statusReason",
                                            "businessStatus",
                                            "intent",
                                            "priority",
                                            "code",
                                            "description",
                                            "focus",
                                            "for",
                                            "encounter",
                                            "executionPeriod",
                                            "authoredOn",
                                            "lastModified",
                                            "requester",
                                            "performerType",
                                            "owner",
                                            "location",
                                            "reasonCode",
                                            "reasonReference",
                                            "insurance",
                                            "note",
                                            "relevantHistory",
                                            "restriction",
                                            "input",
                                            "output",
                                        ],
                                    ));
                                }
                            }
                        }
                    }
                    Ok(Task {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#instantiates_canonical,
                        r#instantiates_uri,
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#group_identifier,
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#status: if config.mode == crate::DeserializationMode::Lax {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#business_status,
                        r#intent: if config.mode == crate::DeserializationMode::Lax {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#priority,
                        r#code,
                        r#description,
                        r#focus,
                        r#for,
                        r#encounter,
                        r#execution_period,
                        r#authored_on,
                        r#last_modified,
                        r#requester,
                        r#performer_type: r#performer_type.unwrap_or(vec![]),
                        r#owner,
                        r#location,
                        r#reason_code,
                        r#reason_reference,
                        r#insurance: r#insurance.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                        r#restriction,
                        r#input: r#input.unwrap_or(vec![]),
                        r#output: r#output.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
