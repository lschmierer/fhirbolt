// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "A value that the referenced question is tested using the specified operator in order for the item to be enabled."]
#[derive(Debug, Clone)]
pub enum QuestionnaireItemEnableWhenAnswer {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for QuestionnaireItemEnableWhenAnswer {
    fn default() -> QuestionnaireItemEnableWhenAnswer {
        QuestionnaireItemEnableWhenAnswer::Invalid
    }
}
#[doc = "A potential answer that's allowed as the answer to this question."]
#[derive(Debug, Clone)]
pub enum QuestionnaireItemAnswerOptionValue {
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for QuestionnaireItemAnswerOptionValue {
    fn default() -> QuestionnaireItemAnswerOptionValue {
        QuestionnaireItemAnswerOptionValue::Invalid
    }
}
#[doc = "The actual value to for an initial answer."]
#[derive(Debug, Clone)]
pub enum QuestionnaireItemInitialValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for QuestionnaireItemInitialValue {
    fn default() -> QuestionnaireItemInitialValue {
        QuestionnaireItemInitialValue::Invalid
    }
}
#[doc = "A constraint indicating that this item should only be enabled (displayed/allow answers to be captured) when the specified condition is true."]
#[derive(Default, Debug, Clone)]
pub struct QuestionnaireItemEnableWhen {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The linkId for the question whose answer (or lack of answer) governs whether this item is enabled."]
    pub r#question: super::super::types::String,
    #[doc = "Specifies the criteria by which the question is enabled."]
    pub r#operator: super::super::types::Code,
    #[doc = "A value that the referenced question is tested using the specified operator in order for the item to be enabled."]
    pub r#answer: QuestionnaireItemEnableWhenAnswer,
}
impl crate::AnyResource for QuestionnaireItemEnableWhen {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for QuestionnaireItemEnableWhen {
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
        if let Some(some) = self.r#question.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("question", &some)?;
        }
        if self.r#question.id.is_some() || !self.r#question.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#question.id,
                extension: &self.r#question.extension,
            };
            state.serialize_entry("_question", &primitive_element)?;
        }
        if let Some(some) = self.r#operator.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("operator", &some)?;
        }
        if self.r#operator.id.is_some() || !self.r#operator.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#operator.id,
                extension: &self.r#operator.extension,
            };
            state.serialize_entry("_operator", &primitive_element)?;
        }
        match self.r#answer {
            QuestionnaireItemEnableWhenAnswer::Boolean(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerBoolean", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerBoolean", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::Decimal(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("answerDecimal", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerDecimal", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::Integer(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerInteger", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerInteger", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::Date(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerDate", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerDate", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::DateTime(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerDateTime", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerDateTime", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::Time(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerTime", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerTime", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::String(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("answerString", &some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_answerString", &primitive_element)?;
                }
            }
            QuestionnaireItemEnableWhenAnswer::Coding(ref value) => {
                state.serialize_entry("answerCoding", value)?;
            }
            QuestionnaireItemEnableWhenAnswer::Quantity(ref value) => {
                state.serialize_entry("answerQuantity", value)?;
            }
            QuestionnaireItemEnableWhenAnswer::Reference(ref value) => {
                state.serialize_entry("answerReference", value)?;
            }
            QuestionnaireItemEnableWhenAnswer::Invalid => {
                return Err(serde::ser::Error::custom("answer is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for QuestionnaireItemEnableWhen {
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
            #[serde(rename = "question")]
            Question,
            #[serde(rename = "_question")]
            QuestionPrimitiveElement,
            #[serde(rename = "operator")]
            Operator,
            #[serde(rename = "_operator")]
            OperatorPrimitiveElement,
            #[serde(rename = "answerBoolean")]
            AnswerBoolean,
            #[serde(rename = "_answerBoolean")]
            AnswerBooleanPrimitiveElement,
            #[serde(rename = "answerDecimal")]
            AnswerDecimal,
            #[serde(rename = "_answerDecimal")]
            AnswerDecimalPrimitiveElement,
            #[serde(rename = "answerInteger")]
            AnswerInteger,
            #[serde(rename = "_answerInteger")]
            AnswerIntegerPrimitiveElement,
            #[serde(rename = "answerDate")]
            AnswerDate,
            #[serde(rename = "_answerDate")]
            AnswerDatePrimitiveElement,
            #[serde(rename = "answerDateTime")]
            AnswerDateTime,
            #[serde(rename = "_answerDateTime")]
            AnswerDateTimePrimitiveElement,
            #[serde(rename = "answerTime")]
            AnswerTime,
            #[serde(rename = "_answerTime")]
            AnswerTimePrimitiveElement,
            #[serde(rename = "answerString")]
            AnswerString,
            #[serde(rename = "_answerString")]
            AnswerStringPrimitiveElement,
            #[serde(rename = "answerCoding")]
            AnswerCoding,
            #[serde(rename = "answerQuantity")]
            AnswerQuantity,
            #[serde(rename = "answerReference")]
            AnswerReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuestionnaireItemEnableWhen;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("QuestionnaireItemEnableWhen")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<QuestionnaireItemEnableWhen, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#question: Option<super::super::types::String> = None;
                let mut r#operator: Option<super::super::types::Code> = None;
                let mut r#answer: Option<QuestionnaireItemEnableWhenAnswer> = None;
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
                            Field::Question => {
                                let some = r#question.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("question"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::QuestionPrimitiveElement => {
                                let some = r#question.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_question"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Operator => {
                                let some = r#operator.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operator"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::OperatorPrimitiveElement => {
                                let some = r#operator.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_operator"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::AnswerBoolean => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Boolean(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Boolean(variant) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerBooleanPrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Boolean(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Boolean(variant) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerBoolean",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerDecimal => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Decimal(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Decimal(variant) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerDecimalPrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Decimal(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Decimal(variant) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerDecimal",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerInteger => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Integer(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Integer(variant) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerInteger",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerIntegerPrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Integer(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Integer(variant) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerInteger",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerDate => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Date(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerDatePrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Date(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerDateTime => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::DateTime(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::DateTime(variant) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerDateTimePrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::DateTime(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::DateTime(variant) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerDateTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerTime => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Time(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerTimePrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::Time(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerTime",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerString => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::String(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "answerString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("answer[x]"));
                                }
                            }
                            Field::AnswerStringPrimitiveElement => {
                                let r#enum = r#answer.get_or_insert(
                                    QuestionnaireItemEnableWhenAnswer::String(Default::default()),
                                );
                                if let QuestionnaireItemEnableWhenAnswer::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_answerString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_answer[x]"));
                                }
                            }
                            Field::AnswerCoding => {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answerCoding"));
                                }
                                r#answer = Some(QuestionnaireItemEnableWhenAnswer::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AnswerQuantity => {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "answerQuantity",
                                    ));
                                }
                                r#answer = Some(QuestionnaireItemEnableWhenAnswer::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::AnswerReference => {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "answerReference",
                                    ));
                                }
                                r#answer = Some(QuestionnaireItemEnableWhenAnswer::Reference(
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
                                        "question",
                                        "operator",
                                        "answerBoolean",
                                        "answerDecimal",
                                        "answerInteger",
                                        "answerDate",
                                        "answerDateTime",
                                        "answerTime",
                                        "answerString",
                                        "answerCoding",
                                        "answerQuantity",
                                        "answerReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(QuestionnaireItemEnableWhen {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#question: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#question.unwrap_or(Default::default())
                        } else {
                            r#question.ok_or(serde::de::Error::missing_field("question"))?
                        },
                        r#operator: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#operator.unwrap_or(Default::default())
                        } else {
                            r#operator.ok_or(serde::de::Error::missing_field("operator"))?
                        },
                        r#answer: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#answer.unwrap_or(Default::default())
                        } else {
                            r#answer.ok_or(serde::de::Error::missing_field("answer[x]"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "One of the permitted answers for a \"choice\" or \"open-choice\" question."]
#[derive(Default, Debug, Clone)]
pub struct QuestionnaireItemAnswerOption {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A potential answer that's allowed as the answer to this question."]
    pub r#value: QuestionnaireItemAnswerOptionValue,
    #[doc = "Indicates whether the answer value is selected when the list of possible answers is initially shown."]
    pub r#initial_selected: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for QuestionnaireItemAnswerOption {
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
        match self.r#value {
            QuestionnaireItemAnswerOptionValue::Integer(ref value) => {
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
            QuestionnaireItemAnswerOptionValue::Date(ref value) => {
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
            QuestionnaireItemAnswerOptionValue::Time(ref value) => {
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
            QuestionnaireItemAnswerOptionValue::String(ref value) => {
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
            QuestionnaireItemAnswerOptionValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            QuestionnaireItemAnswerOptionValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            QuestionnaireItemAnswerOptionValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        if let Some(some) = self.r#initial_selected.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("initialSelected", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_initialSelected", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for QuestionnaireItemAnswerOption {
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
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueReference")]
            ValueReference,
            #[serde(rename = "initialSelected")]
            InitialSelected,
            #[serde(rename = "_initialSelected")]
            InitialSelectedPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuestionnaireItemAnswerOption;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("QuestionnaireItemAnswerOption")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<QuestionnaireItemAnswerOption, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<QuestionnaireItemAnswerOptionValue> = None;
                let mut r#initial_selected: Option<super::super::types::Boolean> = None;
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
                            Field::ValueInteger => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Integer(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Integer(variant) = r#enum
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
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Integer(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Integer(variant) = r#enum
                                {
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
                            Field::ValueDate => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Date(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Date(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Date(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Date(variant) = r#enum {
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
                            Field::ValueTime => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Time(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Time(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::Time(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::Time(variant) = r#enum {
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
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::String(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::String(variant) = r#enum
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
                            }
                            Field::ValueStringPrimitiveElement => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemAnswerOptionValue::String(Default::default()),
                                );
                                if let QuestionnaireItemAnswerOptionValue::String(variant) = r#enum
                                {
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
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(QuestionnaireItemAnswerOptionValue::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(QuestionnaireItemAnswerOptionValue::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::InitialSelected => {
                                let some = r#initial_selected.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "initialSelected",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::InitialSelectedPrimitiveElement => {
                                let some = r#initial_selected.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_initialSelected",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                                        "valueInteger",
                                        "valueDate",
                                        "valueTime",
                                        "valueString",
                                        "valueCoding",
                                        "valueReference",
                                        "initialSelected",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(QuestionnaireItemAnswerOption {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                        r#initial_selected,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "One or more values that should be pre-populated in the answer when initially rendering the questionnaire for user input."]
#[derive(Default, Debug, Clone)]
pub struct QuestionnaireItemInitial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual value to for an initial answer."]
    pub r#value: QuestionnaireItemInitialValue,
}
impl serde::ser::Serialize for QuestionnaireItemInitial {
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
        match self.r#value {
            QuestionnaireItemInitialValue::Boolean(ref value) => {
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
            QuestionnaireItemInitialValue::Decimal(ref value) => {
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
            QuestionnaireItemInitialValue::Integer(ref value) => {
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
            QuestionnaireItemInitialValue::Date(ref value) => {
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
            QuestionnaireItemInitialValue::DateTime(ref value) => {
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
            QuestionnaireItemInitialValue::Time(ref value) => {
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
            QuestionnaireItemInitialValue::String(ref value) => {
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
            QuestionnaireItemInitialValue::Uri(ref value) => {
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
            QuestionnaireItemInitialValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            QuestionnaireItemInitialValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            QuestionnaireItemInitialValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            QuestionnaireItemInitialValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
            QuestionnaireItemInitialValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for QuestionnaireItemInitial {
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
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueDecimal")]
            ValueDecimal,
            #[serde(rename = "_valueDecimal")]
            ValueDecimalPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueUri")]
            ValueUri,
            #[serde(rename = "_valueUri")]
            ValueUriPrimitiveElement,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "valueCoding")]
            ValueCoding,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueReference")]
            ValueReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuestionnaireItemInitial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("QuestionnaireItemInitial")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<QuestionnaireItemInitial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<QuestionnaireItemInitialValue> = None;
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
                            Field::ValueBoolean => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Boolean(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Boolean(variant) = r#enum {
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
                                    QuestionnaireItemInitialValue::Boolean(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Boolean(variant) = r#enum {
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
                            Field::ValueDecimal => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Decimal(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Decimal(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Decimal(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Decimal(variant) = r#enum {
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
                            Field::ValueInteger => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Integer(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Integer(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Integer(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Integer(variant) = r#enum {
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
                            Field::ValueDate => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Date(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Date(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Date(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Date(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::DateTime(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::DateTime(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::DateTime(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::DateTime(variant) = r#enum {
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
                            Field::ValueTime => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Time(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Time(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Time(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Time(variant) = r#enum {
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
                            Field::ValueString => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::String(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::String(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::String(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::String(variant) = r#enum {
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
                            Field::ValueUri => {
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Uri(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Uri(variant) = r#enum {
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
                                let r#enum = r#value.get_or_insert(
                                    QuestionnaireItemInitialValue::Uri(Default::default()),
                                );
                                if let QuestionnaireItemInitialValue::Uri(variant) = r#enum {
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
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value = Some(QuestionnaireItemInitialValue::Attachment(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCoding => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCoding"));
                                }
                                r#value = Some(QuestionnaireItemInitialValue::Coding(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(QuestionnaireItemInitialValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueReference => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueReference",
                                    ));
                                }
                                r#value = Some(QuestionnaireItemInitialValue::Reference(
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
                                        "valueBoolean",
                                        "valueDecimal",
                                        "valueInteger",
                                        "valueDate",
                                        "valueDateTime",
                                        "valueTime",
                                        "valueString",
                                        "valueUri",
                                        "valueAttachment",
                                        "valueCoding",
                                        "valueQuantity",
                                        "valueReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(QuestionnaireItemInitial {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#value: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
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
#[doc = "A particular question, question grouping or display text that is part of the questionnaire."]
#[derive(Default, Debug, Clone)]
pub struct QuestionnaireItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An identifier that is unique within the Questionnaire allowing linkage to the equivalent item in a QuestionnaireResponse resource."]
    pub r#link_id: super::super::types::String,
    #[doc = "This element is a URI that refers to an [ElementDefinition](https://hl7.org/FHIR/elementdefinition.html)) that provides information about this item, including information that might otherwise be included in the instance of the Questionnaire resource. A detailed description of the construction of the URI is shown in Comments, below. If this element is present then the following element values MAY be derived from the Element Definition if the corresponding elements of this Questionnaire resource instance have no value:\n\n* code (ElementDefinition.code) \n* type (ElementDefinition.type) \n* required (ElementDefinition.min) \n* repeats (ElementDefinition.max) \n* maxLength (ElementDefinition.maxLength) \n* answerValueSet (ElementDefinition.binding)\n* options (ElementDefinition.binding)."]
    pub r#definition: Option<super::super::types::Uri>,
    #[doc = "A terminology code that corresponds to this group or question (e.g. a code from LOINC, which defines many questions and answers)."]
    pub r#code: Vec<Box<super::super::types::Coding>>,
    #[doc = "A short label for a particular group, question or set of display text within the questionnaire used for reference by the individual completing the questionnaire."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The name of a section, the text of a question or text content for a display item."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "The type of questionnaire item this is - whether text for display, a grouping of other items or a particular type of data to be captured (string, integer, coded choice, etc.)."]
    pub r#type: super::super::types::Code,
    #[doc = "A constraint indicating that this item should only be enabled (displayed/allow answers to be captured) when the specified condition is true."]
    pub r#enable_when: Vec<QuestionnaireItemEnableWhen>,
    #[doc = "Controls how multiple enableWhen values are interpreted -  whether all or any must be true."]
    pub r#enable_behavior: Option<super::super::types::Code>,
    #[doc = "An indication, if true, that the item must be present in a \"completed\" QuestionnaireResponse.  If false, the item may be skipped when answering the questionnaire."]
    pub r#required: Option<super::super::types::Boolean>,
    #[doc = "An indication, if true, that the item may occur multiple times in the response, collecting multiple answers for questions or multiple sets of answers for groups."]
    pub r#repeats: Option<super::super::types::Boolean>,
    #[doc = "An indication, when true, that the value cannot be changed by a human respondent to the Questionnaire."]
    pub r#read_only: Option<super::super::types::Boolean>,
    #[doc = "The maximum number of characters that are permitted in the answer to be considered a \"valid\" QuestionnaireResponse."]
    pub r#max_length: Option<super::super::types::Integer>,
    #[doc = "A reference to a value set containing a list of codes representing permitted answers for a \"choice\" or \"open-choice\" question."]
    pub r#answer_value_set: Option<super::super::types::Canonical>,
    #[doc = "One of the permitted answers for a \"choice\" or \"open-choice\" question."]
    pub r#answer_option: Vec<QuestionnaireItemAnswerOption>,
    #[doc = "One or more values that should be pre-populated in the answer when initially rendering the questionnaire for user input."]
    pub r#initial: Vec<QuestionnaireItemInitial>,
    #[doc = "Text, questions and other groups to be nested beneath a question or group."]
    pub r#item: Vec<QuestionnaireItem>,
}
impl serde::ser::Serialize for QuestionnaireItem {
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
        if let Some(some) = self.r#link_id.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("linkId", &some)?;
        }
        if self.r#link_id.id.is_some() || !self.r#link_id.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#link_id.id,
                extension: &self.r#link_id.extension,
            };
            state.serialize_entry("_linkId", &primitive_element)?;
        }
        if let Some(some) = self.r#definition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("definition", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_definition", &primitive_element)?;
            }
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if let Some(some) = self.r#prefix.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("prefix", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_prefix", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("text", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
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
        if !self.r#enable_when.is_empty() {
            state.serialize_entry("enableWhen", &self.r#enable_when)?;
        }
        if let Some(some) = self.r#enable_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("enableBehavior", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_enableBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#required.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("required", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_required", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#repeats.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("repeats", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_repeats", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#read_only.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("readOnly", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_readOnly", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#max_length.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("maxLength", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_maxLength", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#answer_value_set.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("answerValueSet", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_answerValueSet", &primitive_element)?;
            }
        }
        if !self.r#answer_option.is_empty() {
            state.serialize_entry("answerOption", &self.r#answer_option)?;
        }
        if !self.r#initial.is_empty() {
            state.serialize_entry("initial", &self.r#initial)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for QuestionnaireItem {
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
            #[serde(rename = "linkId")]
            LinkId,
            #[serde(rename = "_linkId")]
            LinkIdPrimitiveElement,
            #[serde(rename = "definition")]
            Definition,
            #[serde(rename = "_definition")]
            DefinitionPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "prefix")]
            Prefix,
            #[serde(rename = "_prefix")]
            PrefixPrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "enableWhen")]
            EnableWhen,
            #[serde(rename = "enableBehavior")]
            EnableBehavior,
            #[serde(rename = "_enableBehavior")]
            EnableBehaviorPrimitiveElement,
            #[serde(rename = "required")]
            Required,
            #[serde(rename = "_required")]
            RequiredPrimitiveElement,
            #[serde(rename = "repeats")]
            Repeats,
            #[serde(rename = "_repeats")]
            RepeatsPrimitiveElement,
            #[serde(rename = "readOnly")]
            ReadOnly,
            #[serde(rename = "_readOnly")]
            ReadOnlyPrimitiveElement,
            #[serde(rename = "maxLength")]
            MaxLength,
            #[serde(rename = "_maxLength")]
            MaxLengthPrimitiveElement,
            #[serde(rename = "answerValueSet")]
            AnswerValueSet,
            #[serde(rename = "_answerValueSet")]
            AnswerValueSetPrimitiveElement,
            #[serde(rename = "answerOption")]
            AnswerOption,
            #[serde(rename = "initial")]
            Initial,
            #[serde(rename = "item")]
            Item,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = QuestionnaireItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("QuestionnaireItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<QuestionnaireItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#link_id: Option<super::super::types::String> = None;
                let mut r#definition: Option<super::super::types::Uri> = None;
                let mut r#code: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#prefix: Option<super::super::types::String> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#enable_when: Option<Vec<QuestionnaireItemEnableWhen>> = None;
                let mut r#enable_behavior: Option<super::super::types::Code> = None;
                let mut r#required: Option<super::super::types::Boolean> = None;
                let mut r#repeats: Option<super::super::types::Boolean> = None;
                let mut r#read_only: Option<super::super::types::Boolean> = None;
                let mut r#max_length: Option<super::super::types::Integer> = None;
                let mut r#answer_value_set: Option<super::super::types::Canonical> = None;
                let mut r#answer_option: Option<Vec<QuestionnaireItemAnswerOption>> = None;
                let mut r#initial: Option<Vec<QuestionnaireItemInitial>> = None;
                let mut r#item: Option<Vec<QuestionnaireItem>> = None;
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
                            Field::LinkId => {
                                let some = r#link_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LinkIdPrimitiveElement => {
                                let some = r#link_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Definition => {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DefinitionPrimitiveElement => {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_definition"));
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
                            Field::Prefix => {
                                let some = r#prefix.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prefix"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PrefixPrimitiveElement => {
                                let some = r#prefix.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_prefix"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Text => {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TextPrimitiveElement => {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                            Field::EnableWhen => {
                                if r#enable_when.is_some() {
                                    return Err(serde::de::Error::duplicate_field("enableWhen"));
                                }
                                r#enable_when = Some(map_access.next_value()?);
                            }
                            Field::EnableBehavior => {
                                let some = r#enable_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "enableBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::EnableBehaviorPrimitiveElement => {
                                let some = r#enable_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_enableBehavior",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Required => {
                                let some = r#required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("required"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RequiredPrimitiveElement => {
                                let some = r#required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_required"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Repeats => {
                                let some = r#repeats.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repeats"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RepeatsPrimitiveElement => {
                                let some = r#repeats.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_repeats"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ReadOnly => {
                                let some = r#read_only.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("readOnly"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ReadOnlyPrimitiveElement => {
                                let some = r#read_only.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_readOnly"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::MaxLength => {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxLength"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::MaxLengthPrimitiveElement => {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_maxLength"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::AnswerValueSet => {
                                let some = r#answer_value_set.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "answerValueSet",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AnswerValueSetPrimitiveElement => {
                                let some = r#answer_value_set.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_answerValueSet",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::AnswerOption => {
                                if r#answer_option.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answerOption"));
                                }
                                r#answer_option = Some(map_access.next_value()?);
                            }
                            Field::Initial => {
                                if r#initial.is_some() {
                                    return Err(serde::de::Error::duplicate_field("initial"));
                                }
                                r#initial = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
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
                                        "linkId",
                                        "definition",
                                        "code",
                                        "prefix",
                                        "text",
                                        "type",
                                        "enableWhen",
                                        "enableBehavior",
                                        "required",
                                        "repeats",
                                        "readOnly",
                                        "maxLength",
                                        "answerValueSet",
                                        "answerOption",
                                        "initial",
                                        "item",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(QuestionnaireItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#link_id: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#link_id.unwrap_or(Default::default())
                        } else {
                            r#link_id.ok_or(serde::de::Error::missing_field("linkId"))?
                        },
                        r#definition,
                        r#code: r#code.unwrap_or(vec![]),
                        r#prefix,
                        r#text,
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#enable_when: r#enable_when.unwrap_or(vec![]),
                        r#enable_behavior,
                        r#required,
                        r#repeats,
                        r#read_only,
                        r#max_length,
                        r#answer_value_set,
                        r#answer_option: r#answer_option.unwrap_or(vec![]),
                        r#initial: r#initial.unwrap_or(vec![]),
                        r#item: r#item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.\n\nTo support structured, hierarchical registration of data gathered using digital forms and other questionnaires.  Questionnaires provide greater control over presentation and allow capture of data in a domain-independent way (i.e. capturing information that would otherwise require multiple distinct types of resources)."]
#[derive(Default, Debug, Clone)]
pub struct Questionnaire {
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
    #[doc = "An absolute URI that is used to identify this questionnaire when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this questionnaire is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the questionnaire is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this questionnaire when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the questionnaire when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the questionnaire author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the questionnaire. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the questionnaire."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The URL of a Questionnaire that this Questionnaire is based on."]
    pub r#derived_from: Vec<super::super::types::Canonical>,
    #[doc = "The status of this questionnaire. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this questionnaire is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The types of subjects that can be the subject of responses created for the questionnaire."]
    pub r#subject_type: Vec<super::super::types::Code>,
    #[doc = "The date  (and optionally time) when the questionnaire was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the questionnaire changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the questionnaire."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the questionnaire from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate questionnaire instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the questionnaire is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this questionnaire is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the questionnaire and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the questionnaire."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the questionnaire content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "An identifier for this question or group of questions in a particular terminology such as LOINC."]
    pub r#code: Vec<Box<super::super::types::Coding>>,
    #[doc = "A particular question, question grouping or display text that is part of the questionnaire."]
    pub r#item: Vec<QuestionnaireItem>,
}
impl serde::ser::Serialize for Questionnaire {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Questionnaire")?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("version", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
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
        if !self.r#derived_from.is_empty() {
            let values = self
                .r#derived_from
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("derivedFrom", &values)?;
            }
            let requires_elements = self
                .r#derived_from
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#derived_from
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_derivedFrom", &primitive_elements)?;
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
        if let Some(some) = self.r#experimental.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("experimental", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_experimental", &primitive_element)?;
            }
        }
        if !self.r#subject_type.is_empty() {
            let values = self
                .r#subject_type
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("subjectType", &values)?;
            }
            let requires_elements = self
                .r#subject_type
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#subject_type
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_subjectType", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("date", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
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
        if !self.r#use_context.is_empty() {
            state.serialize_entry("useContext", &self.r#use_context)?;
        }
        if !self.r#jurisdiction.is_empty() {
            state.serialize_entry("jurisdiction", &self.r#jurisdiction)?;
        }
        if let Some(some) = self.r#purpose.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("purpose", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_purpose", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#copyright.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("copyright", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_copyright", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#approval_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("approvalDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_approvalDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_review_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("lastReviewDate", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastReviewDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#effective_period.as_ref() {
            state.serialize_entry("effectivePeriod", some)?;
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Questionnaire {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "version")]
            Version,
            #[serde(rename = "_version")]
            VersionPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "derivedFrom")]
            DerivedFrom,
            #[serde(rename = "_derivedFrom")]
            DerivedFromPrimitiveElement,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "subjectType")]
            SubjectType,
            #[serde(rename = "_subjectType")]
            SubjectTypePrimitiveElement,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "publisher")]
            Publisher,
            #[serde(rename = "_publisher")]
            PublisherPrimitiveElement,
            #[serde(rename = "contact")]
            Contact,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "useContext")]
            UseContext,
            #[serde(rename = "jurisdiction")]
            Jurisdiction,
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "copyright")]
            Copyright,
            #[serde(rename = "_copyright")]
            CopyrightPrimitiveElement,
            #[serde(rename = "approvalDate")]
            ApprovalDate,
            #[serde(rename = "_approvalDate")]
            ApprovalDatePrimitiveElement,
            #[serde(rename = "lastReviewDate")]
            LastReviewDate,
            #[serde(rename = "_lastReviewDate")]
            LastReviewDatePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "item")]
            Item,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Questionnaire;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Questionnaire")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Questionnaire, V::Error>
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
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#derived_from: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#subject_type: Option<Vec<super::super::types::Code>> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#approval_date: Option<super::super::types::Date> = None;
                let mut r#last_review_date: Option<super::super::types::Date> = None;
                let mut r#effective_period: Option<Box<super::super::types::Period>> = None;
                let mut r#code: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#item: Option<Vec<QuestionnaireItem>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Questionnaire" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Questionnaire",
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
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Version => {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::VersionPrimitiveElement => {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
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
                            Field::DerivedFrom => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#derived_from.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("derivedFrom"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::DerivedFromPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#derived_from.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_derivedFrom"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
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
                            Field::Experimental => {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("experimental"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExperimentalPrimitiveElement => {
                                let some = r#experimental.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_experimental"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::SubjectType => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#subject_type.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("subjectType"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::SubjectTypePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#subject_type.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_subjectType"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Date => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("date"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DatePrimitiveElement => {
                                let some = r#date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_date"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
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
                            Field::UseContext => {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some(map_access.next_value()?);
                            }
                            Field::Jurisdiction => {
                                if r#jurisdiction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("jurisdiction"));
                                }
                                r#jurisdiction = Some(map_access.next_value()?);
                            }
                            Field::Purpose => {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PurposePrimitiveElement => {
                                let some = r#purpose.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Copyright => {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CopyrightPrimitiveElement => {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ApprovalDate => {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("approvalDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ApprovalDatePrimitiveElement => {
                                let some = r#approval_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_approvalDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::LastReviewDate => {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastReviewDate",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LastReviewDatePrimitiveElement => {
                                let some = r#last_review_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastReviewDate",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::EffectivePeriod => {
                                if r#effective_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectivePeriod",
                                    ));
                                }
                                r#effective_period = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "version",
                                        "name",
                                        "title",
                                        "derivedFrom",
                                        "status",
                                        "experimental",
                                        "subjectType",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "useContext",
                                        "jurisdiction",
                                        "purpose",
                                        "copyright",
                                        "approvalDate",
                                        "lastReviewDate",
                                        "effectivePeriod",
                                        "code",
                                        "item",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Questionnaire {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#url,
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#version,
                        r#name,
                        r#title,
                        r#derived_from: r#derived_from.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#experimental,
                        r#subject_type: r#subject_type.unwrap_or(vec![]),
                        r#date,
                        r#publisher,
                        r#contact: r#contact.unwrap_or(vec![]),
                        r#description,
                        r#use_context: r#use_context.unwrap_or(vec![]),
                        r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                        r#purpose,
                        r#copyright,
                        r#approval_date,
                        r#last_review_date,
                        r#effective_period,
                        r#code: r#code.unwrap_or(vec![]),
                        r#item: r#item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
