// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PlanDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for PlanDefinitionSubject {
    fn default() -> PlanDefinitionSubject {
        PlanDefinitionSubject::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionGoalTargetDetail {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for PlanDefinitionGoalTargetDetail {
    fn default() -> PlanDefinitionGoalTargetDetail {
        PlanDefinitionGoalTargetDetail::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for PlanDefinitionActionSubject {
    fn default() -> PlanDefinitionActionSubject {
        PlanDefinitionActionSubject::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionRelatedActionOffset {
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Invalid,
}
impl Default for PlanDefinitionActionRelatedActionOffset {
    fn default() -> PlanDefinitionActionRelatedActionOffset {
        PlanDefinitionActionRelatedActionOffset::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionTiming {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Range(Box<super::super::types::Range>),
    Timing(Box<super::super::types::Timing>),
    Invalid,
}
impl Default for PlanDefinitionActionTiming {
    fn default() -> PlanDefinitionActionTiming {
        PlanDefinitionActionTiming::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionDefinition {
    Canonical(Box<super::super::types::Canonical>),
    Uri(Box<super::super::types::Uri>),
    Invalid,
}
impl Default for PlanDefinitionActionDefinition {
    fn default() -> PlanDefinitionActionDefinition {
        PlanDefinitionActionDefinition::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionGoalTarget {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Option<PlanDefinitionGoalTargetDetail>,
    pub r#due: Option<Box<super::super::types::Duration>>,
}
impl serde::ser::Serialize for PlanDefinitionGoalTarget {
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
                PlanDefinitionGoalTargetDetail::Quantity(ref value) => {
                    state.serialize_entry("detailQuantity", value)?;
                }
                PlanDefinitionGoalTargetDetail::Range(ref value) => {
                    state.serialize_entry("detailRange", value)?;
                }
                PlanDefinitionGoalTargetDetail::CodeableConcept(ref value) => {
                    state.serialize_entry("detailCodeableConcept", value)?;
                }
                PlanDefinitionGoalTargetDetail::Invalid => {
                    return Err(serde::ser::Error::custom("detail is invalid"))
                }
            }
        }
        if let Some(some) = self.r#due.as_ref() {
            state.serialize_entry("due", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionGoalTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionGoalTarget;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionGoalTarget")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PlanDefinitionGoalTarget, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#measure: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#detail: Option<PlanDefinitionGoalTargetDetail> = None;
                let mut r#due: Option<Box<super::super::types::Duration>> = None;
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
                        "measure" => {
                            if r#measure.is_some() {
                                return Err(serde::de::Error::duplicate_field("measure"));
                            }
                            r#measure = Some(map_access.next_value()?);
                        }
                        "detailQuantity" => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailQuantity"));
                            }
                            r#detail = Some(PlanDefinitionGoalTargetDetail::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        "detailRange" => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailRange"));
                            }
                            r#detail = Some(PlanDefinitionGoalTargetDetail::Range(
                                map_access.next_value()?,
                            ));
                        }
                        "detailCodeableConcept" => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "detailCodeableConcept",
                                ));
                            }
                            r#detail = Some(PlanDefinitionGoalTargetDetail::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "due" => {
                            if r#due.is_some() {
                                return Err(serde::de::Error::duplicate_field("due"));
                            }
                            r#due = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "measure",
                                    "detail",
                                    "due",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionGoalTarget {
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
pub struct PlanDefinitionGoal {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Box<super::super::types::CodeableConcept>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#start: Option<Box<super::super::types::CodeableConcept>>,
    pub r#addresses: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#target: Vec<PlanDefinitionGoalTarget>,
}
impl serde::ser::Serialize for PlanDefinitionGoal {
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
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        state.serialize_entry("description", &self.r#description)?;
        if let Some(some) = self.r#priority.as_ref() {
            state.serialize_entry("priority", some)?;
        }
        if let Some(some) = self.r#start.as_ref() {
            state.serialize_entry("start", some)?;
        }
        if !self.r#addresses.is_empty() {
            state.serialize_entry("addresses", &self.r#addresses)?;
        }
        if !self.r#documentation.is_empty() {
            state.serialize_entry("documentation", &self.r#documentation)?;
        }
        if !self.r#target.is_empty() {
            state.serialize_entry("target", &self.r#target)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionGoal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionGoal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionGoal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PlanDefinitionGoal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#priority: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#start: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#addresses: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#documentation: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#target: Option<Vec<PlanDefinitionGoalTarget>> = None;
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
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "description" => {
                            if r#description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            r#description = Some(map_access.next_value()?);
                        }
                        "priority" => {
                            if r#priority.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            r#priority = Some(map_access.next_value()?);
                        }
                        "start" => {
                            if r#start.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            r#start = Some(map_access.next_value()?);
                        }
                        "addresses" => {
                            if r#addresses.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            r#addresses = Some(map_access.next_value()?);
                        }
                        "documentation" => {
                            if r#documentation.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            r#documentation = Some(map_access.next_value()?);
                        }
                        "target" => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            r#target = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "category",
                                    "description",
                                    "priority",
                                    "start",
                                    "addresses",
                                    "documentation",
                                    "target",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionGoal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category,
                    r#description: r#description
                        .ok_or(serde::de::Error::missing_field("description"))?,
                    r#priority,
                    r#start,
                    r#addresses: r#addresses.unwrap_or(vec![]),
                    r#documentation: r#documentation.unwrap_or(vec![]),
                    r#target: r#target.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionCondition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#kind: super::super::types::Code,
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for PlanDefinitionActionCondition {
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
        if let Some(some) = self.r#kind.value.as_ref() {
            state.serialize_entry("kind", some)?;
        }
        if self.r#kind.id.is_some() || !self.r#kind.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#kind.id,
                extension: &self.r#kind.extension,
            };
            state.serialize_entry("_kind", &primitive_element)?;
        }
        if let Some(some) = self.r#expression.as_ref() {
            state.serialize_entry("expression", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionActionCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionActionCondition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionActionCondition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PlanDefinitionActionCondition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#kind: Option<super::super::types::Code> = None;
                let mut r#expression: Option<Box<super::super::types::Expression>> = None;
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
                        "kind" => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_kind" => {
                            let some = r#kind.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_kind"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "expression" => {
                            if r#expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            r#expression = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "kind",
                                    "expression",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionActionCondition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#kind: r#kind.ok_or(serde::de::Error::missing_field("kind"))?,
                    r#expression,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionRelatedAction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action_id: super::super::types::Id,
    pub r#relationship: super::super::types::Code,
    pub r#offset: Option<PlanDefinitionActionRelatedActionOffset>,
}
impl serde::ser::Serialize for PlanDefinitionActionRelatedAction {
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
        if let Some(some) = self.r#action_id.value.as_ref() {
            state.serialize_entry("actionId", some)?;
        }
        if self.r#action_id.id.is_some() || !self.r#action_id.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#action_id.id,
                extension: &self.r#action_id.extension,
            };
            state.serialize_entry("_actionId", &primitive_element)?;
        }
        if let Some(some) = self.r#relationship.value.as_ref() {
            state.serialize_entry("relationship", some)?;
        }
        if self.r#relationship.id.is_some() || !self.r#relationship.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#relationship.id,
                extension: &self.r#relationship.extension,
            };
            state.serialize_entry("_relationship", &primitive_element)?;
        }
        if let Some(some) = self.r#offset.as_ref() {
            match some {
                PlanDefinitionActionRelatedActionOffset::Duration(ref value) => {
                    state.serialize_entry("offsetDuration", value)?;
                }
                PlanDefinitionActionRelatedActionOffset::Range(ref value) => {
                    state.serialize_entry("offsetRange", value)?;
                }
                PlanDefinitionActionRelatedActionOffset::Invalid => {
                    return Err(serde::ser::Error::custom("offset is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionActionRelatedAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionActionRelatedAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionActionRelatedAction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PlanDefinitionActionRelatedAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#action_id: Option<super::super::types::Id> = None;
                let mut r#relationship: Option<super::super::types::Code> = None;
                let mut r#offset: Option<PlanDefinitionActionRelatedActionOffset> = None;
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
                        "actionId" => {
                            let some = r#action_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("actionId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_actionId" => {
                            let some = r#action_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_actionId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "relationship" => {
                            let some = r#relationship.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_relationship" => {
                            let some = r#relationship.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_relationship"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "offsetDuration" => {
                            if r#offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offsetDuration"));
                            }
                            r#offset = Some(PlanDefinitionActionRelatedActionOffset::Duration(
                                map_access.next_value()?,
                            ));
                        }
                        "offsetRange" => {
                            if r#offset.is_some() {
                                return Err(serde::de::Error::duplicate_field("offsetRange"));
                            }
                            r#offset = Some(PlanDefinitionActionRelatedActionOffset::Range(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "action_id",
                                    "relationship",
                                    "offset",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionActionRelatedAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#action_id: r#action_id.ok_or(serde::de::Error::missing_field("action_id"))?,
                    r#relationship: r#relationship
                        .ok_or(serde::de::Error::missing_field("relationship"))?,
                    r#offset,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for PlanDefinitionActionParticipant {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionActionParticipant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionActionParticipant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionActionParticipant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PlanDefinitionActionParticipant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
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
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "type", "role"],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionActionParticipant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#role,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionDynamicValue {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#expression: Option<Box<super::super::types::Expression>>,
}
impl serde::ser::Serialize for PlanDefinitionActionDynamicValue {
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
        if let Some(some) = self.r#path.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("path", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_path", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#expression.as_ref() {
            state.serialize_entry("expression", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionActionDynamicValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionActionDynamicValue;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionActionDynamicValue")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<PlanDefinitionActionDynamicValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#path: Option<super::super::types::String> = None;
                let mut r#expression: Option<Box<super::super::types::Expression>> = None;
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
                        "path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_path" => {
                            let some = r#path.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_path"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "expression" => {
                            if r#expression.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            r#expression = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "path",
                                    "expression",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionActionDynamicValue {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path,
                    r#expression,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionAction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#prefix: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#text_equivalent: Option<super::super::types::String>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#goal_id: Vec<super::super::types::Id>,
    pub r#subject: Option<PlanDefinitionActionSubject>,
    pub r#trigger: Vec<Box<super::super::types::TriggerDefinition>>,
    pub r#condition: Vec<PlanDefinitionActionCondition>,
    pub r#input: Vec<Box<super::super::types::DataRequirement>>,
    pub r#output: Vec<Box<super::super::types::DataRequirement>>,
    pub r#related_action: Vec<PlanDefinitionActionRelatedAction>,
    pub r#timing: Option<PlanDefinitionActionTiming>,
    pub r#participant: Vec<PlanDefinitionActionParticipant>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#grouping_behavior: Option<super::super::types::Code>,
    pub r#selection_behavior: Option<super::super::types::Code>,
    pub r#required_behavior: Option<super::super::types::Code>,
    pub r#precheck_behavior: Option<super::super::types::Code>,
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    pub r#definition: Option<PlanDefinitionActionDefinition>,
    pub r#transform: Option<super::super::types::Canonical>,
    pub r#dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    pub r#action: Vec<PlanDefinitionAction>,
}
impl serde::ser::Serialize for PlanDefinitionAction {
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
        if let Some(some) = self.r#prefix.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("prefix", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_prefix", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text_equivalent.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("textEquivalent", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_textEquivalent", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#priority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("priority", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_priority", &primitive_element)?;
            }
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if !self.r#reason.is_empty() {
            state.serialize_entry("reason", &self.r#reason)?;
        }
        if !self.r#documentation.is_empty() {
            state.serialize_entry("documentation", &self.r#documentation)?;
        }
        if !self.r#goal_id.is_empty() {
            let values: Vec<_> = self.r#goal_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("goalId", &values)?;
            }
            let requires_elements = self
                .r#goal_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#goal_id
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
                state.serialize_entry("_goalId", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#subject.as_ref() {
            match some {
                PlanDefinitionActionSubject::CodeableConcept(ref value) => {
                    state.serialize_entry("subjectCodeableConcept", value)?;
                }
                PlanDefinitionActionSubject::Reference(ref value) => {
                    state.serialize_entry("subjectReference", value)?;
                }
                PlanDefinitionActionSubject::Invalid => {
                    return Err(serde::ser::Error::custom("subject is invalid"))
                }
            }
        }
        if !self.r#trigger.is_empty() {
            state.serialize_entry("trigger", &self.r#trigger)?;
        }
        if !self.r#condition.is_empty() {
            state.serialize_entry("condition", &self.r#condition)?;
        }
        if !self.r#input.is_empty() {
            state.serialize_entry("input", &self.r#input)?;
        }
        if !self.r#output.is_empty() {
            state.serialize_entry("output", &self.r#output)?;
        }
        if !self.r#related_action.is_empty() {
            state.serialize_entry("relatedAction", &self.r#related_action)?;
        }
        if let Some(some) = self.r#timing.as_ref() {
            match some {
                PlanDefinitionActionTiming::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("timingDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_timingDateTime", &primitive_element)?;
                    }
                }
                PlanDefinitionActionTiming::Age(ref value) => {
                    state.serialize_entry("timingAge", value)?;
                }
                PlanDefinitionActionTiming::Period(ref value) => {
                    state.serialize_entry("timingPeriod", value)?;
                }
                PlanDefinitionActionTiming::Duration(ref value) => {
                    state.serialize_entry("timingDuration", value)?;
                }
                PlanDefinitionActionTiming::Range(ref value) => {
                    state.serialize_entry("timingRange", value)?;
                }
                PlanDefinitionActionTiming::Timing(ref value) => {
                    state.serialize_entry("timingTiming", value)?;
                }
                PlanDefinitionActionTiming::Invalid => {
                    return Err(serde::ser::Error::custom("timing is invalid"))
                }
            }
        }
        if !self.r#participant.is_empty() {
            state.serialize_entry("participant", &self.r#participant)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#grouping_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("groupingBehavior", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_groupingBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#selection_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("selectionBehavior", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_selectionBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#required_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("requiredBehavior", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_requiredBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#precheck_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("precheckBehavior", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_precheckBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#cardinality_behavior.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("cardinalityBehavior", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cardinalityBehavior", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#definition.as_ref() {
            match some {
                PlanDefinitionActionDefinition::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("definitionCanonical", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_definitionCanonical", &primitive_element)?;
                    }
                }
                PlanDefinitionActionDefinition::Uri(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("definitionUri", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_definitionUri", &primitive_element)?;
                    }
                }
                PlanDefinitionActionDefinition::Invalid => {
                    return Err(serde::ser::Error::custom("definition is invalid"))
                }
            }
        }
        if let Some(some) = self.r#transform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("transform", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_transform", &primitive_element)?;
            }
        }
        if !self.r#dynamic_value.is_empty() {
            state.serialize_entry("dynamicValue", &self.r#dynamic_value)?;
        }
        if !self.r#action.is_empty() {
            state.serialize_entry("action", &self.r#action)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinitionAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinitionAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinitionAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PlanDefinitionAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#prefix: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#text_equivalent: Option<super::super::types::String> = None;
                let mut r#priority: Option<super::super::types::Code> = None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#reason: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#documentation: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#goal_id: Option<Vec<super::super::types::Id>> = None;
                let mut r#subject: Option<PlanDefinitionActionSubject> = None;
                let mut r#trigger: Option<Vec<Box<super::super::types::TriggerDefinition>>> = None;
                let mut r#condition: Option<Vec<PlanDefinitionActionCondition>> = None;
                let mut r#input: Option<Vec<Box<super::super::types::DataRequirement>>> = None;
                let mut r#output: Option<Vec<Box<super::super::types::DataRequirement>>> = None;
                let mut r#related_action: Option<Vec<PlanDefinitionActionRelatedAction>> = None;
                let mut r#timing: Option<PlanDefinitionActionTiming> = None;
                let mut r#participant: Option<Vec<PlanDefinitionActionParticipant>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#grouping_behavior: Option<super::super::types::Code> = None;
                let mut r#selection_behavior: Option<super::super::types::Code> = None;
                let mut r#required_behavior: Option<super::super::types::Code> = None;
                let mut r#precheck_behavior: Option<super::super::types::Code> = None;
                let mut r#cardinality_behavior: Option<super::super::types::Code> = None;
                let mut r#definition: Option<PlanDefinitionActionDefinition> = None;
                let mut r#transform: Option<super::super::types::Canonical> = None;
                let mut r#dynamic_value: Option<Vec<PlanDefinitionActionDynamicValue>> = None;
                let mut r#action: Option<Vec<PlanDefinitionAction>> = None;
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
                        "prefix" => {
                            let some = r#prefix.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_prefix" => {
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
                        "title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_title" => {
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
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
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
                        "textEquivalent" => {
                            let some = r#text_equivalent.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("textEquivalent"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_textEquivalent" => {
                            let some = r#text_equivalent.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_textEquivalent"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "priority" => {
                            let some = r#priority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_priority" => {
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
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "reason" => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        "documentation" => {
                            if r#documentation.is_some() {
                                return Err(serde::de::Error::duplicate_field("documentation"));
                            }
                            r#documentation = Some(map_access.next_value()?);
                        }
                        "goalId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#goal_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("goalId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_goalId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#goal_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_goalId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "subjectCodeableConcept" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            r#subject = Some(PlanDefinitionActionSubject::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "subjectReference" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            r#subject = Some(PlanDefinitionActionSubject::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "trigger" => {
                            if r#trigger.is_some() {
                                return Err(serde::de::Error::duplicate_field("trigger"));
                            }
                            r#trigger = Some(map_access.next_value()?);
                        }
                        "condition" => {
                            if r#condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            r#condition = Some(map_access.next_value()?);
                        }
                        "input" => {
                            if r#input.is_some() {
                                return Err(serde::de::Error::duplicate_field("input"));
                            }
                            r#input = Some(map_access.next_value()?);
                        }
                        "output" => {
                            if r#output.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            r#output = Some(map_access.next_value()?);
                        }
                        "relatedAction" => {
                            if r#related_action.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedAction"));
                            }
                            r#related_action = Some(map_access.next_value()?);
                        }
                        "timingDateTime" => {
                            let r#enum = r#timing.get_or_insert(
                                PlanDefinitionActionTiming::DateTime(Default::default()),
                            );
                            if let PlanDefinitionActionTiming::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("timing[x]"));
                            }
                        }
                        "_timingDateTime" => {
                            let r#enum = r#timing.get_or_insert(
                                PlanDefinitionActionTiming::DateTime(Default::default()),
                            );
                            if let PlanDefinitionActionTiming::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_timingDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_timing[x]"));
                            }
                        }
                        "timingAge" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingAge"));
                            }
                            r#timing =
                                Some(PlanDefinitionActionTiming::Age(map_access.next_value()?));
                        }
                        "timingPeriod" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingPeriod"));
                            }
                            r#timing =
                                Some(PlanDefinitionActionTiming::Period(map_access.next_value()?));
                        }
                        "timingDuration" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingDuration"));
                            }
                            r#timing = Some(PlanDefinitionActionTiming::Duration(
                                map_access.next_value()?,
                            ));
                        }
                        "timingRange" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingRange"));
                            }
                            r#timing =
                                Some(PlanDefinitionActionTiming::Range(map_access.next_value()?));
                        }
                        "timingTiming" => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timingTiming"));
                            }
                            r#timing =
                                Some(PlanDefinitionActionTiming::Timing(map_access.next_value()?));
                        }
                        "participant" => {
                            if r#participant.is_some() {
                                return Err(serde::de::Error::duplicate_field("participant"));
                            }
                            r#participant = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "groupingBehavior" => {
                            let some = r#grouping_behavior.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("groupingBehavior"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_groupingBehavior" => {
                            let some = r#grouping_behavior.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_groupingBehavior"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "selectionBehavior" => {
                            let some = r#selection_behavior.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("selectionBehavior"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_selectionBehavior" => {
                            let some = r#selection_behavior.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_selectionBehavior",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "requiredBehavior" => {
                            let some = r#required_behavior.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("requiredBehavior"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_requiredBehavior" => {
                            let some = r#required_behavior.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_requiredBehavior"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "precheckBehavior" => {
                            let some = r#precheck_behavior.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("precheckBehavior"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_precheckBehavior" => {
                            let some = r#precheck_behavior.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_precheckBehavior"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "cardinalityBehavior" => {
                            let some = r#cardinality_behavior.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cardinalityBehavior",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_cardinalityBehavior" => {
                            let some = r#cardinality_behavior.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_cardinalityBehavior",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "definitionCanonical" => {
                            let r#enum = r#definition.get_or_insert(
                                PlanDefinitionActionDefinition::Canonical(Default::default()),
                            );
                            if let PlanDefinitionActionDefinition::Canonical(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "definitionCanonical",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("definition[x]"));
                            }
                        }
                        "_definitionCanonical" => {
                            let r#enum = r#definition.get_or_insert(
                                PlanDefinitionActionDefinition::Canonical(Default::default()),
                            );
                            if let PlanDefinitionActionDefinition::Canonical(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_definitionCanonical",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_definition[x]"));
                            }
                        }
                        "definitionUri" => {
                            let r#enum = r#definition.get_or_insert(
                                PlanDefinitionActionDefinition::Uri(Default::default()),
                            );
                            if let PlanDefinitionActionDefinition::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definitionUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("definition[x]"));
                            }
                        }
                        "_definitionUri" => {
                            let r#enum = r#definition.get_or_insert(
                                PlanDefinitionActionDefinition::Uri(Default::default()),
                            );
                            if let PlanDefinitionActionDefinition::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_definitionUri",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_definition[x]"));
                            }
                        }
                        "transform" => {
                            let some = r#transform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_transform" => {
                            let some = r#transform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_transform"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "dynamicValue" => {
                            if r#dynamic_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicValue"));
                            }
                            r#dynamic_value = Some(map_access.next_value()?);
                        }
                        "action" => {
                            if r#action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#action = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "prefix",
                                    "title",
                                    "description",
                                    "text_equivalent",
                                    "priority",
                                    "code",
                                    "reason",
                                    "documentation",
                                    "goal_id",
                                    "subject",
                                    "trigger",
                                    "condition",
                                    "input",
                                    "output",
                                    "related_action",
                                    "timing",
                                    "participant",
                                    "type",
                                    "grouping_behavior",
                                    "selection_behavior",
                                    "required_behavior",
                                    "precheck_behavior",
                                    "cardinality_behavior",
                                    "definition",
                                    "transform",
                                    "dynamic_value",
                                    "action",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinitionAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#prefix,
                    r#title,
                    r#description,
                    r#text_equivalent,
                    r#priority,
                    r#code: r#code.unwrap_or(vec![]),
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#documentation: r#documentation.unwrap_or(vec![]),
                    r#goal_id: r#goal_id.unwrap_or(vec![]),
                    r#subject,
                    r#trigger: r#trigger.unwrap_or(vec![]),
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#input: r#input.unwrap_or(vec![]),
                    r#output: r#output.unwrap_or(vec![]),
                    r#related_action: r#related_action.unwrap_or(vec![]),
                    r#timing,
                    r#participant: r#participant.unwrap_or(vec![]),
                    r#type,
                    r#grouping_behavior,
                    r#selection_behavior,
                    r#required_behavior,
                    r#precheck_behavior,
                    r#cardinality_behavior,
                    r#definition,
                    r#transform,
                    r#dynamic_value: r#dynamic_value.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct PlanDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#version: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#subject: Option<PlanDefinitionSubject>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#usage: Option<super::super::types::String>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#goal: Vec<PlanDefinitionGoal>,
    pub r#action: Vec<PlanDefinitionAction>,
}
impl serde::ser::Serialize for PlanDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "PlanDefinition")?;
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
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
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
                state.serialize_entry("version", some)?;
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
                state.serialize_entry("name", some)?;
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
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#subtitle.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subtitle", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subtitle", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
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
                state.serialize_entry("experimental", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_experimental", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#subject.as_ref() {
            match some {
                PlanDefinitionSubject::CodeableConcept(ref value) => {
                    state.serialize_entry("subjectCodeableConcept", value)?;
                }
                PlanDefinitionSubject::Reference(ref value) => {
                    state.serialize_entry("subjectReference", value)?;
                }
                PlanDefinitionSubject::Invalid => {
                    return Err(serde::ser::Error::custom("subject is invalid"))
                }
            }
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
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
                state.serialize_entry("publisher", some)?;
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
                state.serialize_entry("description", some)?;
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
                state.serialize_entry("purpose", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_purpose", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#usage.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("usage", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_usage", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#copyright.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("copyright", some)?;
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
                state.serialize_entry("approvalDate", some)?;
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
                state.serialize_entry("lastReviewDate", some)?;
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
        if !self.r#topic.is_empty() {
            state.serialize_entry("topic", &self.r#topic)?;
        }
        if !self.r#author.is_empty() {
            state.serialize_entry("author", &self.r#author)?;
        }
        if !self.r#editor.is_empty() {
            state.serialize_entry("editor", &self.r#editor)?;
        }
        if !self.r#reviewer.is_empty() {
            state.serialize_entry("reviewer", &self.r#reviewer)?;
        }
        if !self.r#endorser.is_empty() {
            state.serialize_entry("endorser", &self.r#endorser)?;
        }
        if !self.r#related_artifact.is_empty() {
            state.serialize_entry("relatedArtifact", &self.r#related_artifact)?;
        }
        if !self.r#library.is_empty() {
            let values: Vec<_> = self.r#library.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("library", &values)?;
            }
            let requires_elements = self
                .r#library
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#library
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
                state.serialize_entry("_library", &primitive_elements)?;
            }
        }
        if !self.r#goal.is_empty() {
            state.serialize_entry("goal", &self.r#goal)?;
        }
        if !self.r#action.is_empty() {
            state.serialize_entry("action", &self.r#action)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PlanDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PlanDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PlanDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PlanDefinition, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#subtitle: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#experimental: Option<super::super::types::Boolean> = None;
                let mut r#subject: Option<PlanDefinitionSubject> = None;
                let mut r#date: Option<super::super::types::DateTime> = None;
                let mut r#publisher: Option<super::super::types::String> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#description: Option<super::super::types::Markdown> = None;
                let mut r#use_context: Option<Vec<Box<super::super::types::UsageContext>>> = None;
                let mut r#jurisdiction: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#purpose: Option<super::super::types::Markdown> = None;
                let mut r#usage: Option<super::super::types::String> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
                let mut r#approval_date: Option<super::super::types::Date> = None;
                let mut r#last_review_date: Option<super::super::types::Date> = None;
                let mut r#effective_period: Option<Box<super::super::types::Period>> = None;
                let mut r#topic: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#author: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#editor: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#reviewer: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#endorser: Option<Vec<Box<super::super::types::ContactDetail>>> = None;
                let mut r#related_artifact: Option<Vec<Box<super::super::types::RelatedArtifact>>> =
                    None;
                let mut r#library: Option<Vec<super::super::types::Canonical>> = None;
                let mut r#goal: Option<Vec<PlanDefinitionGoal>> = None;
                let mut r#action: Option<Vec<PlanDefinitionAction>> = None;
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
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
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
                        "url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_url" => {
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_version" => {
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
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
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
                        "title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_title" => {
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
                        "subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtitle"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_subtitle"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
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
                        "experimental" => {
                            let some = r#experimental.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimental"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_experimental" => {
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
                        "subjectCodeableConcept" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            r#subject = Some(PlanDefinitionSubject::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "subjectReference" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            r#subject =
                                Some(PlanDefinitionSubject::Reference(map_access.next_value()?));
                        }
                        "date" => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_date" => {
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
                        "publisher" => {
                            let some = r#publisher.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_publisher" => {
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
                        "contact" => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
                        }
                        "description" => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_description" => {
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
                        "useContext" => {
                            if r#use_context.is_some() {
                                return Err(serde::de::Error::duplicate_field("useContext"));
                            }
                            r#use_context = Some(map_access.next_value()?);
                        }
                        "jurisdiction" => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some(map_access.next_value()?);
                        }
                        "purpose" => {
                            let some = r#purpose.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_purpose" => {
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
                        "usage" => {
                            let some = r#usage.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("usage"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_usage" => {
                            let some = r#usage.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_usage"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "copyright" => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("copyright"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_copyright" => {
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
                        "approvalDate" => {
                            let some = r#approval_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("approvalDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_approvalDate" => {
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
                        "lastReviewDate" => {
                            let some = r#last_review_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastReviewDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_lastReviewDate" => {
                            let some = r#last_review_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastReviewDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "effectivePeriod" => {
                            if r#effective_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectivePeriod"));
                            }
                            r#effective_period = Some(map_access.next_value()?);
                        }
                        "topic" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic"));
                            }
                            r#topic = Some(map_access.next_value()?);
                        }
                        "author" => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(map_access.next_value()?);
                        }
                        "editor" => {
                            if r#editor.is_some() {
                                return Err(serde::de::Error::duplicate_field("editor"));
                            }
                            r#editor = Some(map_access.next_value()?);
                        }
                        "reviewer" => {
                            if r#reviewer.is_some() {
                                return Err(serde::de::Error::duplicate_field("reviewer"));
                            }
                            r#reviewer = Some(map_access.next_value()?);
                        }
                        "endorser" => {
                            if r#endorser.is_some() {
                                return Err(serde::de::Error::duplicate_field("endorser"));
                            }
                            r#endorser = Some(map_access.next_value()?);
                        }
                        "relatedArtifact" => {
                            if r#related_artifact.is_some() {
                                return Err(serde::de::Error::duplicate_field("relatedArtifact"));
                            }
                            r#related_artifact = Some(map_access.next_value()?);
                        }
                        "library" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#library.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("library"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_library" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#library.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_library"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "goal" => {
                            if r#goal.is_some() {
                                return Err(serde::de::Error::duplicate_field("goal"));
                            }
                            r#goal = Some(map_access.next_value()?);
                        }
                        "action" => {
                            if r#action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#action = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "url",
                                    "identifier",
                                    "version",
                                    "name",
                                    "title",
                                    "subtitle",
                                    "type",
                                    "status",
                                    "experimental",
                                    "subject",
                                    "date",
                                    "publisher",
                                    "contact",
                                    "description",
                                    "use_context",
                                    "jurisdiction",
                                    "purpose",
                                    "usage",
                                    "copyright",
                                    "approval_date",
                                    "last_review_date",
                                    "effective_period",
                                    "topic",
                                    "author",
                                    "editor",
                                    "reviewer",
                                    "endorser",
                                    "related_artifact",
                                    "library",
                                    "goal",
                                    "action",
                                ],
                            ))
                        }
                    }
                }
                Ok(PlanDefinition {
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
                    r#subtitle,
                    r#type,
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#experimental,
                    r#subject,
                    r#date,
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#description,
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#jurisdiction: r#jurisdiction.unwrap_or(vec![]),
                    r#purpose,
                    r#usage,
                    r#copyright,
                    r#approval_date,
                    r#last_review_date,
                    r#effective_period,
                    r#topic: r#topic.unwrap_or(vec![]),
                    r#author: r#author.unwrap_or(vec![]),
                    r#editor: r#editor.unwrap_or(vec![]),
                    r#reviewer: r#reviewer.unwrap_or(vec![]),
                    r#endorser: r#endorser.unwrap_or(vec![]),
                    r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                    r#library: r#library.unwrap_or(vec![]),
                    r#goal: r#goal.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
