// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "A code, group definition, or canonical reference that describes  or identifies the intended subject of the plan definition. Canonical references are allowed to support the definition of protocols for drug and substance quality specifications, and is allowed to reference a MedicinalProductDefinition, SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition, or PackagedProductDefinition resource."]
#[derive(Debug, Clone)]
pub enum PlanDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    Invalid,
}
impl Default for PlanDefinitionSubject {
    fn default() -> PlanDefinitionSubject {
        PlanDefinitionSubject::Invalid
    }
}
#[doc = "The target value of the measure to be achieved to signify fulfillment of the goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT 0.6%, Clear solution, etc. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any value at or above the low value."]
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
#[doc = "A code, group definition, or canonical reference that describes the intended subject of the action and its children, if any. Canonical references are allowed to support the definition of protocols for drug and substance quality specifications, and is allowed to reference a MedicinalProductDefinition, SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition, or PackagedProductDefinition resource."]
#[derive(Debug, Clone)]
pub enum PlanDefinitionActionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    Invalid,
}
impl Default for PlanDefinitionActionSubject {
    fn default() -> PlanDefinitionActionSubject {
        PlanDefinitionActionSubject::Invalid
    }
}
#[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
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
#[doc = "An optional value describing when the action should be performed."]
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
#[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, or a PlanDefinition that describes a series of actions to be taken."]
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
#[doc = "Indicates what should be done and within what timeframe."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionGoalTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The parameter whose value is to be tracked, e.g. body weight, blood pressure, or hemoglobin A1c level."]
    pub r#measure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The target value of the measure to be achieved to signify fulfillment of the goal, e.g. 150 pounds or 7.0%, or in the case of pharmaceutical quality - NMT 0.6%, Clear solution, etc. Either the high or low or both values of the range can be specified. When a low value is missing, it indicates that the goal is achieved at any value at or below the high value. Similarly, if the high value is missing, it indicates that the goal is achieved at any value at or above the low value."]
    pub r#detail: Option<PlanDefinitionGoalTargetDetail>,
    #[doc = "Indicates the timeframe after the start of the goal in which the goal should be met."]
    pub r#due: Option<Box<super::super::types::Duration>>,
}
impl crate::AnyResource for PlanDefinitionGoalTarget {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
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
            #[serde(rename = "due")]
            Due,
            Unknown(String),
        }
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
                                r#detail = Some(PlanDefinitionGoalTargetDetail::Quantity(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DetailRange => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detailRange"));
                                }
                                r#detail = Some(PlanDefinitionGoalTargetDetail::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DetailCodeableConcept => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "detailCodeableConcept",
                                    ));
                                }
                                r#detail = Some(PlanDefinitionGoalTargetDetail::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Due => {
                                if r#due.is_some() {
                                    return Err(serde::de::Error::duplicate_field("due"));
                                }
                                r#due = Some(map_access.next_value()?);
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
                                        "measure",
                                        "detailQuantity",
                                        "detailRange",
                                        "detailCodeableConcept",
                                        "due",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A goal describes an expected outcome that activities within the plan are intended to achieve. For example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, meeting the acceptance criteria for a test as specified by a quality specification, etc."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionGoal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates a category the goal falls within."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-readable and/or coded description of a specific desired objective of care, such as \"control blood pressure\" or \"negotiate an obstacle course\" or \"dance with child at wedding\"."]
    pub r#description: Box<super::super::types::CodeableConcept>,
    #[doc = "Identifies the expected level of importance associated with reaching/sustaining the defined goal."]
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The event after which the goal should begin being pursued."]
    pub r#start: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies problems, conditions, issues, or concerns the goal is intended to address."]
    pub r#addresses: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Didactic or other informational resources associated with the goal that provide further supporting information about the goal. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Indicates what should be done and within what timeframe."]
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "start")]
            Start,
            #[serde(rename = "addresses")]
            Addresses,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "target")]
            Target,
            Unknown(String),
        }
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
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Description => {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value()?);
                            }
                            Field::Priority => {
                                if r#priority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("priority"));
                                }
                                r#priority = Some(map_access.next_value()?);
                            }
                            Field::Start => {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                r#start = Some(map_access.next_value()?);
                            }
                            Field::Addresses => {
                                if r#addresses.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addresses"));
                                }
                                r#addresses = Some(map_access.next_value()?);
                            }
                            Field::Documentation => {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value()?);
                            }
                            Field::Target => {
                                if r#target.is_some() {
                                    return Err(serde::de::Error::duplicate_field("target"));
                                }
                                r#target = Some(map_access.next_value()?);
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
                                        "category",
                                        "description",
                                        "priority",
                                        "start",
                                        "addresses",
                                        "documentation",
                                        "target",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PlanDefinitionGoal {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category,
                        r#description: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#description.unwrap_or(Default::default())
                        } else {
                            r#description.ok_or(serde::de::Error::missing_field("description"))?
                        },
                        r#priority,
                        r#start,
                        r#addresses: r#addresses.unwrap_or(vec![]),
                        r#documentation: r#documentation.unwrap_or(vec![]),
                        r#target: r#target.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An expression that describes applicability criteria or start/stop conditions for the action."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionCondition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of condition."]
    pub r#kind: super::super::types::Code,
    #[doc = "An expression that returns true or false, indicating whether the condition is satisfied."]
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
            let some = Ok(some)?;
            state.serialize_entry("kind", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "kind")]
            Kind,
            #[serde(rename = "_kind")]
            KindPrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            Unknown(String),
        }
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
                            Field::Kind => {
                                let some = r#kind.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("kind"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::KindPrimitiveElement => {
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
                            Field::Expression => {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                r#expression = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "kind", "expression"],
                                ));
                            },
                        }
                    }
                    Ok(PlanDefinitionActionCondition {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#kind: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#kind.unwrap_or(Default::default())
                        } else {
                            r#kind.ok_or(serde::de::Error::missing_field("kind"))?
                        },
                        r#expression,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionRelatedAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The element id of the related action."]
    pub r#action_id: super::super::types::Id,
    #[doc = "The relationship of this action to the related action."]
    pub r#relationship: super::super::types::Code,
    #[doc = "A duration or range of durations to apply to the relationship. For example, 30-60 minutes before."]
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
            let some = Ok(some)?;
            state.serialize_entry("actionId", &some)?;
        }
        if self.r#action_id.id.is_some() || !self.r#action_id.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#action_id.id,
                extension: &self.r#action_id.extension,
            };
            state.serialize_entry("_actionId", &primitive_element)?;
        }
        if let Some(some) = self.r#relationship.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("relationship", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "actionId")]
            ActionId,
            #[serde(rename = "_actionId")]
            ActionIdPrimitiveElement,
            #[serde(rename = "relationship")]
            Relationship,
            #[serde(rename = "_relationship")]
            RelationshipPrimitiveElement,
            #[serde(rename = "offsetDuration")]
            OffsetDuration,
            #[serde(rename = "offsetRange")]
            OffsetRange,
            Unknown(String),
        }
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
                            Field::ActionId => {
                                let some = r#action_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("actionId"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ActionIdPrimitiveElement => {
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
                            Field::Relationship => {
                                let some = r#relationship.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relationship"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RelationshipPrimitiveElement => {
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
                            Field::OffsetDuration => {
                                if r#offset.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "offsetDuration",
                                    ));
                                }
                                r#offset = Some(PlanDefinitionActionRelatedActionOffset::Duration(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::OffsetRange => {
                                if r#offset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offsetRange"));
                                }
                                r#offset = Some(PlanDefinitionActionRelatedActionOffset::Range(
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
                                        "actionId",
                                        "relationship",
                                        "offsetDuration",
                                        "offsetRange",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(PlanDefinitionActionRelatedAction {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#action_id: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#action_id.unwrap_or(Default::default())
                        } else {
                            r#action_id.ok_or(serde::de::Error::missing_field("actionId"))?
                        },
                        r#relationship: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#relationship.unwrap_or(Default::default())
                        } else {
                            r#relationship.ok_or(serde::de::Error::missing_field("relationship"))?
                        },
                        r#offset,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates who should participate in performing the action described."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionParticipant {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of participant in the action."]
    pub r#type: super::super::types::Code,
    #[doc = "The role the participant should play in performing the described action."]
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
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "role")]
            Role,
            Unknown(String),
        }
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
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "type", "role"],
                                ));
                            },
                        }
                    }
                    Ok(PlanDefinitionActionParticipant {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#role,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionActionDynamicValue {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The path to the element to be customized. This is the path on the resource that will hold the result of the calculation defined by the expression. The specified path SHALL be a FHIRPath resolveable on the specified target type of the ActivityDefinition, and SHALL consist only of identifiers, constant indexers, and a restricted subset of functions. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "An expression specifying the value of the customized element."]
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
                let some = Ok(some)?;
                state.serialize_entry("path", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "path")]
            Path,
            #[serde(rename = "_path")]
            PathPrimitiveElement,
            #[serde(rename = "expression")]
            Expression,
            Unknown(String),
        }
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
                            Field::Path => {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PathPrimitiveElement => {
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
                            Field::Expression => {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                r#expression = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "path", "expression"],
                                ));
                            },
                        }
                    }
                    Ok(PlanDefinitionActionDynamicValue {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#path,
                        r#expression,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An action or group of actions to be taken as part of the plan. For example, in clinical care, an action would be to prescribe a particular indicated medication, or perform a particular test as appropriate. In pharmaceutical quality, an action would be the test that needs to be performed on a drug product as defined in the quality specification."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinitionAction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A user-visible prefix for the action."]
    pub r#prefix: Option<super::super::types::String>,
    #[doc = "The textual description of the action displayed to a user. For example, when the action is a test to be performed, the title would be the title of the test such as Assay by HPLC."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A brief description of the action used to provide a summary to display to the user."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A text equivalent of the action to be performed. This provides a human-interpretable description of the action when the definition is consumed by a system that might not be capable of interpreting it dynamically."]
    pub r#text_equivalent: Option<super::super::types::String>,
    #[doc = "Indicates how quickly the action should be addressed with respect to other actions."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "A code that provides a meaning, grouping, or classification for the action or action group. For example, a section may have a LOINC code for the section of a documentation template. In pharmaceutical quality, an action (Test) such as pH could be classified as a physical property."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A description of why this action is necessary or appropriate."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Didactic or other informational resources associated with the action that can be provided to the CDS recipient. Information resources can include inline text commentary and links to web resources."]
    pub r#documentation: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "Identifies goals that this action supports. The reference must be to a goal element defined within this plan definition. In pharmaceutical quality, a goal represents acceptance criteria (Goal) for a given action (Test), so the goalId would be the unique id of a defined goal element establishing the acceptance criteria for the action."]
    pub r#goal_id: Vec<super::super::types::Id>,
    #[doc = "A code, group definition, or canonical reference that describes the intended subject of the action and its children, if any. Canonical references are allowed to support the definition of protocols for drug and substance quality specifications, and is allowed to reference a MedicinalProductDefinition, SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition, or PackagedProductDefinition resource."]
    pub r#subject: Option<PlanDefinitionActionSubject>,
    #[doc = "A description of when the action should be triggered."]
    pub r#trigger: Vec<Box<super::super::types::TriggerDefinition>>,
    #[doc = "An expression that describes applicability criteria or start/stop conditions for the action."]
    pub r#condition: Vec<PlanDefinitionActionCondition>,
    #[doc = "Defines input data requirements for the action."]
    pub r#input: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "Defines the outputs of the action, if any."]
    pub r#output: Vec<Box<super::super::types::DataRequirement>>,
    #[doc = "A relationship to another action such as \"before\" or \"30-60 minutes after start of\"."]
    pub r#related_action: Vec<PlanDefinitionActionRelatedAction>,
    #[doc = "An optional value describing when the action should be performed."]
    pub r#timing: Option<PlanDefinitionActionTiming>,
    #[doc = "Indicates who should participate in performing the action described."]
    pub r#participant: Vec<PlanDefinitionActionParticipant>,
    #[doc = "The type of action to perform (create, update, remove)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Defines the grouping behavior for the action and its children."]
    pub r#grouping_behavior: Option<super::super::types::Code>,
    #[doc = "Defines the selection behavior for the action and its children."]
    pub r#selection_behavior: Option<super::super::types::Code>,
    #[doc = "Defines the required behavior for the action."]
    pub r#required_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action should usually be preselected."]
    pub r#precheck_behavior: Option<super::super::types::Code>,
    #[doc = "Defines whether the action can be selected multiple times."]
    pub r#cardinality_behavior: Option<super::super::types::Code>,
    #[doc = "A reference to an ActivityDefinition that describes the action to be taken in detail, or a PlanDefinition that describes a series of actions to be taken."]
    pub r#definition: Option<PlanDefinitionActionDefinition>,
    #[doc = "A reference to a StructureMap resource that defines a transform that can be executed to produce the intent resource using the ActivityDefinition instance as the input."]
    pub r#transform: Option<super::super::types::Canonical>,
    #[doc = "Customizations that should be applied to the statically defined resource. For example, if the dosage of a medication must be computed based on the patient's weight, a customization would be used to specify an expression that calculated the weight, and the path on the resource that would contain the result."]
    pub r#dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    #[doc = "Sub actions that are contained within the action. The behavior of this action determines the functionality of the sub-actions. For example, a selection behavior of at-most-one indicates that of the sub-actions, at most one may be chosen as part of realizing the action definition."]
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
        if let Some(some) = self.r#text_equivalent.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("textEquivalent", &some)?;
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
            let values = self
                .r#goal_id
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
                PlanDefinitionActionSubject::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subjectCanonical", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_subjectCanonical", &primitive_element)?;
                    }
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
                        let some = Ok(some)?;
                        state.serialize_entry("timingDateTime", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("groupingBehavior", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("selectionBehavior", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("requiredBehavior", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("precheckBehavior", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("cardinalityBehavior", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("definitionCanonical", &some)?;
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
                        let some = Ok(some)?;
                        state.serialize_entry("definitionUri", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("transform", &some)?;
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
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "prefix")]
            Prefix,
            #[serde(rename = "_prefix")]
            PrefixPrimitiveElement,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "textEquivalent")]
            TextEquivalent,
            #[serde(rename = "_textEquivalent")]
            TextEquivalentPrimitiveElement,
            #[serde(rename = "priority")]
            Priority,
            #[serde(rename = "_priority")]
            PriorityPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "documentation")]
            Documentation,
            #[serde(rename = "goalId")]
            GoalId,
            #[serde(rename = "_goalId")]
            GoalIdPrimitiveElement,
            #[serde(rename = "subjectCodeableConcept")]
            SubjectCodeableConcept,
            #[serde(rename = "subjectReference")]
            SubjectReference,
            #[serde(rename = "subjectCanonical")]
            SubjectCanonical,
            #[serde(rename = "_subjectCanonical")]
            SubjectCanonicalPrimitiveElement,
            #[serde(rename = "trigger")]
            Trigger,
            #[serde(rename = "condition")]
            Condition,
            #[serde(rename = "input")]
            Input,
            #[serde(rename = "output")]
            Output,
            #[serde(rename = "relatedAction")]
            RelatedAction,
            #[serde(rename = "timingDateTime")]
            TimingDateTime,
            #[serde(rename = "_timingDateTime")]
            TimingDateTimePrimitiveElement,
            #[serde(rename = "timingAge")]
            TimingAge,
            #[serde(rename = "timingPeriod")]
            TimingPeriod,
            #[serde(rename = "timingDuration")]
            TimingDuration,
            #[serde(rename = "timingRange")]
            TimingRange,
            #[serde(rename = "timingTiming")]
            TimingTiming,
            #[serde(rename = "participant")]
            Participant,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "groupingBehavior")]
            GroupingBehavior,
            #[serde(rename = "_groupingBehavior")]
            GroupingBehaviorPrimitiveElement,
            #[serde(rename = "selectionBehavior")]
            SelectionBehavior,
            #[serde(rename = "_selectionBehavior")]
            SelectionBehaviorPrimitiveElement,
            #[serde(rename = "requiredBehavior")]
            RequiredBehavior,
            #[serde(rename = "_requiredBehavior")]
            RequiredBehaviorPrimitiveElement,
            #[serde(rename = "precheckBehavior")]
            PrecheckBehavior,
            #[serde(rename = "_precheckBehavior")]
            PrecheckBehaviorPrimitiveElement,
            #[serde(rename = "cardinalityBehavior")]
            CardinalityBehavior,
            #[serde(rename = "_cardinalityBehavior")]
            CardinalityBehaviorPrimitiveElement,
            #[serde(rename = "definitionCanonical")]
            DefinitionCanonical,
            #[serde(rename = "_definitionCanonical")]
            DefinitionCanonicalPrimitiveElement,
            #[serde(rename = "definitionUri")]
            DefinitionUri,
            #[serde(rename = "_definitionUri")]
            DefinitionUriPrimitiveElement,
            #[serde(rename = "transform")]
            Transform,
            #[serde(rename = "_transform")]
            TransformPrimitiveElement,
            #[serde(rename = "dynamicValue")]
            DynamicValue,
            #[serde(rename = "action")]
            Action,
            Unknown(String),
        }
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
                            Field::TextEquivalent => {
                                let some = r#text_equivalent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "textEquivalent",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TextEquivalentPrimitiveElement => {
                                let some = r#text_equivalent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_textEquivalent",
                                    ));
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
                            Field::Reason => {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason = Some(map_access.next_value()?);
                            }
                            Field::Documentation => {
                                if r#documentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("documentation"));
                                }
                                r#documentation = Some(map_access.next_value()?);
                            }
                            Field::GoalId => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#goal_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("goalId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::GoalIdPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#goal_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_goalId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::SubjectCodeableConcept => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectCodeableConcept",
                                    ));
                                }
                                r#subject = Some(PlanDefinitionActionSubject::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::SubjectReference => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectReference",
                                    ));
                                }
                                r#subject = Some(PlanDefinitionActionSubject::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::SubjectCanonical => {
                                let r#enum = r#subject.get_or_insert(
                                    PlanDefinitionActionSubject::Canonical(Default::default()),
                                );
                                if let PlanDefinitionActionSubject::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subjectCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("subject[x]"));
                                }
                            }
                            Field::SubjectCanonicalPrimitiveElement => {
                                let r#enum = r#subject.get_or_insert(
                                    PlanDefinitionActionSubject::Canonical(Default::default()),
                                );
                                if let PlanDefinitionActionSubject::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_subjectCanonical",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_subject[x]"));
                                }
                            }
                            Field::Trigger => {
                                if r#trigger.is_some() {
                                    return Err(serde::de::Error::duplicate_field("trigger"));
                                }
                                r#trigger = Some(map_access.next_value()?);
                            }
                            Field::Condition => {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value()?);
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
                            Field::RelatedAction => {
                                if r#related_action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatedAction"));
                                }
                                r#related_action = Some(map_access.next_value()?);
                            }
                            Field::TimingDateTime => {
                                let r#enum = r#timing.get_or_insert(
                                    PlanDefinitionActionTiming::DateTime(Default::default()),
                                );
                                if let PlanDefinitionActionTiming::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timingDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("timing[x]"));
                                }
                            }
                            Field::TimingDateTimePrimitiveElement => {
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
                            Field::TimingAge => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingAge"));
                                }
                                r#timing =
                                    Some(PlanDefinitionActionTiming::Age(map_access.next_value()?));
                            }
                            Field::TimingPeriod => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingPeriod"));
                                }
                                r#timing = Some(PlanDefinitionActionTiming::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TimingDuration => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "timingDuration",
                                    ));
                                }
                                r#timing = Some(PlanDefinitionActionTiming::Duration(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TimingRange => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingRange"));
                                }
                                r#timing = Some(PlanDefinitionActionTiming::Range(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::TimingTiming => {
                                if r#timing.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timingTiming"));
                                }
                                r#timing = Some(PlanDefinitionActionTiming::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Participant => {
                                if r#participant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("participant"));
                                }
                                r#participant = Some(map_access.next_value()?);
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::GroupingBehavior => {
                                let some = r#grouping_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "groupingBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::GroupingBehaviorPrimitiveElement => {
                                let some = r#grouping_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_groupingBehavior",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::SelectionBehavior => {
                                let some = r#selection_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "selectionBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SelectionBehaviorPrimitiveElement => {
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
                            Field::RequiredBehavior => {
                                let some = r#required_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "requiredBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::RequiredBehaviorPrimitiveElement => {
                                let some = r#required_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_requiredBehavior",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::PrecheckBehavior => {
                                let some = r#precheck_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "precheckBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PrecheckBehaviorPrimitiveElement => {
                                let some = r#precheck_behavior.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_precheckBehavior",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::CardinalityBehavior => {
                                let some = r#cardinality_behavior.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "cardinalityBehavior",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CardinalityBehaviorPrimitiveElement => {
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
                            Field::DefinitionCanonical => {
                                let r#enum = r#definition.get_or_insert(
                                    PlanDefinitionActionDefinition::Canonical(Default::default()),
                                );
                                if let PlanDefinitionActionDefinition::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definitionCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("definition[x]"));
                                }
                            }
                            Field::DefinitionCanonicalPrimitiveElement => {
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_definition[x]",
                                    ));
                                }
                            }
                            Field::DefinitionUri => {
                                let r#enum = r#definition.get_or_insert(
                                    PlanDefinitionActionDefinition::Uri(Default::default()),
                                );
                                if let PlanDefinitionActionDefinition::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "definitionUri",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("definition[x]"));
                                }
                            }
                            Field::DefinitionUriPrimitiveElement => {
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_definition[x]",
                                    ));
                                }
                            }
                            Field::Transform => {
                                let some = r#transform.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("transform"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TransformPrimitiveElement => {
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
                            Field::DynamicValue => {
                                if r#dynamic_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dynamicValue"));
                                }
                                r#dynamic_value = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
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
                                        "prefix",
                                        "title",
                                        "description",
                                        "textEquivalent",
                                        "priority",
                                        "code",
                                        "reason",
                                        "documentation",
                                        "goalId",
                                        "subjectCodeableConcept",
                                        "subjectReference",
                                        "subjectCanonical",
                                        "trigger",
                                        "condition",
                                        "input",
                                        "output",
                                        "relatedAction",
                                        "timingDateTime",
                                        "timingAge",
                                        "timingPeriod",
                                        "timingDuration",
                                        "timingRange",
                                        "timingTiming",
                                        "participant",
                                        "type",
                                        "groupingBehavior",
                                        "selectionBehavior",
                                        "requiredBehavior",
                                        "precheckBehavior",
                                        "cardinalityBehavior",
                                        "definitionCanonical",
                                        "definitionUri",
                                        "transform",
                                        "dynamicValue",
                                        "action",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact. The resource is general enough to support the description of a broad range of clinical and non-clinical artifacts such as clinical decision support rules, order sets, protocols, and drug quality specifications."]
#[derive(Default, Debug, Clone)]
pub struct PlanDefinition {
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
    #[doc = "An absolute URI that is used to identify this plan definition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this plan definition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the plan definition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this plan definition when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the plan definition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the plan definition author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence. To provide a version consistent with the Decision Support Service specification, use the format Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the Decision Support Service specification. Note that a version is required for non-experimental active artifacts."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the plan definition. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the plan definition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "An explanatory or alternate title for the plan definition giving additional information about its content."]
    pub r#subtitle: Option<super::super::types::String>,
    #[doc = "A high-level category for the plan definition that distinguishes the kinds of systems that would be interested in the plan definition."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of this plan definition. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this plan definition is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "A code, group definition, or canonical reference that describes  or identifies the intended subject of the plan definition. Canonical references are allowed to support the definition of protocols for drug and substance quality specifications, and is allowed to reference a MedicinalProductDefinition, SubstanceDefinition, AdministrableProductDefinition, ManufacturedItemDefinition, or PackagedProductDefinition resource."]
    pub r#subject: Option<PlanDefinitionSubject>,
    #[doc = "The date  (and optionally time) when the plan definition was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the plan definition changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the plan definition."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "A free text natural language description of the plan definition from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate plan definition instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the plan definition is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this plan definition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A detailed description of how the plan definition is used from a clinical perspective."]
    pub r#usage: Option<super::super::types::String>,
    #[doc = "A copyright statement relating to the plan definition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the plan definition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the plan definition content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptive topics related to the content of the plan definition. Topics provide a high-level categorization of the definition that can be useful for filtering and searching."]
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "An individual or organization responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "Related artifacts such as additional documentation, justification, or bibliographic references."]
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    #[doc = "A reference to a Library resource containing any formal logic used by the plan definition."]
    pub r#library: Vec<super::super::types::Canonical>,
    #[doc = "A goal describes an expected outcome that activities within the plan are intended to achieve. For example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, meeting the acceptance criteria for a test as specified by a quality specification, etc."]
    pub r#goal: Vec<PlanDefinitionGoal>,
    #[doc = "An action or group of actions to be taken as part of the plan. For example, in clinical care, an action would be to prescribe a particular indicated medication, or perform a particular test as appropriate. In pharmaceutical quality, an action would be the test that needs to be performed on a drug product as defined in the quality specification."]
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
        if let Some(some) = self.r#subtitle.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("subtitle", &some)?;
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
        if let Some(some) = self.r#subject.as_ref() {
            match some {
                PlanDefinitionSubject::CodeableConcept(ref value) => {
                    state.serialize_entry("subjectCodeableConcept", value)?;
                }
                PlanDefinitionSubject::Reference(ref value) => {
                    state.serialize_entry("subjectReference", value)?;
                }
                PlanDefinitionSubject::Canonical(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("subjectCanonical", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_subjectCanonical", &primitive_element)?;
                    }
                }
                PlanDefinitionSubject::Invalid => {
                    return Err(serde::ser::Error::custom("subject is invalid"))
                }
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
        if let Some(some) = self.r#usage.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("usage", &some)?;
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
            let values = self
                .r#library
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
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
            #[serde(rename = "subtitle")]
            Subtitle,
            #[serde(rename = "_subtitle")]
            SubtitlePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "experimental")]
            Experimental,
            #[serde(rename = "_experimental")]
            ExperimentalPrimitiveElement,
            #[serde(rename = "subjectCodeableConcept")]
            SubjectCodeableConcept,
            #[serde(rename = "subjectReference")]
            SubjectReference,
            #[serde(rename = "subjectCanonical")]
            SubjectCanonical,
            #[serde(rename = "_subjectCanonical")]
            SubjectCanonicalPrimitiveElement,
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
            #[serde(rename = "usage")]
            Usage,
            #[serde(rename = "_usage")]
            UsagePrimitiveElement,
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
            #[serde(rename = "topic")]
            Topic,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "editor")]
            Editor,
            #[serde(rename = "reviewer")]
            Reviewer,
            #[serde(rename = "endorser")]
            Endorser,
            #[serde(rename = "relatedArtifact")]
            RelatedArtifact,
            #[serde(rename = "library")]
            Library,
            #[serde(rename = "_library")]
            LibraryPrimitiveElement,
            #[serde(rename = "goal")]
            Goal,
            #[serde(rename = "action")]
            Action,
            Unknown(String),
        }
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
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "PlanDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"PlanDefinition",
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
                            Field::Subtitle => {
                                let some = r#subtitle.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtitle"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::SubtitlePrimitiveElement => {
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
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
                            Field::SubjectCodeableConcept => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectCodeableConcept",
                                    ));
                                }
                                r#subject = Some(PlanDefinitionSubject::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::SubjectReference => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subjectReference",
                                    ));
                                }
                                r#subject = Some(PlanDefinitionSubject::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::SubjectCanonical => {
                                let r#enum = r#subject.get_or_insert(
                                    PlanDefinitionSubject::Canonical(Default::default()),
                                );
                                if let PlanDefinitionSubject::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "subjectCanonical",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("subject[x]"));
                                }
                            }
                            Field::SubjectCanonicalPrimitiveElement => {
                                let r#enum = r#subject.get_or_insert(
                                    PlanDefinitionSubject::Canonical(Default::default()),
                                );
                                if let PlanDefinitionSubject::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_subjectCanonical",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_subject[x]"));
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
                            Field::Usage => {
                                let some = r#usage.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usage"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::UsagePrimitiveElement => {
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
                            Field::Topic => {
                                if r#topic.is_some() {
                                    return Err(serde::de::Error::duplicate_field("topic"));
                                }
                                r#topic = Some(map_access.next_value()?);
                            }
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Editor => {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                r#editor = Some(map_access.next_value()?);
                            }
                            Field::Reviewer => {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                r#reviewer = Some(map_access.next_value()?);
                            }
                            Field::Endorser => {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                r#endorser = Some(map_access.next_value()?);
                            }
                            Field::RelatedArtifact => {
                                if r#related_artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedArtifact",
                                    ));
                                }
                                r#related_artifact = Some(map_access.next_value()?);
                            }
                            Field::Library => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#library.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("library"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::LibraryPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#library.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_library"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Goal => {
                                if r#goal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("goal"));
                                }
                                r#goal = Some(map_access.next_value()?);
                            }
                            Field::Action => {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action = Some(map_access.next_value()?);
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
                                        "subtitle",
                                        "type",
                                        "status",
                                        "experimental",
                                        "subjectCodeableConcept",
                                        "subjectReference",
                                        "subjectCanonical",
                                        "date",
                                        "publisher",
                                        "contact",
                                        "description",
                                        "useContext",
                                        "jurisdiction",
                                        "purpose",
                                        "usage",
                                        "copyright",
                                        "approvalDate",
                                        "lastReviewDate",
                                        "effectivePeriod",
                                        "topic",
                                        "author",
                                        "editor",
                                        "reviewer",
                                        "endorser",
                                        "relatedArtifact",
                                        "library",
                                        "goal",
                                        "action",
                                    ],
                                ));
                            },
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
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
