// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The date or event after which the goal should begin being pursued."]
#[derive(Debug, Clone)]
pub enum GoalStart {
    Date(Box<super::super::types::Date>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for GoalStart {
    fn default() -> GoalStart {
        GoalStart::Invalid
    }
}
#[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
#[derive(Debug, Clone)]
pub enum GoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Ratio(Box<super::super::types::Ratio>),
    Invalid,
}
impl Default for GoalTargetDetail {
    fn default() -> GoalTargetDetail {
        GoalTargetDetail::Invalid
    }
}
#[doc = "Indicates either the date or the duration after start by which the goal should be met."]
#[derive(Debug, Clone)]
pub enum GoalTargetDue {
    Date(Box<super::super::types::Date>),
    Duration(Box<super::super::types::Duration>),
    Invalid,
}
impl Default for GoalTargetDue {
    fn default() -> GoalTargetDue {
        GoalTargetDue::Invalid
    }
}
#[doc = "Indicates what should be done by when."]
#[derive(Default, Debug, Clone)]
pub struct GoalTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The parameter whose value is being tracked, e.g. body weight, blood pressure, or hemoglobin A1c level."]
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target value of the focus to be achieved to signify the fulfillment of the goal, e.g. 150 pounds, 7.0%. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any focus value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any focus value at or above the low value."]
    pub r#detail: Option<GoalTargetDetail>,
    #[doc = "Indicates either the date or the duration after start by which the goal should be met."]
    pub r#due: Option<GoalTargetDue>,
}
impl serde::ser::Serialize for GoalTarget {
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
            if let Some(some) = self.r#measure.as_ref() {
                state.serialize_entry("measure", some)?;
            }
            if let Some(some) = self.r#detail.as_ref() {
                match some {
                    GoalTargetDetail::Quantity(ref value) => {
                        state.serialize_entry("detailQuantity", value)?;
                    }
                    GoalTargetDetail::Range(ref value) => {
                        state.serialize_entry("detailRange", value)?;
                    }
                    GoalTargetDetail::CodeableConcept(ref value) => {
                        state.serialize_entry("detailCodeableConcept", value)?;
                    }
                    GoalTargetDetail::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailString", value)?;
                        }
                    }
                    GoalTargetDetail::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailBoolean", value)?;
                        }
                    }
                    GoalTargetDetail::Integer(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("detailInteger", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_detailInteger", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("detailInteger", value)?;
                        }
                    }
                    GoalTargetDetail::Ratio(ref value) => {
                        state.serialize_entry("detailRatio", value)?;
                    }
                    GoalTargetDetail::Invalid => {
                        return Err(serde::ser::Error::custom("detail is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#due.as_ref() {
                match some {
                    GoalTargetDue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("dueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_dueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("dueDate", value)?;
                        }
                    }
                    GoalTargetDue::Duration(ref value) => {
                        state.serialize_entry("dueDuration", value)?;
                    }
                    GoalTargetDue::Invalid => {
                        return Err(serde::ser::Error::custom("due is invalid"))
                    }
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for GoalTarget {
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
            #[serde(rename = "measure")]
            Measure,
            #[serde(rename = "detailQuantity")]
            DetailQuantity,
            #[serde(rename = "detailRange")]
            DetailRange,
            #[serde(rename = "detailCodeableConcept")]
            DetailCodeableConcept,
            #[serde(rename = "detailString")]
            DetailString,
            #[serde(rename = "_detailString")]
            DetailStringPrimitiveElement,
            #[serde(rename = "detailBoolean")]
            DetailBoolean,
            #[serde(rename = "_detailBoolean")]
            DetailBooleanPrimitiveElement,
            #[serde(rename = "detailInteger")]
            DetailInteger,
            #[serde(rename = "_detailInteger")]
            DetailIntegerPrimitiveElement,
            #[serde(rename = "detailRatio")]
            DetailRatio,
            #[serde(rename = "dueDate")]
            DueDate,
            #[serde(rename = "_dueDate")]
            DueDatePrimitiveElement,
            #[serde(rename = "dueDuration")]
            DueDuration,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = GoalTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("GoalTarget")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<GoalTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#measure: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#detail: Option<GoalTargetDetail> = None;
                let mut r#due: Option<GoalTargetDue> = None;
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
                            Field::Measure => {
                                if r#measure.is_some() {
                                    return Err(serde::de::Error::duplicate_field("measure"));
                                }
                                r#measure = Some(map_access.next_value()?);
                            }
                            Field::DetailQuantity => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailQuantity",
                                    ));
                                }
                                r#detail =
                                    Some(GoalTargetDetail::Quantity(map_access.next_value()?));
                            }
                            Field::DetailRange => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailRange"));
                                }
                                r#detail = Some(GoalTargetDetail::Range(map_access.next_value()?));
                            }
                            Field::DetailCodeableConcept => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailCodeableConcept",
                                    ));
                                }
                                r#detail = Some(GoalTargetDetail::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DetailString => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::String(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::String(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "detailString",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("detail[x]"));
                                    }
                                } else {
                                    if r#detail.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailString",
                                        ));
                                    }
                                    r#detail =
                                        Some(GoalTargetDetail::String(map_access.next_value()?));
                                }
                            }
                            Field::DetailStringPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::String(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::String(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_detailString",
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
                                            "_detail[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "detailString",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "measure",
                                            "detailQuantity",
                                            "detailRange",
                                            "detailCodeableConcept",
                                            "detailString",
                                            "detailBoolean",
                                            "detailInteger",
                                            "detailRatio",
                                            "dueDate",
                                            "dueDuration",
                                        ],
                                    ));
                                }
                            }
                            Field::DetailBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::Boolean(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::Boolean(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "detailBoolean",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("detail[x]"));
                                    }
                                } else {
                                    if r#detail.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailBoolean",
                                        ));
                                    }
                                    r#detail =
                                        Some(GoalTargetDetail::Boolean(map_access.next_value()?));
                                }
                            }
                            Field::DetailBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::Boolean(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::Boolean(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_detailBoolean",
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
                                            "_detail[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "detailBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "measure",
                                            "detailQuantity",
                                            "detailRange",
                                            "detailCodeableConcept",
                                            "detailString",
                                            "detailBoolean",
                                            "detailInteger",
                                            "detailRatio",
                                            "dueDate",
                                            "dueDuration",
                                        ],
                                    ));
                                }
                            }
                            Field::DetailInteger => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::Integer(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::Integer(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "detailInteger",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("detail[x]"));
                                    }
                                } else {
                                    if r#detail.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "detailInteger",
                                        ));
                                    }
                                    r#detail =
                                        Some(GoalTargetDetail::Integer(map_access.next_value()?));
                                }
                            }
                            Field::DetailIntegerPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#detail.get_or_insert(GoalTargetDetail::Integer(
                                        Default::default(),
                                    ));
                                    if let GoalTargetDetail::Integer(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_detailInteger",
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
                                            "_detail[x]",
                                        ));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "detailInteger",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "measure",
                                            "detailQuantity",
                                            "detailRange",
                                            "detailCodeableConcept",
                                            "detailString",
                                            "detailBoolean",
                                            "detailInteger",
                                            "detailRatio",
                                            "dueDate",
                                            "dueDuration",
                                        ],
                                    ));
                                }
                            }
                            Field::DetailRatio => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailRatio"));
                                }
                                r#detail = Some(GoalTargetDetail::Ratio(map_access.next_value()?));
                            }
                            Field::DueDate => {
                                if _ctx.from_json {
                                    let r#enum = r#due
                                        .get_or_insert(GoalTargetDue::Date(Default::default()));
                                    if let GoalTargetDue::Date(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "dueDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("due[x]"));
                                    }
                                } else {
                                    if r#due.is_some() {
                                        return Err(serde::de::Error::duplicate_field("dueDate"));
                                    }
                                    r#due = Some(GoalTargetDue::Date(map_access.next_value()?));
                                }
                            }
                            Field::DueDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#due
                                        .get_or_insert(GoalTargetDue::Date(Default::default()));
                                    if let GoalTargetDue::Date(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_dueDate",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_due[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "dueDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "measure",
                                            "detailQuantity",
                                            "detailRange",
                                            "detailCodeableConcept",
                                            "detailString",
                                            "detailBoolean",
                                            "detailInteger",
                                            "detailRatio",
                                            "dueDate",
                                            "dueDuration",
                                        ],
                                    ));
                                }
                            }
                            Field::DueDuration => {
                                if r#due.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dueDuration"));
                                }
                                r#due = Some(GoalTargetDue::Duration(map_access.next_value()?));
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
                                        "measure",
                                        "detailQuantity",
                                        "detailRange",
                                        "detailCodeableConcept",
                                        "detailString",
                                        "detailBoolean",
                                        "detailInteger",
                                        "detailRatio",
                                        "dueDate",
                                        "dueDuration",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(GoalTarget {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#measure,
                        r#detail,
                        r#due,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
#[derive(Default, Debug, Clone)]
pub struct Goal {
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
    #[doc = "Business identifiers assigned to this goal by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The state of the goal throughout its lifecycle."]
    pub r#lifecycle_status: super::super::types::Code,
    #[doc = "Describes the progression, or lack thereof, towards the goal against the target."]
    pub r#achievement_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates a category the goal falls within."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the mutually agreed level of importance associated with reaching/sustaining the goal."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-readable and/or coded description of a specific desired objective of care, such as \"control blood pressure\" or \"negotiate an obstacle course\" or \"dance with child at wedding\"."]
    pub r#description: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the patient, group or organization for whom the goal is being established."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The date or event after which the goal should begin being pursued."]
    pub r#start: Option<GoalStart>,
    #[doc = "Indicates what should be done by when."]
    pub r#target: Vec<GoalTarget>,
    #[doc = "Identifies when the current status.  I.e. When initially created, when achieved, when cancelled, etc."]
    pub r#status_date: Option<super::super::types::Date>,
    #[doc = "Captures the reason for the current status."]
    pub r#status_reason: Option<super::super::types::String>,
    #[doc = "Indicates whose goal this is - patient goal, practitioner goal, etc."]
    pub r#expressed_by: Option<Box<super::super::types::Reference>>,
    #[doc = "The identified conditions and other health record elements that are intended to be addressed by the goal."]
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    #[doc = "Any comments related to the goal."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Identifies the change (or lack of change) at the point when the status of the goal is assessed."]
    pub r#outcome_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of what's changed (or not changed)."]
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
}
impl crate::AnyResource for Goal {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Goal")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#lifecycle_status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lifecycleStatus", &some)?;
                }
                if self.r#lifecycle_status.id.is_some()
                    || !self.r#lifecycle_status.extension.is_empty()
                {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#lifecycle_status.id.as_ref(),
                        extension: &self.r#lifecycle_status.extension,
                    };
                    state.serialize_entry("_lifecycleStatus", &primitive_element)?;
                }
            } else {
                state.serialize_entry("lifecycleStatus", &self.r#lifecycle_status)?;
            }
            if let Some(some) = self.r#achievement_status.as_ref() {
                state.serialize_entry("achievementStatus", some)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#priority.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            state.serialize_entry("description", &self.r#description)?;
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#start.as_ref() {
                match some {
                    GoalStart::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("startDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_startDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("startDate", value)?;
                        }
                    }
                    GoalStart::CodeableConcept(ref value) => {
                        state.serialize_entry("startCodeableConcept", value)?;
                    }
                    GoalStart::Invalid => {
                        return Err(serde::ser::Error::custom("start is invalid"))
                    }
                }
            }
            if !self.r#target.is_empty() {
                state.serialize_entry("target", &self.r#target)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_date.as_ref() {
                    state.serialize_entry("statusDate", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status_reason.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("statusReason", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_statusReason", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#status_reason.as_ref() {
                    state.serialize_entry("statusReason", some)?;
                }
            }
            if let Some(some) = self.r#expressed_by.as_ref() {
                state.serialize_entry("expressedBy", some)?;
            }
            if !self.r#addresses.is_empty() {
                state.serialize_entry("addresses", &self.r#addresses)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            if !self.r#outcome_code.is_empty() {
                state.serialize_entry("outcomeCode", &self.r#outcome_code)?;
            }
            if !self.r#outcome_reference.is_empty() {
                state.serialize_entry("outcomeReference", &self.r#outcome_reference)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Goal {
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
            #[serde(rename = "lifecycleStatus")]
            LifecycleStatus,
            #[serde(rename = "_lifecycleStatus")]
            LifecycleStatusPrimitiveElement,
            #[serde(rename = "achievementStatus")]
            AchievementStatus,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "startDate")]
            StartDate,
            #[serde(rename = "_startDate")]
            StartDatePrimitiveElement,
            #[serde(rename = "startCodeableConcept")]
            StartCodeableConcept,
            #[serde(rename = "target")]
            Target,
            #[serde(rename = "statusDate")]
            StatusDate,
            #[serde(rename = "_statusDate")]
            StatusDatePrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "_statusReason")]
            StatusReasonPrimitiveElement,
            #[serde(rename = "expressedBy")]
            ExpressedBy,
            #[serde(rename = "addresses")]
            Addresses,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "outcomeCode")]
            OutcomeCode,
            #[serde(rename = "outcomeReference")]
            OutcomeReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Goal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Goal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Goal, V::Error>
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
                let mut r#lifecycle_status: Option<super::super::types::Code> = None;
                let mut r#achievement_status: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#priority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#start: Option<GoalStart> = None;
                let mut r#target: Option<Vec<GoalTarget>> = None;
                let mut r#status_date: Option<super::super::types::Date> = None;
                let mut r#status_reason: Option<super::super::types::String> = None;
                let mut r#expressed_by: Option<Box<super::super::types::Reference>> = None;
                let mut r#addresses: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#outcome_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#outcome_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Goal" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Goal",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
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
                            Field::LifecycleStatus => {
                                if _ctx.from_json {
                                    let some = r#lifecycle_status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lifecycleStatus",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#lifecycle_status.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "lifecycleStatus",
                                        ));
                                    }
                                    r#lifecycle_status = Some(map_access.next_value()?);
                                }
                            }
                            Field::LifecycleStatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#lifecycle_status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_lifecycleStatus",
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
                                        "lifecycleStatus",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
                                        ],
                                    ));
                                }
                            }
                            Field::AchievementStatus => {
                                if r#achievement_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "achievementStatus",
                                    ));
                                }
                                r#achievement_status = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value()?);
                            }
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::StartDate => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#start.get_or_insert(GoalStart::Date(Default::default()));
                                    if let GoalStart::Date(variant) = r#enum {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "startDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("start[x]"));
                                    }
                                } else {
                                    if r#start.is_some() {
                                        return Err(serde::de::Error::duplicate_field("startDate"));
                                    }
                                    r#start = Some(GoalStart::Date(map_access.next_value()?));
                                }
                            }
                            Field::StartDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum =
                                        r#start.get_or_insert(GoalStart::Date(Default::default()));
                                    if let GoalStart::Date(variant) = r#enum {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_startDate",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_start[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "startDate",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
                                        ],
                                    ));
                                }
                            }
                            Field::StartCodeableConcept => {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "startCodeableConcept",
                                    ));
                                }
                                r#start =
                                    Some(GoalStart::CodeableConcept(map_access.next_value()?));
                            }
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
                            }
                            Field::StatusDate => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status_date.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusDate",
                                        ));
                                    }
                                    r#status_date = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status_date.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_statusDate",
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
                                        "statusDate",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
                                        ],
                                    ));
                                }
                            }
                            Field::StatusReason => {
                                if _ctx.from_json {
                                    let some = r#status_reason.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusReason",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status_reason.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "statusReason",
                                        ));
                                    }
                                    r#status_reason = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusReasonPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status_reason.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_statusReason",
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
                                        "statusReason",
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
                                            "lifecycleStatus",
                                            "achievementStatus",
                                            "category",
                                            "priority",
                                            "description",
                                            "subject",
                                            "startDate",
                                            "startCodeableConcept",
                                            "target",
                                            "statusDate",
                                            "statusReason",
                                            "expressedBy",
                                            "addresses",
                                            "note",
                                            "outcomeCode",
                                            "outcomeReference",
                                        ],
                                    ));
                                }
                            }
                            Field::ExpressedBy => {
                                if r#expressed_by.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expressedBy"));
                                }
                                r#expressed_by = Some(map_access.next_value()?);
                            }
                            Field::Addresses => {
                                if r#addresses.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addresses"));
                                }
                                r#addresses = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
                            }
                            Field::OutcomeCode => {
                                if r#outcome_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcomeCode"));
                                }
                                r#outcome_code = Some(map_access.next_value()?);
                            }
                            Field::OutcomeReference => {
                                if r#outcome_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outcomeReference",
                                    ));
                                }
                                r#outcome_reference = Some(map_access.next_value()?);
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
                                        "lifecycleStatus",
                                        "achievementStatus",
                                        "category",
                                        "priority",
                                        "description",
                                        "subject",
                                        "startDate",
                                        "startCodeableConcept",
                                        "target",
                                        "statusDate",
                                        "statusReason",
                                        "expressedBy",
                                        "addresses",
                                        "note",
                                        "outcomeCode",
                                        "outcomeReference",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Goal {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#lifecycle_status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#lifecycle_status.unwrap_or(Default::default())
                        } else {
                            r#lifecycle_status
                                .ok_or(serde::de::Error::missing_field("lifecycleStatus"))?
                        },
                        r#achievement_status,
                        r#category: r#category.unwrap_or(vec![]),
                        r#priority,
                        r#description: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#description.unwrap_or(Default::default())
                        } else {
                            r#description.ok_or(serde::de::Error::missing_field("description"))?
                        },
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#start,
                        r#target: r#target.unwrap_or(vec![]),
                        r#status_date,
                        r#status_reason,
                        r#expressed_by,
                        r#addresses: r#addresses.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                        r#outcome_code: r#outcome_code.unwrap_or(vec![]),
                        r#outcome_reference: r#outcome_reference.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
