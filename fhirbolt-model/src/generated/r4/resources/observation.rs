// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The time or time-period the observed value is asserted as being true. For biological subjects - e.g. human patients - this is usually called the \"physiologically relevant time\". This is usually either the time of the procedure or of specimen collection, but very often the source of the date/time is not known, only the date/time itself."]
#[derive(Debug, Clone)]
pub enum ObservationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Instant(Box<super::super::types::Instant>),
    Invalid,
}
impl Default for ObservationEffective {
    fn default() -> ObservationEffective {
        ObservationEffective::Invalid
    }
}
#[doc = "The information determined as a result of making the observation, if the information has a simple value."]
#[derive(Debug, Clone)]
pub enum ObservationValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ObservationValue {
    fn default() -> ObservationValue {
        ObservationValue::Invalid
    }
}
#[doc = "The information determined as a result of making the observation, if the information has a simple value."]
#[derive(Debug, Clone)]
pub enum ObservationComponentValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ObservationComponentValue {
    fn default() -> ObservationComponentValue {
        ObservationComponentValue::Invalid
    }
}
#[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range.  Multiple reference ranges are interpreted as an \"OR\".   In other words, to represent two distinct target populations, two `referenceRange` elements would be used."]
#[derive(Default, Debug, Clone)]
pub struct ObservationReferenceRange {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The value of the low bound of the reference range.  The low bound of the reference range endpoint is inclusive of the value (e.g.  reference range is >=5 - <=9). If the low bound is omitted,  it is assumed to be meaningless (e.g. reference range is <=2.3)."]
    pub r#low: Option<Box<super::super::types::Quantity>>,
    #[doc = "The value of the high bound of the reference range.  The high bound of the reference range endpoint is inclusive of the value (e.g.  reference range is >=5 - <=9). If the high bound is omitted,  it is assumed to be meaningless (e.g. reference range is >= 2.3)."]
    pub r#high: Option<Box<super::super::types::Quantity>>,
    #[doc = "Codes to indicate the what part of the targeted reference population it applies to. For example, the normal or therapeutic range."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Codes to indicate the target population this reference range applies to.  For example, a reference range may be based on the normal population or a particular sex or race.  Multiple `appliesTo`  are interpreted as an \"AND\" of the target populations.  For example, to represent a target population of African American females, both a code of female and a code for African American would be used."]
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The age at which this reference range is applicable. This is a neonatal age (e.g. number of weeks at term) if the meaning says so."]
    pub r#age: Option<Box<super::super::types::Range>>,
    #[doc = "Text based reference range in an observation which may be used when a quantitative range is not appropriate for an observation.  An example would be a reference value of \"Negative\" or a list or table of \"normals\"."]
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ObservationReferenceRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#low.as_ref() {
                state.serialize_entry("low", some)?;
            }
            if let Some(some) = self.r#high.as_ref() {
                state.serialize_entry("high", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if !self.r#applies_to.is_empty() {
                state.serialize_entry("appliesTo", &self.r#applies_to)?;
            }
            if let Some(some) = self.r#age.as_ref() {
                state.serialize_entry("age", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("text", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_text", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text.as_ref() {
                    state.serialize_entry("text", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationReferenceRange {
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
            #[serde(rename = "low")]
            Low,
            #[serde(rename = "high")]
            High,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "appliesTo")]
            AppliesTo,
            #[serde(rename = "age")]
            Age,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObservationReferenceRange;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationReferenceRange")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationReferenceRange, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#low: Option<Box<super::super::types::Quantity>> = None;
                let mut r#high: Option<Box<super::super::types::Quantity>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#applies_to: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#age: Option<Box<super::super::types::Range>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Low => {
                                if r#low.is_some() {
                                    return Err(serde::de::Error::duplicate_field("low"));
                                }
                                r#low = Some(map_access.next_value()?);
                            }
                            Field::High => {
                                if r#high.is_some() {
                                    return Err(serde::de::Error::duplicate_field("high"));
                                }
                                r#high = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::AppliesTo => {
                                if r#applies_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("appliesTo"));
                                }
                                r#applies_to = Some(map_access.next_value()?);
                            }
                            Field::Age => {
                                if r#age.is_some() {
                                    return Err(serde::de::Error::duplicate_field("age"));
                                }
                                r#age = Some(map_access.next_value()?);
                            }
                            Field::Text => {
                                if _ctx.from_json {
                                    let some = r#text.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text.is_some() {
                                        return Err(serde::de::Error::duplicate_field("text"));
                                    }
                                    r#text = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "text",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "low",
                                            "high",
                                            "type",
                                            "appliesTo",
                                            "age",
                                            "text",
                                        ],
                                    ));
                                }
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
                                        "low",
                                        "high",
                                        "type",
                                        "appliesTo",
                                        "age",
                                        "text",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ObservationReferenceRange {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#low,
                        r#high,
                        r#type,
                        r#applies_to: r#applies_to.unwrap_or(vec![]),
                        r#age,
                        r#text,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Some observations have multiple component observations.  These component observations are expressed as separate code value pairs that share the same attributes.  Examples include systolic and diastolic component observations for blood pressure measurement and multiple component observations for genetics observations."]
#[derive(Default, Debug, Clone)]
pub struct ObservationComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Describes what was observed. Sometimes this is called the observation \"code\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The information determined as a result of making the observation, if the information has a simple value."]
    pub r#value: Option<ObservationComponentValue>,
    #[doc = "Provides a reason why the expected value in the element Observation.component.value\\[x\\] is missing."]
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A categorical assessment of an observation value.  For example, high, low, normal."]
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range."]
    pub r#reference_range: Vec<ObservationReferenceRange>,
}
impl serde::ser::Serialize for ObservationComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    ObservationComponentValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    ObservationComponentValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    ObservationComponentValue::String(ref value) => {
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
                    ObservationComponentValue::Boolean(ref value) => {
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
                    ObservationComponentValue::Integer(ref value) => {
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
                    ObservationComponentValue::Range(ref value) => {
                        state.serialize_entry("valueRange", value)?;
                    }
                    ObservationComponentValue::Ratio(ref value) => {
                        state.serialize_entry("valueRatio", value)?;
                    }
                    ObservationComponentValue::SampledData(ref value) => {
                        state.serialize_entry("valueSampledData", value)?;
                    }
                    ObservationComponentValue::Time(ref value) => {
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
                    ObservationComponentValue::DateTime(ref value) => {
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
                    ObservationComponentValue::Period(ref value) => {
                        state.serialize_entry("valuePeriod", value)?;
                    }
                    ObservationComponentValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#data_absent_reason.as_ref() {
                state.serialize_entry("dataAbsentReason", some)?;
            }
            if !self.r#interpretation.is_empty() {
                state.serialize_entry("interpretation", &self.r#interpretation)?;
            }
            if !self.r#reference_range.is_empty() {
                state.serialize_entry("referenceRange", &self.r#reference_range)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for ObservationComponent {
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
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueRatio")]
            ValueRatio,
            #[serde(rename = "valueSampledData")]
            ValueSampledData,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valuePeriod")]
            ValuePeriod,
            #[serde(rename = "dataAbsentReason")]
            DataAbsentReason,
            #[serde(rename = "interpretation")]
            Interpretation,
            #[serde(rename = "referenceRange")]
            ReferenceRange,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ObservationComponent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ObservationComponent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ObservationComponent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<ObservationComponentValue> = None;
                let mut r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#interpretation: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reference_range: Option<Vec<ObservationReferenceRange>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value = Some(ObservationComponentValue::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(ObservationComponentValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::String(Default::default()),
                                    );
                                    if let ObservationComponentValue::String(variant) = r#enum {
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
                                    r#value = Some(ObservationComponentValue::String(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::String(Default::default()),
                                    );
                                    if let ObservationComponentValue::String(variant) = r#enum {
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
                                            "code",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Boolean(Default::default()),
                                    );
                                    if let ObservationComponentValue::Boolean(variant) = r#enum {
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
                                    r#value = Some(ObservationComponentValue::Boolean(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Boolean(Default::default()),
                                    );
                                    if let ObservationComponentValue::Boolean(variant) = r#enum {
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
                                            "code",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Integer(Default::default()),
                                    );
                                    if let ObservationComponentValue::Integer(variant) = r#enum {
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
                                    r#value = Some(ObservationComponentValue::Integer(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Integer(Default::default()),
                                    );
                                    if let ObservationComponentValue::Integer(variant) = r#enum {
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
                                            "code",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(ObservationComponentValue::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value = Some(ObservationComponentValue::Ratio(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value = Some(ObservationComponentValue::SampledData(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Time(Default::default()),
                                    );
                                    if let ObservationComponentValue::Time(variant) = r#enum {
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
                                    r#value = Some(ObservationComponentValue::Time(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::Time(Default::default()),
                                    );
                                    if let ObservationComponentValue::Time(variant) = r#enum {
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
                                            "code",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::DateTime(Default::default()),
                                    );
                                    if let ObservationComponentValue::DateTime(variant) = r#enum {
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
                                    r#value = Some(ObservationComponentValue::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        ObservationComponentValue::DateTime(Default::default()),
                                    );
                                    if let ObservationComponentValue::DateTime(variant) = r#enum {
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
                                            "code",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "referenceRange",
                                        ],
                                    ));
                                }
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(ObservationComponentValue::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DataAbsentReason => {
                                if r#data_absent_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dataAbsentReason",
                                    ));
                                }
                                r#data_absent_reason = Some(map_access.next_value()?);
                            }
                            Field::Interpretation => {
                                if r#interpretation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "interpretation",
                                    ));
                                }
                                r#interpretation = Some(map_access.next_value()?);
                            }
                            Field::ReferenceRange => {
                                if r#reference_range.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceRange",
                                    ));
                                }
                                r#reference_range = Some(map_access.next_value()?);
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
                                        "code",
                                        "valueQuantity",
                                        "valueCodeableConcept",
                                        "valueString",
                                        "valueBoolean",
                                        "valueInteger",
                                        "valueRange",
                                        "valueRatio",
                                        "valueSampledData",
                                        "valueTime",
                                        "valueDateTime",
                                        "valuePeriod",
                                        "dataAbsentReason",
                                        "interpretation",
                                        "referenceRange",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(ObservationComponent {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#value,
                        r#data_absent_reason,
                        r#interpretation: r#interpretation.unwrap_or(vec![]),
                        r#reference_range: r#reference_range.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Measurements and simple assertions made about a patient, device or other subject.\n\nObservations are a key aspect of healthcare.  This resource is used to capture those that do not require more sophisticated mechanisms."]
#[derive(Default, Debug, Clone)]
pub struct Observation {
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
    #[doc = "A unique identifier assigned to this observation."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event.  For example, a MedicationRequest may require a patient to have laboratory test performed before  it is dispensed."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular Observation is a component or step.  For example,  an observation as part of a procedure."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "The status of the result value."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies the general type of observation being made."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes what was observed. Sometimes this is called the observation \"name\"."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The patient, or group of patients, location, or device this observation is about and into whose record the observation is placed. If the actual focus of the observation is different from the subject (or a sample of, part, or region of the subject), the `focus` element or the `code` itself specifies the actual focus of the observation."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The actual focus of an observation when it is not the patient of record representing something or someone associated with the patient such as a spouse, parent, fetus, or donor. For example, fetus observations in a mother's record.  The focus of an observation could also be an existing condition,  an intervention, the subject's diet,  another observation of the subject,  or a body structure such as tumor or implanted device.   An example use case would be using the Observation resource to capture whether the mother is trained to change her child's tracheostomy tube. In this example, the child is the patient of record and the mother is the focus."]
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    #[doc = "The healthcare event  (e.g. a patient and healthcare provider interaction) during which this observation is made."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The time or time-period the observed value is asserted as being true. For biological subjects - e.g. human patients - this is usually called the \"physiologically relevant time\". This is usually either the time of the procedure or of specimen collection, but very often the source of the date/time is not known, only the date/time itself."]
    pub r#effective: Option<ObservationEffective>,
    #[doc = "The date and time this version of the observation was made available to providers, typically after the results have been reviewed and verified."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "Who was responsible for asserting the observed value as \"true\"."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The information determined as a result of making the observation, if the information has a simple value."]
    pub r#value: Option<ObservationValue>,
    #[doc = "Provides a reason why the expected value in the element Observation.value\\[x\\] is missing."]
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A categorical assessment of an observation value.  For example, high, low, normal."]
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Comments about the observation or the results."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Indicates the site on the subject's body where the observation was made (i.e. the target site)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the mechanism used to perform the observation."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specimen that was used when this observation was made."]
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    #[doc = "The device used to generate the observation data."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Guidance on how to interpret the value by comparison to a normal or recommended range.  Multiple reference ranges are interpreted as an \"OR\".   In other words, to represent two distinct target populations, two `referenceRange` elements would be used."]
    pub r#reference_range: Vec<ObservationReferenceRange>,
    #[doc = "This observation is a group observation (e.g. a battery, a panel of tests, a set of vital sign measurements) that includes the target as a member of the group."]
    pub r#has_member: Vec<Box<super::super::types::Reference>>,
    #[doc = "The target resource that represents a measurement from which this observation value is derived. For example, a calculated anion gap or a fetal measurement based on an ultrasound image."]
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    #[doc = "Some observations have multiple component observations.  These component observations are expressed as separate code value pairs that share the same attributes.  Examples include systolic and diastolic component observations for blood pressure measurement and multiple component observations for genetics observations."]
    pub r#component: Vec<ObservationComponent>,
}
impl crate::AnyResource for Observation {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Observation")?;
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
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            state.serialize_entry("code", &self.r#code)?;
            if let Some(some) = self.r#subject.as_ref() {
                state.serialize_entry("subject", some)?;
            }
            if !self.r#focus.is_empty() {
                state.serialize_entry("focus", &self.r#focus)?;
            }
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#effective.as_ref() {
                match some {
                    ObservationEffective::DateTime(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("effectiveDateTime", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_effectiveDateTime", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("effectiveDateTime", value)?;
                        }
                    }
                    ObservationEffective::Period(ref value) => {
                        state.serialize_entry("effectivePeriod", value)?;
                    }
                    ObservationEffective::Timing(ref value) => {
                        state.serialize_entry("effectiveTiming", value)?;
                    }
                    ObservationEffective::Instant(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("effectiveInstant", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_effectiveInstant", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("effectiveInstant", value)?;
                        }
                    }
                    ObservationEffective::Invalid => {
                        return Err(serde::ser::Error::custom("effective is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#issued.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("issued", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_issued", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#issued.as_ref() {
                    state.serialize_entry("issued", some)?;
                }
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    ObservationValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    ObservationValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    ObservationValue::String(ref value) => {
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
                    ObservationValue::Boolean(ref value) => {
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
                    ObservationValue::Integer(ref value) => {
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
                    ObservationValue::Range(ref value) => {
                        state.serialize_entry("valueRange", value)?;
                    }
                    ObservationValue::Ratio(ref value) => {
                        state.serialize_entry("valueRatio", value)?;
                    }
                    ObservationValue::SampledData(ref value) => {
                        state.serialize_entry("valueSampledData", value)?;
                    }
                    ObservationValue::Time(ref value) => {
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
                    ObservationValue::DateTime(ref value) => {
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
                    ObservationValue::Period(ref value) => {
                        state.serialize_entry("valuePeriod", value)?;
                    }
                    ObservationValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#data_absent_reason.as_ref() {
                state.serialize_entry("dataAbsentReason", some)?;
            }
            if !self.r#interpretation.is_empty() {
                state.serialize_entry("interpretation", &self.r#interpretation)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if let Some(some) = self.r#body_site.as_ref() {
                state.serialize_entry("bodySite", some)?;
            }
            if let Some(some) = self.r#method.as_ref() {
                state.serialize_entry("method", some)?;
            }
            if let Some(some) = self.r#specimen.as_ref() {
                state.serialize_entry("specimen", some)?;
            }
            if let Some(some) = self.r#device.as_ref() {
                state.serialize_entry("device", some)?;
            }
            if !self.r#reference_range.is_empty() {
                state.serialize_entry("referenceRange", &self.r#reference_range)?;
            }
            if !self.r#has_member.is_empty() {
                state.serialize_entry("hasMember", &self.r#has_member)?;
            }
            if !self.r#derived_from.is_empty() {
                state.serialize_entry("derivedFrom", &self.r#derived_from)?;
            }
            if !self.r#component.is_empty() {
                state.serialize_entry("component", &self.r#component)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Observation {
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
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "focus")]
            Focus,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "effectiveDateTime")]
            EffectiveDateTime,
            #[serde(rename = "_effectiveDateTime")]
            EffectiveDateTimePrimitiveElement,
            #[serde(rename = "effectivePeriod")]
            EffectivePeriod,
            #[serde(rename = "effectiveTiming")]
            EffectiveTiming,
            #[serde(rename = "effectiveInstant")]
            EffectiveInstant,
            #[serde(rename = "_effectiveInstant")]
            EffectiveInstantPrimitiveElement,
            #[serde(rename = "issued")]
            Issued,
            #[serde(rename = "_issued")]
            IssuedPrimitiveElement,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueInteger")]
            ValueInteger,
            #[serde(rename = "_valueInteger")]
            ValueIntegerPrimitiveElement,
            #[serde(rename = "valueRange")]
            ValueRange,
            #[serde(rename = "valueRatio")]
            ValueRatio,
            #[serde(rename = "valueSampledData")]
            ValueSampledData,
            #[serde(rename = "valueTime")]
            ValueTime,
            #[serde(rename = "_valueTime")]
            ValueTimePrimitiveElement,
            #[serde(rename = "valueDateTime")]
            ValueDateTime,
            #[serde(rename = "_valueDateTime")]
            ValueDateTimePrimitiveElement,
            #[serde(rename = "valuePeriod")]
            ValuePeriod,
            #[serde(rename = "dataAbsentReason")]
            DataAbsentReason,
            #[serde(rename = "interpretation")]
            Interpretation,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "specimen")]
            Specimen,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "referenceRange")]
            ReferenceRange,
            #[serde(rename = "hasMember")]
            HasMember,
            #[serde(rename = "derivedFrom")]
            DerivedFrom,
            #[serde(rename = "component")]
            Component,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Observation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Observation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Observation, V::Error>
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
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#focus: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#effective: Option<ObservationEffective> = None;
                let mut r#issued: Option<super::super::types::Instant> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#value: Option<ObservationValue> = None;
                let mut r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#interpretation: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#specimen: Option<Box<super::super::types::Reference>> = None;
                let mut r#device: Option<Box<super::super::types::Reference>> = None;
                let mut r#reference_range: Option<Vec<ObservationReferenceRange>> = None;
                let mut r#has_member: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#derived_from: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#component: Option<Vec<ObservationComponent>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Observation" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Observation",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
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
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Focus => {
                                if r#focus.is_some() {
                                    return Err(serde::de::Error::duplicate_field("focus"));
                                }
                                r#focus = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::EffectiveDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ObservationEffective::DateTime(Default::default()),
                                    );
                                    if let ObservationEffective::DateTime(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "effectiveDateTime",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effective[x]",
                                        ));
                                    }
                                } else {
                                    if r#effective.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveDateTime",
                                        ));
                                    }
                                    r#effective = Some(ObservationEffective::DateTime(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::EffectiveDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ObservationEffective::DateTime(Default::default()),
                                    );
                                    if let ObservationEffective::DateTime(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_effectiveDateTime",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_effective[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "effectiveDateTime",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::EffectivePeriod => {
                                if r#effective.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectivePeriod",
                                    ));
                                }
                                r#effective =
                                    Some(ObservationEffective::Period(map_access.next_value()?));
                            }
                            Field::EffectiveTiming => {
                                if r#effective.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "effectiveTiming",
                                    ));
                                }
                                r#effective =
                                    Some(ObservationEffective::Timing(map_access.next_value()?));
                            }
                            Field::EffectiveInstant => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ObservationEffective::Instant(Default::default()),
                                    );
                                    if let ObservationEffective::Instant(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "effectiveInstant",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effective[x]",
                                        ));
                                    }
                                } else {
                                    if r#effective.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "effectiveInstant",
                                        ));
                                    }
                                    r#effective = Some(ObservationEffective::Instant(
                                        map_access.next_value()?,
                                    ));
                                }
                            }
                            Field::EffectiveInstantPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#effective.get_or_insert(
                                        ObservationEffective::Instant(Default::default()),
                                    );
                                    if let ObservationEffective::Instant(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_effectiveInstant",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_effective[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "effectiveInstant",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::Issued => {
                                if _ctx.from_json {
                                    let some = r#issued.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#issued.is_some() {
                                        return Err(serde::de::Error::duplicate_field("issued"));
                                    }
                                    r#issued = Some(map_access.next_value()?);
                                }
                            }
                            Field::IssuedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#issued.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_issued"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "issued",
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
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value =
                                    Some(ObservationValue::Quantity(map_access.next_value()?));
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(ObservationValue::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ValueString => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::String(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::String(variant) = r#enum {
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
                                    r#value =
                                        Some(ObservationValue::String(map_access.next_value()?));
                                }
                            }
                            Field::ValueStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::String(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::String(variant) = r#enum {
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::Boolean(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::Boolean(variant) = r#enum {
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
                                    r#value =
                                        Some(ObservationValue::Boolean(map_access.next_value()?));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::Boolean(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::Boolean(variant) = r#enum {
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::Integer(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::Integer(variant) = r#enum {
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
                                    r#value =
                                        Some(ObservationValue::Integer(map_access.next_value()?));
                                }
                            }
                            Field::ValueIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::Integer(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::Integer(variant) = r#enum {
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueRange => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRange"));
                                }
                                r#value = Some(ObservationValue::Range(map_access.next_value()?));
                            }
                            Field::ValueRatio => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueRatio"));
                                }
                                r#value = Some(ObservationValue::Ratio(map_access.next_value()?));
                            }
                            Field::ValueSampledData => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueSampledData",
                                    ));
                                }
                                r#value =
                                    Some(ObservationValue::SampledData(map_access.next_value()?));
                            }
                            Field::ValueTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value
                                        .get_or_insert(ObservationValue::Time(Default::default()));
                                    if let ObservationValue::Time(variant) = r#enum {
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
                                    r#value =
                                        Some(ObservationValue::Time(map_access.next_value()?));
                                }
                            }
                            Field::ValueTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value
                                        .get_or_insert(ObservationValue::Time(Default::default()));
                                    if let ObservationValue::Time(variant) = r#enum {
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueDateTime => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::DateTime(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::DateTime(variant) = r#enum {
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
                                    r#value =
                                        Some(ObservationValue::DateTime(map_access.next_value()?));
                                }
                            }
                            Field::ValueDateTimePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(ObservationValue::DateTime(
                                        Default::default(),
                                    ));
                                    if let ObservationValue::DateTime(variant) = r#enum {
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
                                            "meta",
                                            "implicitRules",
                                            "language",
                                            "text",
                                            "contained",
                                            "extension",
                                            "modifierExtension",
                                            "identifier",
                                            "basedOn",
                                            "partOf",
                                            "status",
                                            "category",
                                            "code",
                                            "subject",
                                            "focus",
                                            "encounter",
                                            "effectiveDateTime",
                                            "effectivePeriod",
                                            "effectiveTiming",
                                            "effectiveInstant",
                                            "issued",
                                            "performer",
                                            "valueQuantity",
                                            "valueCodeableConcept",
                                            "valueString",
                                            "valueBoolean",
                                            "valueInteger",
                                            "valueRange",
                                            "valueRatio",
                                            "valueSampledData",
                                            "valueTime",
                                            "valueDateTime",
                                            "valuePeriod",
                                            "dataAbsentReason",
                                            "interpretation",
                                            "note",
                                            "bodySite",
                                            "method",
                                            "specimen",
                                            "device",
                                            "referenceRange",
                                            "hasMember",
                                            "derivedFrom",
                                            "component",
                                        ],
                                    ));
                                }
                            }
                            Field::ValuePeriod => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuePeriod"));
                                }
                                r#value = Some(ObservationValue::Period(map_access.next_value()?));
                            }
                            Field::DataAbsentReason => {
                                if r#data_absent_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "dataAbsentReason",
                                    ));
                                }
                                r#data_absent_reason = Some(map_access.next_value()?);
                            }
                            Field::Interpretation => {
                                if r#interpretation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "interpretation",
                                    ));
                                }
                                r#interpretation = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::BodySite => {
                                if r#body_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("bodySite"));
                                }
                                r#body_site = Some(map_access.next_value()?);
                            }
                            Field::Method => {
                                if r#method.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                r#method = Some(map_access.next_value()?);
                            }
                            Field::Specimen => {
                                if r#specimen.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specimen"));
                                }
                                r#specimen = Some(map_access.next_value()?);
                            }
                            Field::Device => {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                r#device = Some(map_access.next_value()?);
                            }
                            Field::ReferenceRange => {
                                if r#reference_range.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceRange",
                                    ));
                                }
                                r#reference_range = Some(map_access.next_value()?);
                            }
                            Field::HasMember => {
                                if r#has_member.is_some() {
                                    return Err(serde::de::Error::duplicate_field("hasMember"));
                                }
                                r#has_member = Some(map_access.next_value()?);
                            }
                            Field::DerivedFrom => {
                                if r#derived_from.is_some() {
                                    return Err(serde::de::Error::duplicate_field("derivedFrom"));
                                }
                                r#derived_from = Some(map_access.next_value()?);
                            }
                            Field::Component => {
                                if r#component.is_some() {
                                    return Err(serde::de::Error::duplicate_field("component"));
                                }
                                r#component = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
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
                                        "identifier",
                                        "basedOn",
                                        "partOf",
                                        "status",
                                        "category",
                                        "code",
                                        "subject",
                                        "focus",
                                        "encounter",
                                        "effectiveDateTime",
                                        "effectivePeriod",
                                        "effectiveTiming",
                                        "effectiveInstant",
                                        "issued",
                                        "performer",
                                        "valueQuantity",
                                        "valueCodeableConcept",
                                        "valueString",
                                        "valueBoolean",
                                        "valueInteger",
                                        "valueRange",
                                        "valueRatio",
                                        "valueSampledData",
                                        "valueTime",
                                        "valueDateTime",
                                        "valuePeriod",
                                        "dataAbsentReason",
                                        "interpretation",
                                        "note",
                                        "bodySite",
                                        "method",
                                        "specimen",
                                        "device",
                                        "referenceRange",
                                        "hasMember",
                                        "derivedFrom",
                                        "component",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Observation {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#subject,
                        r#focus: r#focus.unwrap_or(vec![]),
                        r#encounter,
                        r#effective,
                        r#issued,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#value,
                        r#data_absent_reason,
                        r#interpretation: r#interpretation.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#body_site,
                        r#method,
                        r#specimen,
                        r#device,
                        r#reference_range: r#reference_range.unwrap_or(vec![]),
                        r#has_member: r#has_member.unwrap_or(vec![]),
                        r#derived_from: r#derived_from.unwrap_or(vec![]),
                        r#component: r#component.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
