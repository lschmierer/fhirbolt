// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
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
#[derive(Default, Debug, Clone)]
pub struct GoalTarget {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Option<GoalTargetDetail>,
    pub r#due: Option<GoalTargetDue>,
}
impl serde::ser::Serialize for GoalTarget {
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("detailString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_detailString", &primitive_element)?;
                    }
                }
                GoalTargetDetail::Boolean(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("detailBoolean", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_detailBoolean", &primitive_element)?;
                    }
                }
                GoalTargetDetail::Integer(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("detailInteger", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_detailInteger", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("dueDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_dueDate", &primitive_element)?;
                    }
                }
                GoalTargetDue::Duration(ref value) => {
                    state.serialize_entry("dueDuration", value)?;
                }
                GoalTargetDue::Invalid => return Err(serde::ser::Error::custom("due is invalid")),
            }
        }
        state.end()
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                                return Err(serde::de::Error::duplicate_field("detailQuantity"));
                            }
                            r#detail = Some(GoalTargetDetail::Quantity(map_access.next_value()?));
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
                            r#detail =
                                Some(GoalTargetDetail::CodeableConcept(map_access.next_value()?));
                        }
                        Field::DetailString => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::String(Default::default()));
                            if let GoalTargetDetail::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("detail[x]"));
                            }
                        }
                        Field::DetailStringPrimitiveElement => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::String(Default::default()));
                            if let GoalTargetDetail::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_detailString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_detail[x]"));
                            }
                        }
                        Field::DetailBoolean => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::Boolean(Default::default()));
                            if let GoalTargetDetail::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("detail[x]"));
                            }
                        }
                        Field::DetailBooleanPrimitiveElement => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::Boolean(Default::default()));
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
                                return Err(serde::de::Error::duplicate_field("_detail[x]"));
                            }
                        }
                        Field::DetailInteger => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::Integer(Default::default()));
                            if let GoalTargetDetail::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("detail[x]"));
                            }
                        }
                        Field::DetailIntegerPrimitiveElement => {
                            let r#enum = r#detail
                                .get_or_insert(GoalTargetDetail::Integer(Default::default()));
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
                                return Err(serde::de::Error::duplicate_field("_detail[x]"));
                            }
                        }
                        Field::DetailRatio => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRatio"));
                            }
                            r#detail = Some(GoalTargetDetail::Ratio(map_access.next_value()?));
                        }
                        Field::DueDate => {
                            let r#enum =
                                r#due.get_or_insert(GoalTargetDue::Date(Default::default()));
                            if let GoalTargetDue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("due[x]"));
                            }
                        }
                        Field::DueDatePrimitiveElement => {
                            let r#enum =
                                r#due.get_or_insert(GoalTargetDue::Date(Default::default()));
                            if let GoalTargetDue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dueDate"));
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
                        }
                        Field::DueDuration => {
                            if r#due.is_some() {
                                return Err(serde::de::Error::duplicate_field("dueDuration"));
                            }
                            r#due = Some(GoalTargetDue::Duration(map_access.next_value()?));
                        }
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Goal {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#lifecycle_status: super::super::types::Code,
    pub r#achievement_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Box<super::super::types::CodeableConcept>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#start: Option<GoalStart>,
    pub r#target: Vec<GoalTarget>,
    pub r#status_date: Option<super::super::types::Date>,
    pub r#status_reason: Option<super::super::types::String>,
    pub r#expressed_by: Option<Box<super::super::types::Reference>>,
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#outcome_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Goal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Goal")?;
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
        if let Some(some) = self.r#lifecycle_status.value.as_ref() {
            state.serialize_entry("lifecycleStatus", some)?;
        }
        if self.r#lifecycle_status.id.is_some() || !self.r#lifecycle_status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#lifecycle_status.id,
                extension: &self.r#lifecycle_status.extension,
            };
            state.serialize_entry("_lifecycleStatus", &primitive_element)?;
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
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("startDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_startDate", &primitive_element)?;
                    }
                }
                GoalStart::CodeableConcept(ref value) => {
                    state.serialize_entry("startCodeableConcept", value)?;
                }
                GoalStart::Invalid => return Err(serde::ser::Error::custom("start is invalid")),
            }
        }
        if !self.r#target.is_empty() {
            state.serialize_entry("target", &self.r#target)?;
        }
        if let Some(some) = self.r#status_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("statusDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_statusDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#status_reason.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("statusReason", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_statusReason", &primitive_element)?;
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
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
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
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
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
                            some.value = Some(map_access.next_value()?);
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
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
                            let some = r#lifecycle_status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleStatus"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LifecycleStatusPrimitiveElement => {
                            let some = r#lifecycle_status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lifecycleStatus"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::AchievementStatus => {
                            if r#achievement_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("achievementStatus"));
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
                            let r#enum = r#start.get_or_insert(GoalStart::Date(Default::default()));
                            if let GoalStart::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("startDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("start[x]"));
                            }
                        }
                        Field::StartDatePrimitiveElement => {
                            let r#enum = r#start.get_or_insert(GoalStart::Date(Default::default()));
                            if let GoalStart::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_startDate"));
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
                        }
                        Field::StartCodeableConcept => {
                            if r#start.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "startCodeableConcept",
                                ));
                            }
                            r#start = Some(GoalStart::CodeableConcept(map_access.next_value()?));
                        }
                        Field::Target => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#target = Some(map_access.next_value()?);
                        }
                        Field::StatusDate => {
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusDatePrimitiveElement => {
                            let some = r#status_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_statusDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::StatusReason => {
                            let some = r#status_reason.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusReason"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusReasonPrimitiveElement => {
                            let some = r#status_reason.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_statusReason"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
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
                                return Err(serde::de::Error::duplicate_field("outcomeReference"));
                            }
                            r#outcome_reference = Some(map_access.next_value()?);
                        }
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
                    r#lifecycle_status: r#lifecycle_status
                        .ok_or(serde::de::Error::missing_field("lifecycleStatus"))?,
                    r#achievement_status,
                    r#category: r#category.unwrap_or(vec![]),
                    r#priority,
                    r#description: r#description
                        .ok_or(serde::de::Error::missing_field("description"))?,
                    r#subject: r#subject.ok_or(serde::de::Error::missing_field("subject"))?,
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
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
